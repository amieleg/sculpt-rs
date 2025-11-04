use macroquad::prelude::*;
use crate::my_model::*;
use crate::tools::*;

pub struct HandTool
{

}

impl ModelTool for HandTool
{
    fn start_up(&mut self, m: &mut Model, p: &Player) 
    {
        
    }

    fn update(&mut self, m: &mut Model, p: &Player)
    {
        
    }

    fn gen_mesh(&self, m: &Model) -> Mesh
    {
        m.gen_mesh()
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