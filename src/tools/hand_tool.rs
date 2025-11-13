use macroquad::prelude::*;
use crate::my_model::*;
use crate::tools::*;

pub struct HandTool
{

}

impl ModelTool for HandTool
{
    fn start_up(&mut self, _m: &mut Model, _p: &Player) 
    {
        
    }

    fn update(&mut self, _m: &mut Model, _p: &Player)
    {
        
    }

    fn gen_mesh(&self, _m: &Model) -> Mesh
    {
        return Mesh
        {
            vertices: vec![],
            indices: vec![],
            texture: None,
        }
    }

    fn get_name(&self) -> &str
    {
        "Hand Tool"
    }

    fn get_texture_index(&self) -> usize 
    {
        return 0
    }
}

impl HandTool
{
    pub fn new() -> HandTool
    {
        HandTool
        {

        }
    }
}