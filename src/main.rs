use macroquad::prelude::*;
use macroquad::amiel::*;

mod my_model;
mod utils;
mod player;
mod tools;
mod toolbar;
mod aligners;

use crate::my_model::*;
use crate::player::*;
use crate::toolbar::Toolbar;
use crate::utils::TextureAtlas;

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
    let texture = load_texture("assets/cubetexturecolor64-48.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    let icon_atlas = TextureAtlas::icons_atlas().await;

    let mut f3 = false;

    let mut m = Model::gen_cube_model(Some(texture));

    let mut p = Player::new();

    let mut tb = Toolbar::new().await;// toolbar


    set_cursor_grab(p.mi.grabbed);
    show_mouse(false);

    loop {

        // --- UPDATE / INPUT ---

        let delta = get_frame_time();
        p.update(delta);

        if is_key_pressed(KeyCode::Escape)
        {
            break;
        }
        if is_key_pressed(KeyCode::Tab) 
        {
            p.mi.grabbed = !p.mi.grabbed;
            set_cursor_grab(p.mi.grabbed);
            show_mouse(!p.mi.grabbed);
        }
        if is_key_pressed(KeyCode::F3)
        {
            f3 = !f3;
        }

        // --- 3D DRAWING ---

        clear_background(SKYBLUE);

        set_camera(&Camera3D {
            position: p.mi.position,
            up: p.mi.up,
            target: p.mi.position + p.mi.front,
            fovy: 90.0_f32.to_radians(),
            ..Default::default()
        });

        if tb.update()
        {
            let tool = tb.get_current_tool_mut();
            tool.start_up(&mut m, &p);
        }
    
        let tool = tb.get_current_tool_mut();
        tool.update(&mut m, &p);

        draw_grid(16, 1.0, BLACK, BLACK);

        let basemesh = m.gen_mesh();
        let fullmesh = tool.gen_mesh(&m);

        draw_mesh(&basemesh);
        draw_mesh_wires(&fullmesh, BLACK);

        // --- 2D Drawing ---

        set_default_camera();

        // Draw crosshair
        draw_circle_lines(screen_width() / 2., screen_height() / 2., 5.0, 1.0, RED);
        draw_circle_lines(screen_width() / 2., screen_height() / 2., 0.0, 2.0, RED);

        // Draw debug if needed
        if f3
        {
            draw_debug(&p, &m);
        }

        tb.draw_toolbar(&icon_atlas);

        next_frame().await;
    }
}



/// Draws text information from p and m on the screen
pub fn draw_debug(p: &Player, m: &Model)
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
            format!("Intersect X: {} Y: {} Z: {}", intsect.0.x, intsect.0.y, intsect.0.z).as_str(),
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