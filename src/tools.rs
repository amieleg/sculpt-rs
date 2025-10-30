use macroquad::prelude::*;

use crate::my_model::*;
use crate::tri;

pub mod placer_tool;
pub mod single_vertex_tool;
pub mod hand_tool;

/// Trait to be implemented by any tool that is used to edit a model
/// @update - Takes a model and a location (usually where the player is looking) and does something to the model
///           or the tools internal state. Also handles input handling
/// @gen_mesh - Takes a model and generates a mesh from it, usually adding some parts from the ModelTools internal state
pub trait ModelTool
{
    fn start_up(&mut self, m: &mut Model, loc: Vec3);
    fn update(&mut self, m: &mut Model, loc: Vec3);
    fn gen_mesh(&self, m: &Model) -> Mesh; 
    fn get_name(&self) -> &str;
}

/// An addition to a model. The triangles may hold indices referencing a 'main' model.
#[derive(Clone)]
pub struct ModelAddition
{
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl ModelAddition
{
    pub fn new() -> ModelAddition
    {
        ModelAddition
        {
            vertices: vec![],
            triangles: vec![],
        }
    }
}