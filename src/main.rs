use macroquad::prelude::*;
use macroquad::amiel::*;
use macroquad::ui::Skin;
use macroquad::ui::StyleBuilder;
use macroquad::ui::{self, hash, widgets, root_ui};

mod my_model;
mod utils;
mod player;
mod tools;
mod toolbar;

use crate::my_model::*;
use crate::player::*;
use crate::tools::*;
use crate::toolbar::Toolbar;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

pub static PIXEL_SIZE: u16 = 1;

#[macroquad::main(conf)]
async fn main() {
    let texture = load_texture("assets/birdcolors.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    let mut f3 = false;

    let mut m = Model::new(texture);

    let mut p = Player::new();

    let mut tb = Toolbar::new();// toolbar

    set_cursor_grab(p.mi.grabbed);
    show_mouse(false);

    loop {

        // --- UPDATE / INPUT ---

        let delta = get_frame_time();
        p.update(delta);
        { // Lifetime mut mi reference
        let mi = &mut p.mi;

        if is_key_pressed(KeyCode::Escape)
        {
            break;
        }
        if is_key_pressed(KeyCode::Tab) 
        {
            mi.grabbed = !mi.grabbed;
            set_cursor_grab(mi.grabbed);
            show_mouse(!mi.grabbed);
        }
        if is_key_pressed(KeyCode::F3)
        {
            f3 = !f3;
        }
        if tb.update()
        {
            let tool = tb.get_current_tool_mut();
            tool.start_up(&mut m, mi.position + mi.front * 2.0);
        }

        let tool = tb.get_current_tool_mut();
        tool.update(&mut m, mi.position + mi.front * 2.0);

        // --- 3D DRAWING ---

        clear_background(SKYBLUE);

        set_camera(&Camera3D {
            position: mi.position,
            up: mi.up,
            target: mi.position + mi.front,
            fovy: 90.0_f32.to_radians(),
            ..Default::default()
        });

        let basemesh = m.gen_mesh();
        let fullmesh = tool.gen_mesh(&m);

        draw_mesh(&basemesh);
        draw_mesh_wires(&fullmesh, BLACK);
        } // end mut mi reference

        // --- 2D Drawing ---

        set_default_camera();

        // Draw crosshair
        draw_circle_lines((conf().window_width / 2) as f32, (conf().window_height / 2) as f32, 5.0, 1.0, RED);
        draw_circle_lines((conf().window_width / 2) as f32, (conf().window_height / 2) as f32, 0.0, 1.0, RED);

        // Draw debug if needed
        if f3
        {
            draw_info(&p, &m);
        }

        draw_toolbar(&tb);

        next_frame().await;
    }
}

pub fn draw_toolbar(tb: &Toolbar)
{
    root_ui().window(hash!(), Vec2::new((conf().window_width as f32 / 2.) - 200., conf().window_height as f32 - 80.), Vec2::new(400., 60.), |ui| {
            for i in 1..tb.tools.len()
            {
                let tool = &tb.tools[i];
            
                if i == tb.selected_tool
                {
                    let red_style = ui.style_builder()
                    .text_color(RED)
                    .build();

                    let red_skin = Skin {
                        label_style: red_style,
                        ..ui.default_skin()
                    };

                    ui.push_skin(&red_skin);
                    ui.label(None, &tool.as_ref().get_name());
                    ui.pop_skin();
                }
                else 
                {
                    ui.label(None, &tool.as_ref().get_name());
                }
                ui.same_line(0.0);
            }
        });
}

/// Draws text information from p and m on the screen
pub fn draw_info(p: &Player, m: &Model)
{
    let mi = &p.mi;
    draw_text(
        format!("Player pos: X: {} Y: {} Z: {}", mi.position.x, mi.position.y, mi.position.z).as_str(),
        10.0,
        30.0,
        30.0,
        BLACK,
    );
    let intsect = m.send_ray(mi.position, mi.front);
    if let Some(intsect) = intsect
    {
        draw_text(
            format!("Intersect X: {} Y: {} Z: {}", intsect.x, intsect.y, intsect.z).as_str(),
            10.0,
            30.0 + 18.0,
            30.0,
            BLACK,
        );
    }
    else 
    {
        draw_text(
            format!("No intersect!").as_str(),
            10.0,
            30.0 + 18.0,
            30.0,
            BLACK,
        );  
    }
}