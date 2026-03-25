use std::collections::HashMap;
use std::iter::Map;

use macroquad::prelude::*;
use macroquad::ui::widgets::Window;
use macroquad::ui::{Ui, widgets::Group};
use macroquad::ui::{hash, root_ui};
use crate::material_atlas::MaterialAtlas;
use crate::tools::ModelTool;
use crate::my_model::*;
use crate::player::*;


pub struct PolyBrushTool
{
    material_atlas: MaterialAtlas,
    selected_id: u16,
    palette: Texture2D
}

impl ModelTool for PolyBrushTool
{
    fn update(&mut self, m: &mut Model, p: &Player)
    {
        if is_mouse_button_pressed(MouseButton::Left)
        {
            if let Some((_,target_poly,poly_index)) = m.send_ray(p.mi.position, p.mi.front)
            {
                m.polys[poly_index as usize].material = Some(self.selected_id);
                m.set_uv(poly_index, &self.material_atlas);
            }
        }
        if is_key_pressed(KeyCode::C)
        {
            self.selected_id = (self.selected_id + 1) % self.material_atlas.palette_size.pow(2);
        }

        self.draw_info();
    }

    fn draw_settings(&mut self, ui: &mut Ui)
    {

    }

    fn start_up(&mut self, m: &mut Model, p: &Player)
    {

    }

    fn get_name(&self) -> &str
    {
        "Material Brush Tool"
    }

    fn get_texture_index(&self) -> usize
    {
        4
    }
}

impl PolyBrushTool
{
    pub async fn new(atlas: MaterialAtlas) -> PolyBrushTool
    {
        PolyBrushTool 
        {
            palette: atlas.compile(),
            material_atlas: atlas,
            selected_id: 0,
        }
    }

    pub fn merge(&self, m: &mut Model)
    {

    }

    pub fn draw_info(&self)
    {
        Window::new(hash!(screen_width() as u64, screen_height() as u64, "material brush tool"), Vec2::new(10., screen_height() - 210.), Vec2::new(300., 200.)).titlebar(false).ui(&mut *root_ui(), |ui|
        {
            Group::new(hash!("title"), vec2(294.,30.)).position(vec2(2.,2.)).ui(ui, |ui| 
            {
                ui.label(vec2(0.,0.), "Material Brush");
            });

            let mut image_canvas = ui.canvas();
            let image_cursor = image_canvas.cursor();

            let texture = &self.palette;
            texture.set_filter(FilterMode::Nearest);

            image_canvas.image(Rect::new(image_cursor.x, image_cursor.y + 34., 170., 170.), texture);
        });
    }
}