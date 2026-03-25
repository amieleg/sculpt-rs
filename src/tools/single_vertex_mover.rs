use crate::aligners::Aligner;
use crate::utils::vec_as_change;
use crate::{my_model::Model, tools::ModelTool, player::*, aligners::GridAligner};
use macroquad::amiel::draw_mesh_wires;
use macroquad::prelude::*;
use macroquad::models::Mesh;
use macroquad::ui::{hash, root_ui};
use macroquad::ui::widgets::Window;


#[derive(PartialEq)]
pub enum SVMState
{
    Selecting,
    Moving
}

pub struct SingleVertexMover
{
    selected: u16,
    vertex_start_pos: Vec3,
    mouse_start_pos: Vec3,
    arrows_pos: Vec3,
    state: SVMState,
    aligner: GridAligner,
    mouse_control: bool,
    arrow_control: bool,
    grain_size: f32,
}

impl ModelTool for SingleVertexMover
{
    fn update(&mut self, m: &mut Model, p: &Player)
    {
        if self.state == SVMState::Selecting
        {
            let looking_at = m.send_ray_vertex(p.mi.position, p.mi.front, 0.1);

            if let Some(index_vec) = looking_at
            {
                let looking_at_vertex = m.vertices[index_vec as usize];

                draw_sphere(looking_at_vertex, 0.05, None, RED);

                if is_mouse_button_pressed(MouseButton::Left)
                {
                    self.selected = index_vec;
                    self.vertex_start_pos = looking_at_vertex;
                    self.mouse_start_pos = self.aligner.align(m, p);
                    self.arrows_pos = Vec3::ZERO;
                    self.state = SVMState::Moving;
                }
            }

        }
        else if self.state == SVMState::Moving
        {
            let new_aligned_pos = self.aligner.align(m, p);
            let mut mouse_component = Vec3::ZERO;

            if self.mouse_control
            {
                mouse_component = new_aligned_pos - self.mouse_start_pos;
            }

            if self.arrow_control
            {
                let dir = p.front_as_direction() * self.grain_size;
                let rightdir = vec3(-dir.z,dir.y,dir.x);
                
                if is_key_pressed(KeyCode::Left)
                {
                    self.arrows_pos -= rightdir;
                }
                if is_key_pressed(KeyCode::Right)
                {
                    self.arrows_pos += rightdir;
                }
                if is_key_pressed(KeyCode::Up)
                {
                    self.arrows_pos += dir;
                }
                if is_key_pressed(KeyCode::Down)
                {
                    self.arrows_pos -=  dir;
                }
                if is_key_pressed(KeyCode::Minus)
                {
                    self.arrows_pos.y -= self.grain_size;
                }
                if is_key_pressed(KeyCode::Equal)
                {
                    self.arrows_pos.y += self.grain_size;
                }
            }

            m.vertices[self.selected as usize] = ((self.vertex_start_pos + mouse_component + self.arrows_pos) * (1. / self.grain_size)).round() * self.grain_size;

            draw_sphere(m.vertices[self.selected as usize], 0.05, None, RED);

            if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Enter)
            {
                self.state = SVMState::Selecting;
            }
        }
        self.draw_info(m, p);
    }

    fn shut_down(&mut self, _m: &mut Model, _p: &Player)
    {
        self.state = SVMState::Selecting;
    }

    fn draw_mesh(&self, m: &Model)
    {
        draw_mesh_wires(&m.gen_mesh(), BLACK);
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
            vertex_start_pos: Vec3::ZERO,
            mouse_start_pos: Vec3::ZERO,
            arrows_pos: Vec3::ZERO,
            state: SVMState::Selecting,
            aligner: GridAligner::new(0.1),
            mouse_control: true,
            arrow_control: true,
            grain_size: 0.1,
        }
    }

    pub fn draw_info(&self, m: &Model, _p: &Player)
    {
        Window::new(hash!(screen_width() as u64, screen_height() as u64, "svm tool"), Vec2::new(10., screen_height() - 60.), Vec2::new(300., 50.)).titlebar(false).ui(&mut *root_ui(), |ui|
        {
            if self.state == SVMState::Selecting
            {
                ui.label(Vec2::new(0.,0.), "Select a vertex and use mouse / arrow keys");
                ui.label(Vec2::new(0.,10.), "to move.");
            }
            else 
            {
                ui.label(Vec2::new(0.,0.), &format!("Vertex started at {}.", self.vertex_start_pos));
                let change = m.vertices[self.selected as usize] - self.vertex_start_pos;
                ui.label(Vec2::new(0.,10.), &(String::from("Moved by ") + &vec_as_change(change)));
                ui.label(Vec2::new(0.,20.), &format!("Now at {}.", m.vertices[self.selected as usize]));
            }
        });
    }
}