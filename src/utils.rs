use macroquad::prelude::*;

pub fn triangle_distance_squared(loc: Vec3, poly: Vec<Vertex>) -> f32
{
    poly.iter().map(|v| loc.distance_squared(v.position)).sum::<f32>() / poly.len() as f32
}

pub fn drop_y_normalize(inp: Vec3) -> Vec3
{
    return vec3(inp.x, 0., inp.z).normalize();
}

pub struct TextureAtlas
{
    pub textures: Vec<Texture2D>,
}

impl TextureAtlas
{
    pub async fn icons_atlas() -> TextureAtlas
    {
        let cube_tool = load_texture("assets/icons/cube_tool.png").await.unwrap();

        let sv_tool = load_texture("assets/icons/sv_tool.png").await.unwrap();

        let svm_tool = load_texture("assets/icons/svm_tool.png").await.unwrap();

        let brush_tool = load_texture("assets/icons/brush_tool.png").await.unwrap();

        let inspect_tool = load_texture("assets/icons/inspect_tool.png").await.unwrap();

        let textures = [inspect_tool, cube_tool, sv_tool, svm_tool, brush_tool];
        
        for t in &textures
        {
            t.set_filter(FilterMode::Nearest);
        }

        return TextureAtlas
        {
            textures: textures.to_vec(),
        }
    }

    pub fn get_texture(&self, index: usize) -> Texture2D
    {
        if index < self.textures.len()
        {
            return self.textures[index].clone();
        }
        return self.textures[0].clone();
    }
}

