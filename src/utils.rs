use macroquad::prelude::*;

pub fn triangle_distance_squared(v: Vec3, tri: [Vertex; 3]) -> f32
{
    return (v.distance_squared(tri[0].position) + v.distance_squared(tri[1].position) + v.distance_squared(tri[2].position)) / 3.0;
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
        cube_tool.set_filter(FilterMode::Nearest);

        let sv_tool = load_texture("assets/icons/sv_tool.png").await.unwrap();
        sv_tool.set_filter(FilterMode::Nearest);

        let svm_tool = load_texture("assets/icons/svm_tool.png").await.unwrap();
        svm_tool.set_filter(FilterMode::Nearest);

        return TextureAtlas
        {
            textures: vec![sv_tool, cube_tool, svm_tool],
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