use std::{collections::HashMap, fs::read_dir, path::{Path, PathBuf}};

use macroquad::{color::Color, math::{Rect, Vec2, vec2}, texture::{Image, Texture2D, load_image, load_texture}};

pub struct MaterialAtlas
{
    pub id_to_string: HashMap<u16, String>,
    pub textures: HashMap<String, Image>,
    pub uvs: HashMap<String, (Vec2,Vec2)>,
    pub palette_size: u16,
    pub material_size: u16,
}

impl MaterialAtlas
{
    pub async fn new(names: Vec<String>, imgs: Vec<PathBuf>, material_size: u16, palette_size: u16) -> MaterialAtlas
    {
        let mut textures = HashMap::new();
        let mut id_to_string = HashMap::new();
        for i in 0..names.len()
        {
            let material_image = load_image(imgs[i].to_str().unwrap()).await;

            if material_image.is_ok()
            {
                textures.insert(names[i].clone(), material_image.unwrap());
                id_to_string.insert(i as u16,names[i].clone());
            }
        }

        let mut material_atlas = 
            MaterialAtlas
            {
                id_to_string: id_to_string,
                textures: textures,
                uvs: HashMap::new(),
                palette_size: palette_size,
                material_size: material_size
            };

        material_atlas.set_uv();
        
        return material_atlas;
    }

    pub async fn empty() -> MaterialAtlas
    {
        return MaterialAtlas::new(vec![], vec![], 256, 2).await;
    }

    pub async fn from_folder(dir: &Path) -> Result<MaterialAtlas, String>
    {
        if dir.is_dir() && let Ok(children) = read_dir(dir)
        {
            let mut mat_paths = vec![];
            let mut mat_names = vec![];
            for entry in children
            {
                if let Ok(entry) = entry
                {
                    let pathbuf = entry.path();
                    println!("Loading material image {}", pathbuf.to_str().unwrap());

                    if pathbuf.as_path().extension().unwrap() == "png"
                    {
                        mat_paths.push(pathbuf.clone());
                        mat_names.push(String::from(pathbuf.file_name().unwrap().to_str().unwrap()));
                    }
                }
            }
            
            return Ok(Self::new(mat_names, mat_paths, 256,4).await);
        }
        return Err(String::from("Couldnt build from folder"));
    }

    pub fn set_uv(&mut self) 
    {
        let uv_unit = 1. / self.palette_size as f32;
        let mut i = 0;
        for material_name in self.textures.keys()
        {
            let u1 = (i % self.palette_size) as f32 / self.palette_size as f32;
            let v1 = (i / self.palette_size) as f32 / self.palette_size as f32;
            let u2 = u1 + uv_unit;
            let v2 = v1 + uv_unit;
            i += 1;

            self.uvs.insert(material_name.to_string(), (vec2(u1,v1),vec2(u2,v2)));
        }
    }

    pub fn get_uvs(&self, id: u16) -> (Vec2, Vec2)
    {
        if let Some(name) = self.id_to_string.get(&id)
        {
            return *self.uvs.get(name).unwrap();
        }
        else 
        {
            return (Vec2::ZERO, Vec2::ZERO);
        }
    }

    pub fn compile(&self) -> Texture2D
    {
        let quality = self.material_size * self.palette_size;

        let mut compiled_atlas = Image::gen_image_color(quality, quality, Color::from_rgba(249,26,86,255));

        let mut i = 0;

        for material_image in self.textures.values()
        {            
            image_on_image(&material_image, &mut compiled_atlas, (i % self.palette_size) * self.material_size, (i / self.palette_size) * self.material_size, quality / self.palette_size, quality / self.palette_size);
            i += 1;
        }

        return Texture2D::from_image(&compiled_atlas);
    }
}

pub fn image_on_image(from: &Image, onto: &mut Image, x: u16, y: u16, width: u16, height: u16)
{
    for from_y in 0..width
    {
        for from_x in 0..height
        {
            let target_y = y + from_y;
            let target_x = x + from_x;

            if target_y < (onto.height() as u16) && target_x < (onto.width() as u16)
            {
                onto.set_pixel(target_x as u32, target_y as u32, from.get_pixel(from_x as u32, from_y as u32));
            }
        }
    }
}

