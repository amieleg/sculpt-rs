use crate::{my_model::{Model, Poly}, player::Player, tools::{ModelAddition, ModelTool}};
use macroquad::prelude::*;
use macroquad::amiel::*;

pub struct SinglePolyTool
{
    vertices: Vec<Vec3>,
    maybe_indeces: Vec<Option<u16>>,
    potential_vertex: Option<Vec3>,
}

impl ModelTool for SinglePolyTool
{
    fn update(&mut self, m: &mut Model, p: &Player)
    {
        if let Some(looking_at_index) = m.send_ray_vertex(p.mi.position, p.mi.front, 0.1)
        {
            let looking_at_vertex = m.vertices[looking_at_index as usize];
            
            draw_sphere(looking_at_vertex,  0.05, None, RED);
            self.potential_vertex = Some(looking_at_vertex);
            
            if is_mouse_button_pressed(MouseButton::Left) && self.vertices.len() < 4
            {
                self.vertices.push(looking_at_vertex);
                self.maybe_indeces.push(Some(looking_at_index));
            }
        }
        else 
        {
            let new_vertex = p.mi.position + p.mi.front * 2.0;

            self.potential_vertex = Some(new_vertex);

            if is_mouse_button_pressed(MouseButton::Left) && self.vertices.len() < 4
            {
                self.vertices.push(new_vertex);
                self.maybe_indeces.push(None);
            }
        }

        if is_key_pressed(KeyCode::Enter) || (is_mouse_button_pressed(MouseButton::Left) && self.vertices.len() >= 4)
        {
            self.merge(m);
            self.vertices = vec![];
            self.maybe_indeces = vec![];
            self.potential_vertex = None;
        }
        if is_key_pressed(KeyCode::Backspace)
        {
            self.vertices = vec![];
            self.maybe_indeces = vec![];
            self.potential_vertex = None;
        }
        
        self.draw_potential_poly();
    }

    fn start_up(&mut self, m: &mut Model, p: &Player)
    {
        
    }

    fn get_name(&self) -> &str {
        "Single Poly Tool"
    }

    fn get_texture_index(&self) -> usize 
    {
        return 2;    
    }
}

impl SinglePolyTool
{
    pub fn new() -> SinglePolyTool
    {
        return SinglePolyTool 
        {
            vertices: vec![],
            maybe_indeces: vec![],
            potential_vertex: None,
        }
    }

    pub fn draw_potential_poly(&self)
    {
        if self.vertices.len() == 2
        {
            draw_line_3d(self.vertices[0], self.vertices[1], BLACK);
        }
        else if self.vertices.len() == 3
        {
            let vertices_list: Vec<Vertex> = self.vertices.iter().map(|v| Vertex {
                position: *v,
                uv: Vec2::ZERO,
                normal: Vec4::ZERO,
                color: [127, 127, 255, 127],
            }).collect();
            draw_tri_3d(vertices_list.try_into().unwrap());
        }
        else if self.vertices.len() == 4
        {
            let vertices_list: Vec<Vertex> = self.vertices.iter().map(|v| Vertex {
                position: *v,
                uv: Vec2::ZERO,
                normal: Vec4::ZERO,
                color: [127, 127, 255, 127],
            }).collect();
            draw_quad_3d(vertices_list.try_into().unwrap());
        }
    }

    pub fn merge(&self, m: &mut Model)
    {
        let mut new_poly_indeces = vec![];
        let mut new_vertices = vec![];
        let mut new_vertex_index = m.vertices.len() as u16;


        for i in 0..self.maybe_indeces.len()
        {
            if let Some(index) = self.maybe_indeces[i]
            {
                new_poly_indeces.push(index);
            }
            else
            {
                new_vertices.push(self.vertices[i]);
                new_poly_indeces.push(new_vertex_index);
                new_vertex_index += 1;
            }
        }

        m.vertices.append(&mut new_vertices);
        
        if self.vertices.len() == 4
        {
            m.polys.push(Poly::Quad { ix: new_poly_indeces.try_into().unwrap(), uvs: [vec2(0.,0.); 4], normal: Vec3::ZERO });
        }
        else if self.vertices.len() == 3
        {
            m.polys.push(Poly::Triangle { ix: new_poly_indeces.try_into().unwrap(), uvs: [vec2(0.,0.); 3], normal: Vec3::ZERO });
        }
    }
}