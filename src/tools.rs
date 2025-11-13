use macroquad::prelude::*;

use crate::my_model::*;
use crate::player::Player;

pub mod placer_tool;
pub mod single_vertex_tool;
pub mod hand_tool;
pub mod single_vertex_mover;
pub mod brush_tool;
pub mod inspect_tool;

/// Trait to be implemented by any tool that is used to edit a model
/// @start_up - called when the player starts using the tool, sets the tool's internal state, not always necessary 
///         if both can be done in update
/// 
/// @update - Takes a model and a location (usually where the player is looking) and does something to the model
///           or the tools internal state. Also handles input handling
/// @gen_mesh - Takes a model and generates a mesh from it, usually adding some parts from the ModelTools internal state
pub trait ModelTool
{
    fn start_up(&mut self, m: &mut Model, p: &Player);
    fn update(&mut self, m: &mut Model, p: &Player);
    fn gen_mesh(&self, m: &Model) -> Mesh; 
    fn get_name(&self) -> &str;
    fn get_texture_index(&self) -> usize;
}

/// An addition to a model. The triangles may hold indices referencing a 'main' model.
#[derive(Clone)]
pub struct ModelAddition
{
    pub vertices: Vec<Vec3>,
    pub polys: Vec<Poly>,
}

impl ModelAddition
{
    pub fn new() -> ModelAddition
    {
        ModelAddition
        {
            vertices: vec![],
            polys: vec![],
        }
    }

    // Generates a mesh with both a given model and the modeladdition
    pub fn gen_mesh(&self, m: &Model) -> Mesh
    {
        let mut positions = m.vertices.clone();
        positions.append(&mut self.vertices.clone());

        let mut base = m.gen_mesh();

        for poly in &self.polys
        {
            match poly
            {
                Poly::Triangle{ ix, uvs, ..} => 
                    base.vertices.append(&mut [
                        Vertex::new2(positions[ix[0] as usize], uvs[0],WHITE), 
                        Vertex::new2(positions[ix[1] as usize], uvs[1], WHITE), 
                        Vertex::new2(positions[ix[2] as usize], uvs[2], WHITE),
                    ].to_vec()),
                Poly::Quad{ ix, uvs, ..} => 
                    base.vertices.append(&mut [
                        Vertex::new2(positions[ix[0] as usize], uvs[0],WHITE), 
                        Vertex::new2(positions[ix[1] as usize], uvs[1], WHITE), 
                        Vertex::new2(positions[ix[2] as usize], uvs[2], WHITE),
                        Vertex::new2(positions[ix[2] as usize], uvs[2],WHITE), 
                        Vertex::new2(positions[ix[3] as usize], uvs[3], WHITE), 
                        Vertex::new2(positions[ix[0] as usize], uvs[0], WHITE),
                    ].to_vec()),
            }
        }

        base.indices = (0..(base.vertices.len() as u16)).collect();

        return base;
    }

}