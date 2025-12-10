use macroquad::ui::{hash, root_ui, widgets::Window};
use macroquad::prelude::*;

use crate::FileSettings;
use crate::my_model::Model;

pub fn draw_paused(file_settings: &FileSettings, m: &mut Model)
{
    draw_rectangle(0.,0.,screen_width(),screen_height(), Color::from_rgba(0,0,0,127));

    Window::new(hash!(screen_width() as u64, screen_height() as u64, "pause menu"), Vec2::new(10., 10.), Vec2::new(300., screen_height() - 20.)).titlebar(false).ui(&mut *root_ui(), |ui|
    {
        ui.label(vec2(0.,0.), "Paused");
        
    });
}