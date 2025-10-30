//! Single-vertex placement tool.
//!
//! This tool allows placing a single vertex on an existing model surface and
//! temporarily shows the geometry that would be created by connecting that
//! vertex to the nearest triangle on the base model. Pressing the placement key
//! (B) toggles placing/committing the vertex.
//!
//! Intended to be used as a `ModelTool` implementation.

use macroquad::prelude::*;
use crate::my_model::*;
use crate::tools::*;
use crate::placer_tool::PlacingState;
use crate::tri;

/// Fields:
/// - `ma`: temporary ModelAddition (the vertex being placed and its triangles)
/// - `state`: current placing state
pub struct SingleVertexTool
{
    pub ma: ModelAddition, // model addition
    pub state: PlacingState,
}

impl ModelTool for SingleVertexTool
{
    fn start_up(&mut self, m: &mut Model, loc: Vec3) 
    {
        self.set_addition(m, loc);
    }

    /// Handle input and update the tool state.
    ///
    /// - If the placement key (lmb) is pressed:
    ///   - If switching to `Placing`: create the new vertex
    ///   - If switching to `Nothing` -> merge the addition into the model and stop placing
    /// - While `Placing` update the temporary vertex position/triangles.
    fn update(&mut self, m: &mut Model, loc: Vec3)
    {
        if is_mouse_button_pressed(MouseButton::Left)
        {
            self.merge(m);
            self.set_addition(m, loc);
        }

        self.update_addition(m, loc);
    }

    /// Generate a mesh representing the combination of the base model and the
    /// tool's current addition. The returned mesh is used for preview rendering.
    fn gen_mesh(&self, m: &Model) -> Mesh
    {
        let mut vertices = m.vertices.clone();
        vertices.append(&mut self.ma.vertices.clone());

        let mut indices = m.triangles.clone();
        indices.append(&mut self.ma.triangles.clone());
        let indices_flattened: Vec<u16> = indices.iter().flat_map(|tri| tri.ix.to_vec()).collect();

        Mesh
        {
            vertices: vertices,
            indices: indices_flattened,
            texture: m.texture.clone(),
        }
    }

    fn get_name(&self) -> &str {
        "Single Vertex Tool"
    }
}

impl SingleVertexTool
{
    pub fn new() -> SingleVertexTool
    {
        SingleVertexTool
        {
            ma: ModelAddition::new(),
            state: PlacingState::Nothing
        }
    }

    /// Start a placement by adding a vertex to the temporary addition and
    /// performing an initial update of triangles.
    fn set_addition(&mut self, m: &mut Model, loc: Vec3)
    {
        self.ma.vertices.push(Vertex::new2(loc, vec2(0.0, 0.0), WHITE));
        self.update_addition(m, loc);
    }

    /// Update the temporary vertex position and recompute the triangles that
    /// connect it to the nearest triangle on the model.
    /// set_addition needs to have been called first to add the vertex.
    fn update_addition(&mut self, m: &Model, loc: Vec3)
    {
        self.ma.vertices[0] = Vertex::new2(loc, vec2(0.0, 0.0), WHITE);
        self.set_addition_triangles(m, loc);
    }

    /// Merge the temporary addition into the provided model
    fn merge(&mut self, m: &mut Model)
    {
        m.vertices.append(&mut self.ma.vertices);
        m.triangles.append(&mut self.ma.triangles); 
    }

    /// Recompute the triangles for the temporary addition so the new vertex
    /// connects to the closest triangle on the base model.
    ///
    /// If a closest triangle exists, three new triangles are created that connct
    /// the new vertex to each edge of the closest triangle.
    fn set_addition_triangles(&mut self, m: &Model, loc: Vec3)
    {
        let closest_tri = m.get_closest_triangle(loc);

        if let Some(tri) = closest_tri
        {
            self.ma.triangles = vec![  tri![tri.ix[0], tri.ix[1], m.vertices.len() as u16],
                                        tri![tri.ix[1], tri.ix[2], m.vertices.len() as u16],
                                        tri![tri.ix[0], tri.ix[2], m.vertices.len() as u16]];
        }
    }
}