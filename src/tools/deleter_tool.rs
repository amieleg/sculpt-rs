use macroquad::prelude::*;
use crate::my_model::*;
use crate::tools::*;

#[derive(PartialEq)]
pub enum DTState
{
    Vertices,
    Polys,
}

pub struct DeleterTool
{
    state: DTState
}

impl ModelTool for DeleterTool
{  
    fn update(&mut self, m: &mut Model, p: &Player)
    {
        if self.state == DTState::Vertices
        {
            if let Some(vertex_index) = m.send_ray_vertex(p.mi.position, p.mi.front, 0.05)
            {
                draw_sphere(m.vertices[vertex_index as usize], 0.05, None, RED);
                if is_mouse_button_pressed(MouseButton::Left)
                {
                    m.delete_vertex(vertex_index);
                }
            }

            if is_key_pressed(KeyCode::T)
            {
                self.state = DTState::Polys;
            }
        }
        else
        {
            if is_mouse_button_pressed(MouseButton::Left)
            {
                if let Some((_, _, poly_index)) = m.send_ray(p.mi.position, p.mi.front)
                {
                    m.delete_poly(poly_index)
                }
            }
            if is_key_pressed(KeyCode::T)
            {
                self.state = DTState::Vertices;
            }
        }
    }

    fn get_name(&self) -> &str
    {
        "Deleter Tool"
    }

    fn get_texture_index(&self) -> usize 
    {
        return 5
    }
}

impl DeleterTool
{
    pub fn new() -> DeleterTool
    {
        DeleterTool
        {
            state: DTState::Polys,
        }
    }
}