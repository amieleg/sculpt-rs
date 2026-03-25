//! Single-vertex placement tool.
//!
//! This tool allows placing a single vertex on an existing model surface and
//! temporarily shows the geometry that would be created by connecting that
//! vertex to the nearest triangle on the base model. Pressing the placement key
//! (B) toggles placing/committing the vertex.
//!
//! Intended to be used as a `ModelTool` implementation.

use macroquad::prelude::*;
use crate::aligners::*;
use crate::my_model::*;
use crate::tools::*;

/// Fields:
/// - `ma`: temporary ModelAddition (the vertex being placed and its triangles)
/// - `state`: current placing state
pub struct SingleVertexTool
{
    pub ma: ModelAddition, // model addition
    pub aligner: SimpleAligner,
}

impl ModelTool for SingleVertexTool
{
    fn start_up(&mut self, m: &mut Model, p: &Player) 
    {
        self.set_addition(m, self.aligner.align(m,p));
    }

    /// Handle input and update the tool state.
    ///
    /// - If the placement key (lmb) is pressed:
    ///   - If switching to `Placing`: create the new vertex
    ///   - If switching to `Nothing` -> merge the addition into the model and stop placing
    /// - While `Placing` update the temporary vertex position/triangles.
    fn update(&mut self, m: &mut Model, p: &Player)
    {
        if is_mouse_button_pressed(MouseButton::Left)
        {
            self.merge(m);
            self.set_addition(m, self.aligner.align(m,p));
        }

        self.update_addition(m, self.aligner.align(m,p));
    }

    /// Generate a mesh representing the combination of the base model and the
    /// tool's current addition. The returned mesh is used for preview rendering.
    fn draw_mesh(&self, m: &Model)
    {
        draw_mesh_wires(&self.ma.gen_mesh_full(&m, WHITE), BLACK);
    }

    fn get_name(&self) -> &str {
        "Single Vertex Tool"
    }

    fn get_texture_index(&self) -> usize 
    {
        return 2;    
    }
}

impl SingleVertexTool
{
    pub fn new() -> SingleVertexTool
    {
        SingleVertexTool
        {
            ma: ModelAddition::new(),
            aligner: SimpleAligner{},
        }
    }

    /// Start a placement by adding a vertex to the temporary addition and
    /// performing an initial update of triangles.
    fn set_addition(&mut self, m: &mut Model, loc: Vec3)
    {
        self.ma.vertices.push(loc);
        self.update_addition(m, loc);
    }

    /// Update the temporary vertex position and recompute the triangles that
    /// connect it to the nearest triangle on the model.
    /// set_addition needs to have been called first to add the vertex.
    fn update_addition(&mut self, m: &Model, loc: Vec3)
    {
        self.ma.vertices[0] = loc;
        self.set_addition_triangles(m, loc);
    }

    /// Merge the temporary addition into the provided model
    fn merge(&mut self, m: &mut Model)
    {
        m.vertices.append(&mut self.ma.vertices);
        m.polys.append(&mut self.ma.polys); 
    }

    /// Recompute the triangles for the temporary addition so the new vertex
    /// connects to the closest triangle on the base model.
    ///
    /// If a closest triangle exists, three new triangles are created that connct
    /// the new vertex to each edge of the closest triangle.
    fn set_addition_triangles(&mut self, m: &Model, loc: Vec3)
    {
        let closest_poly = m.get_closest_poly(loc);

        if let Some(poly) = closest_poly
        {
            let mut ma_polys = vec![];
            let poly_indexes = &poly.ixs;
            let base_uvs: Vec<Vec2> = poly.uvs;

            for i in 0..poly_indexes.len()
            {
                ma_polys.push( Poly{ixs: vec![poly_indexes[i], poly_indexes[(i+1) % poly_indexes.len()], m.vertices.len() as u16], uvs: vec![base_uvs[i], base_uvs[(i+1) % poly_indexes.len()], base_uvs[(i+2) % poly_indexes.len()]] , normal: Vec3::ZERO, material: poly.material});
            }

            self.ma.polys = ma_polys;
        }
    }
}