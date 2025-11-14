use macroquad::prelude::*;
use macroquad::ui::{widgets::*, hash, root_ui};
use crate::my_model::*;
use crate::tools::*;

pub struct InspectTool
{

}

impl ModelTool for InspectTool
{
    fn start_up(&mut self, _: &mut Model, _: &Player) 
    {
        
    }

    fn update(&mut self, m: &mut Model, p: &Player)
    {
        self.draw_info(m, p);
    }

    fn gen_mesh(&self, m: &Model) -> Mesh
    {
        m.gen_mesh()
    }

    fn get_name(&self) -> &str
    {
        "Inspect Tool"
    }

    fn get_texture_index(&self) -> usize 
    {
        return 0;
    }
}

impl InspectTool
{
    pub fn new() -> InspectTool
    {
        InspectTool
        {

        }
    }

    fn draw_info(&self, m: &Model, p: &Player)
    {
        let looking_at = m.send_ray(p.mi.position, p.mi.front);

        Window::new(hash!(screen_width() as u64, screen_height() as u64, "inspect_tool"), Vec2::new(screen_width() - 310., screen_height() - 210.), Vec2::new(300., 200.)).titlebar(false).ui(&mut *root_ui(), |ui|
        {
            if let Some((_, poly)) = looking_at
            {
                let looking_at_vertices = m.poly_as_vertices(&poly);
                
                Group::new(hash!(), Vec2::new(290., 40.)).ui(ui, |ui| {
                    ui.label(Vec2::new(0., 0.), "Looking at triangle:");
                });

                Group::new(hash!(), Vec2::new(290., 150.)).position(vec2(0., 50.)).ui(ui, |ui| {
                    for v in looking_at_vertices
                    {
                        Group::new(hash!(), Vec2::new(280., 50.)).ui(ui, |ui| {
                            ui.label(Vec2::new(0., 0.), &format!("Vertex {}:", v.position));
                        });
                    }
                });
            }
            else 
            {    
                ui.label(Vec2::new(10., 10.), "Looking at nothing!");
            }
            
        });
    }
}