use macroquad::prelude::*;
use macroquad::ui::widgets::Window;
use macroquad::ui::{Ui, widgets::Group};
use macroquad::ui::{hash, root_ui};
use crate::tools::ModelTool;
use crate::my_model::*;
use crate::player::*;


pub struct BrushTool
{
    color: Color,
    image: Image,
}

impl ModelTool for BrushTool
{
    fn update(&mut self, m: &mut Model, p: &Player)
    {
        if let Some((loc, poly, _)) = m.send_ray(p.mi.position, p.mi.front)
        {
            if is_mouse_button_down(MouseButton::Left)
            {
                if let Some(uv) = m.calc_uv(poly, loc)
                {
                    let pixel_x = (uv.x * self.image.width as f32) as u32;
                    let pixel_y = (uv.y * self.image.height as f32) as u32;

                    self.image.set_pixel(pixel_x, pixel_y, self.color);
                }
            }
        }
        self.merge(m);
        self.draw_info();
    }

    fn draw_settings(&mut self, ui: &mut Ui)
    {
        ui.label(vec2(0.,0.), "Brush tool");

        Group::new(hash!(), vec2(394., 280.)).position(vec2(0.,20.)).ui(ui, |ui|
        {
            ui.slider(hash!(), "Red", 0.0..1.0, &mut self.color.r);
            ui.slider(hash!(), "Green", 0.0..1.0, &mut self.color.g);
            ui.slider(hash!(), "Blue", 0.0..1.0, &mut self.color.b);

            Group::new(hash!(), vec2(26., 26.)).position(vec2(350.,20.)).ui(ui, |ui|
            {
                let mut canvas = ui.canvas();
                let cursor = canvas.cursor();

                canvas.rect(Rect::new(cursor.x + 0.,cursor.y + 0.,20.,20.), self.color, self.color);
            });
        });
    }

    fn get_name(&self) -> &str
    {
        "Brush Tool"
    }

    fn get_texture_index(&self) -> usize
    {
        4
    }
}

impl BrushTool
{
    pub async fn new(img: &str) -> BrushTool
    {
        BrushTool
        {
            color: BLACK,
            image: load_image(img).await.unwrap(),
        }
    }

    pub fn merge(&self, m: &mut Model)
    {
        m.texture.as_mut().unwrap().update(&self.image);
    }

    pub fn draw_info(&self)
    {
        Window::new(hash!(screen_width() as u64, screen_height() as u64, "brush tool"), Vec2::new(10., screen_height() - 210.), Vec2::new(300., 200.)).titlebar(false).ui(&mut *root_ui(), |ui|
        {
            ui.label(vec2(0.,0.), "Brush Tool Color: ");

            let mut color_square = ui.canvas();
            let color_cursor = color_square.cursor();

            color_square.rect(Rect::new(color_cursor.x + 120.,color_cursor.y + 0.,20.,20.), self.color, self.color);

            let mut image_canvas = ui.canvas();
            let image_cursor = image_canvas.cursor();

            let texture = &Texture2D::from_image(&self.image);
            texture.set_filter(FilterMode::Nearest);

            image_canvas.image(Rect::new(image_cursor.x, color_cursor.y+25., 290., 170.), &texture);
        });
    }
}