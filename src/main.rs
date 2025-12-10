use macroquad::prelude::*;
use macroquad::amiel::*;

mod my_model;
mod utils;
mod player;
mod tools;
mod toolbar;
mod aligners;
mod file_io;
mod paused;

use crate::file_io::*;
use crate::my_model::*;
use crate::player::*;
use crate::paused::draw_paused;
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

pub const PIXEL_SIZE: u16 = 1;

pub static mut SMALL_GRID_SIZE: f32 = 0.1;
pub static mut PLAYER_REACH: f32 = 0.1;
pub static mut VERTEX_IDENTIFIER_SIZE: f32 = 0.1;

#[derive(PartialEq)]
pub enum EditorState
{
    Normal,
    ToolSettings,
    Paused
}

pub struct FileSettings
{
    model_file_path: String,
    texture_file_path: String,
}

#[macroquad::main(conf)]
async fn main() {
    let mut file_settings = FileSettings{model_file_path: String::from("out.obj"), texture_file_path: String::from("assets/cubetexturecolor64-48.png")};
    let texture = load_texture(&file_settings.texture_file_path).await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    let icon_atlas = TextureAtlas::icons_atlas().await;

    let mut f3 = false;

    //let mut m = Model::new(Some(texture));//Model::gen_cube_model(Some(texture));
    let mut m = load_obj_file(&file_settings.model_file_path).unwrap();
    m.texture = Some(texture);

    let mut p = Player::new();

    let mut tb = Toolbar::new().await;// toolbar

    let mut state = EditorState::Normal;


    set_cursor_grab(p.mi.grabbed);
    show_mouse(false);

    loop {

        // --- UPDATE / INPUT ---

        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape)
        {
            if state == EditorState::Paused
            {
                state = EditorState::Normal;
            }
            else 
            {
                state = EditorState::Paused;
            }

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
        if state != EditorState::Paused
        {
            if is_mouse_button_pressed(MouseButton::Right)
            {
                if state == EditorState::Normal
                {
                    state = EditorState::ToolSettings;
                    set_cursor_grab(false);
                    show_mouse(true);
                }
                else if state == EditorState::ToolSettings
                {
                    state = EditorState::Normal;
                    set_cursor_grab(true);
                    show_mouse(false);
                }
            }
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

        if state == EditorState::Normal
        {
            p.update(delta);

            if tb.update()
            {
                tb.tools[tb.last_tool].shut_down(&mut m, &p);

                tb.get_current_tool_mut().start_up(&mut m, &p);
            }
        
            tb.get_current_tool_mut().update(&mut m, &p);
        }

        draw_grid(16, 1.0, BLACK, BLACK);

        let basemesh = m.gen_mesh();
        let fullmesh = tb.get_current_tool_mut().gen_mesh(&m);

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

        if state == EditorState::ToolSettings
        {
            tb.get_current_tool_mut().open_settings();
        }
        if state == EditorState::Paused
        {
            draw_paused(&file_settings, &mut m);
        }

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
    draw_text(
        format!("Drawing {} vertices and {} polys", m.vertices.len(), m.polys.len()).as_str(),
        10.0,
        60.0,
        30.0,
        BLACK,
    );
}