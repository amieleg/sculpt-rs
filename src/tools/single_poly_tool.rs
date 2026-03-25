use crate::{my_model::{Model, Poly}, player::Player, tools::{ModelAddition, ModelTool}};
use macroquad::prelude::*;
use macroquad::amiel::*;

pub struct SinglePolyTool
{
    ma: ModelAddition,
    pot_vertex: Vec3,
}

static POT_BLUE: Color = Color::from_rgba(127,127,255,127);
static POT_RED: Color = Color::from_rgba(255,127,127,127);

impl ModelTool for SinglePolyTool
{
    fn update(&mut self, m: &mut Model, p: &Player)
    {
        if let Some(looking_at_index) = m.send_ray_vertex(p.mi.position, p.mi.front, 0.1)
        {
            let looking_at_vertex = m.vertices[looking_at_index as usize];
            
            draw_sphere(looking_at_vertex,  0.05, None, RED);
            self.pot_vertex = looking_at_vertex;
            
            if is_mouse_button_pressed(MouseButton::Left)
            {
                self.ma.polys[0].ixs.push(looking_at_index);
                self.ma.polys[0].uvs.push(Vec2::ZERO);
            }
        }
        else 
        {
            let new_vertex = p.mi.position + p.mi.front * 2.0;
            
            self.pot_vertex = new_vertex;
            

            if is_mouse_button_pressed(MouseButton::Left)
            {
                self.ma.vertices.push(new_vertex);
                self.ma.polys[0].ixs.push(m.vertices.len() as u16 + self.ma.vertices.len() as u16 - 1);
                self.ma.polys[0].uvs.push(Vec2::ZERO);
            }
        }

        if is_key_pressed(KeyCode::Enter)
        {
            self.merge(m);
            self.start_up(m,p);
        }
        if is_key_pressed(KeyCode::Backspace)
        {
            self.ma.vertices.clear();
            self.ma.polys.clear();
        }
        
        self.draw_potential_poly(m);       
    }

    fn draw_mesh(&self, m: &Model)
    {
        draw_mesh(&self.ma.gen_mesh(&m, POT_BLUE));
    }

    fn start_up(&mut self, m: &mut Model, p: &Player)
    {
        self.ma.polys = vec![Poly{ixs:vec![],uvs:vec![],normal:Vec3::ZERO,material:None}];
        self.ma.vertices = vec![];
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
            ma: ModelAddition::new(),
            pot_vertex: Vec3::ZERO,
        }
    }

    pub fn draw_potential_poly(&self, m: &Model)
    {
        if self.ma.polys[0].ixs.len() == 1
        {
            draw_line_3d(self.ma.get_pos(m, self.ma.polys[0].ixs[0]), self.pot_vertex, POT_RED);
        }
        else if self.ma.polys[0].ixs.len() >= 2
        {
            let len = self.ma.polys[0].ixs.len();
            draw_tri_3d([
                Vertex::new2(self.ma.get_pos(m, self.ma.polys[0].ixs[0]), Vec2::ZERO, POT_BLUE),
                Vertex::new2(self.ma.get_pos(m, self.ma.polys[0].ixs[len-1]), Vec2::ZERO, POT_BLUE),
                Vertex::new2(self.pot_vertex, Vec2::ZERO, POT_BLUE)
            ])   
        }
    }

    pub fn merge(&mut self, m: &mut Model)
    {
        self.ma.append_to(m);
    }
}