use macroquad::prelude::*;
use crate::tools::ModelTool;
use crate::my_model::*;
use crate::player::*;


pub struct BrushTool
{
    color: Color,
    image: Image,
}

impl ModelTool for BrushTool
{
    fn start_up(&mut self, _m: &mut Model, _p: &Player)
    {
        
    }

    fn update(&mut self, m: &mut Model, p: &Player)
    {
        if let Some((loc, poly)) = m.send_ray(p.mi.position, p.mi.front)
        {
            if is_mouse_button_down(MouseButton::Left)
            {
                if let Some(uv) = m.calc_uv(poly, loc)
                {
                    let pixel_x = (uv.x * self.image.width as f32) as u32;
                    let pixel_y = (uv.y * self.image.height as f32) as u32;

                    self.image.set_pixel(pixel_x, pixel_y, self.color);
                }
            }
        }
        self.merge(m);
    }

    fn gen_mesh(&self, m: &Model) -> Mesh
    {
        m.gen_mesh()
    } 

    fn get_name(&self) -> &str
    {
        "Brush Tool"
    }

    fn get_texture_index(&self) -> usize
    {
        4
    }
}

impl BrushTool
{
    pub async fn new(img: &str) -> BrushTool
    {
        BrushTool
        {
            color: BLACK,
            image: load_image(img).await.unwrap(),
        }
    }

    pub fn merge(&self, m: &mut Model)
    {
        m.texture.as_mut().unwrap().update(&self.image);
    }
}