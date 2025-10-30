use macroquad::prelude::*;
use crate::my_model::*;
use crate::tools::*;
use crate::tri;


#[derive(PartialEq)]
pub enum PlacingState
{
    Nothing,
    Placing
}

pub struct PlacerTool
{
    pub name: String,
    pub ma: ModelAddition, // ma = ModelAddition
    pub state: PlacingState,
    pub placing: ModelAddition, // The ModelAddition that the PlacerTool will be able to add to the main model, read-only
}

/// A tool allowing the user to place a certain ModelAddition in the main model
impl ModelTool for PlacerTool
{
    fn start_up(&mut self, m: &mut Model, loc: Vec3) 
    {
        self.set_addition(loc);
    }

    /// Handles input for the PlacerTool
    /// When B is first pressed, the user will be able to move around a ModelAddition
    /// When B is pressed again, the ModelAddition is merged into the main model
    fn update(&mut self, m: &mut Model, loc: Vec3)
    {
        if is_mouse_button_pressed(MouseButton::Left)
        {
            self.merge(m);
        }

        self.set_addition(loc);
    }

    /// Generates a mesh based on the ModelAddition of the PlacerTool and a given model
    /// Before being added the final mesh, the triangles of the ma are transformed such that the triangles can be appended onto the main model and still reference the right vertices
    fn gen_mesh(&self, m: &Model) -> Mesh
    {
        let mut vertices = m.vertices.clone();
        let base_vertices = vertices.len() as u16;
        vertices.append(&mut self.ma.vertices.clone());

        let mut indices: Vec<u16> = m.triangles.clone().iter()
                                                        .flat_map(|tri| tri.ix.to_vec())
                                                        .collect();
        let mut transformed_addition_indices: Vec<u16> = self.ma.triangles.clone().iter()
                                                                                    .flat_map(|tri| [tri.ix[0] + base_vertices, tri.ix[1] + base_vertices, tri.ix[2] + base_vertices])
                                                                                    .collect();

        indices.append(&mut transformed_addition_indices);

        Mesh
        {
            vertices: vertices,
            indices: indices,
            texture: m.texture.clone(),
        }
    }

    fn get_name(&self) -> &str {
        return &self.name;
    }
}

impl PlacerTool
{
    pub fn new(name: String, addition: ModelAddition) -> PlacerTool
    {
        PlacerTool
        {
            name: name,
            ma: ModelAddition::new(),
            state: PlacingState::Nothing,
            placing: addition
        }
    }

    /// Sets ma according to the given location
    pub fn set_addition(&mut self, loc: Vec3)
    {
        self.ma = self.placing.clone();
        self.ma.vertices = self.ma.vertices.iter().map(|v| Vertex::new2(v.position + loc, v.uv, v.color.into())).collect();
    }

    /// Merges the ma into a given model, non-reversible
    fn merge(&mut self, m: &mut Model)
    {
        let amount_vertices = m.vertices.len() as u16;
        m.vertices.append(&mut self.ma.vertices);
        m.triangles.append(&mut self.ma.triangles.iter().map(|tri| tri![tri.ix[0] + amount_vertices, tri.ix[1] + amount_vertices, tri.ix[2] + amount_vertices]).collect()); 
    }
}