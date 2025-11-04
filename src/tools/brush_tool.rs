use macroquad::prelude::*;
use macroquad::models::*;
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
    fn start_up(&mut self, m: &mut Model, p: &Player)
    {
        
    }

    fn update(&mut self, m: &mut Model, p: &Player)
    {
        if is_mouse_button_down(MouseButton::Left)
        {
            self.image.bytes[45 * 4] = 0; 
            self.image.bytes[45 * 4 + 1] = 0; 
            self.image.bytes[45 * 4 + 2] = 0; 
            self.image.bytes[45 * 4 + 3] = 255;; 
        
            self.merge(m);
        }
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
        0
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