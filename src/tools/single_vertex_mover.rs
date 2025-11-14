use crate::aligners::Aligner;
use crate::{my_model::Model, tools::ModelTool, player::*, aligners::GridAligner};
use macroquad::prelude::*;
use macroquad::models::Mesh;


#[derive(PartialEq)]
pub enum SVMState
{
    Selecting,
    Moving
}

pub struct SingleVertexMover
{
    selected: u16,
    state: SVMState,
    aligner: GridAligner,
}

impl ModelTool for SingleVertexMover
{
    fn start_up(&mut self, _m: &mut Model, _p: &Player)
    {

    }
    fn update(&mut self, m: &mut Model, p: &Player)
    {
        if self.state == SVMState::Selecting
        {
            let looking_at = m.send_ray_vertex(p.mi.position, p.mi.front, 0.1);

            if let Some(index_vec) = looking_at
            {
                let looking_at_pos = m.vertices[index_vec as usize];

                draw_sphere(looking_at_pos, 0.05, None, RED); // change this to use gen_mesh

                if is_mouse_button_pressed(MouseButton::Left)
                {
                    self.selected = index_vec;
                    self.state = SVMState::Moving;
                }
            }

        }
        else if self.state == SVMState::Moving
        {
            let aligned_pos = self.aligner.align(m, p);

            m.vertices[self.selected as usize] = aligned_pos;

            if is_mouse_button_released(MouseButton::Left)
            {
                self.state = SVMState::Selecting;
            }
        }
    }

    fn gen_mesh(&self, m: &Model) -> Mesh
    {
        m.gen_mesh()
    }

    fn get_name(&self) -> &str
    {
        "Single Vertex Mover Tool"
    }

    fn get_texture_index(&self) -> usize
    {
        3
    }
}

impl SingleVertexMover
{
    pub fn new() -> SingleVertexMover
    {
        SingleVertexMover
        {
            selected: 0,
            state: SVMState::Selecting,
            aligner: GridAligner::new(0.1),
        }
    }
}