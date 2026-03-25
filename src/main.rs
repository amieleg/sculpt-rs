use std::path::Path;
use std::path::PathBuf;

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
mod material_atlas;

use crate::file_io::*;
use crate::material_atlas::MaterialAtlas;
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
pub enum EditorMode
{
    Normal,
    ToolSettings,
    Paused
}

pub struct EditorState
{
    mode: EditorMode,
    debug: bool,
    grid: bool
}

pub struct FileSettings
{
    model_file_path: PathBuf,
    texture_file_path: PathBuf,
}

fn hide_cursor(toggle: bool, p: &mut Player)
{
    p.mi.grabbed = toggle;
    set_cursor_grab(toggle);
    show_mouse(!toggle);
}

#[macroquad::main(conf)]
async fn main() {
    let mut file_settings = FileSettings{model_file_path: PathBuf::from("out.obj"), texture_file_path: PathBuf::from("assets/cubetexturecolor64-48.png")};

    let icon_atlas = TextureAtlas::icons_atlas().await;

    let material_atlas = MaterialAtlas::from_folder(Path::new("assets/seamlessTextures2PNG")).await.unwrap();

    let mut m = Model::new(Some(material_atlas.compile()));
    let mut p = Player::new();
    let mut tb = Toolbar::new(material_atlas).await;

    let mut state = EditorState{mode: EditorMode::Normal, debug: false, grid: true};


    set_cursor_grab(p.mi.grabbed);
    show_mouse(false);


    loop {

        // --- UPDATE / INPUT ---

        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape)
        {
            if state.mode == EditorMode::Paused
            {
                state.mode = EditorMode::Normal;
            }
            else 
            {
                state.mode = EditorMode::Paused;
            }

            hide_cursor(!p.mi.grabbed, &mut p);
        }
        if is_key_pressed(KeyCode::F3)
        {
            state.debug = !state.debug;
        }
        if is_key_pressed(KeyCode::G)
        {
            state.grid = !state.grid;
        }
        if state.mode != EditorMode::Paused
        {
            if is_mouse_button_pressed(MouseButton::Right)
            {
                if state.mode == EditorMode::Normal
                {
                    state.mode = EditorMode::ToolSettings;
                    hide_cursor(false, &mut p,);
                }
                else if state.mode == EditorMode::ToolSettings
                {
                    state.mode = EditorMode::Normal;
                    hide_cursor(true, &mut p);
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

        if state.mode == EditorMode::Normal
        {
            p.update(delta);

            if tb.update()
            {
                tb.tools[tb.last_tool].shut_down(&mut m, &p);
                tb.get_current_tool_mut().start_up(&mut m, &p);
            }
        
            tb.get_current_tool_mut().update(&mut m, &p);
        }

        if state.grid
        {
            draw_grid(16, 1.0, BLACK, BLACK);
        }

        let basemesh = m.gen_mesh();
        draw_mesh(&basemesh);
        tb.get_current_tool_mut().draw_mesh(&m);

        // --- 2D Drawing ---

        set_default_camera();

        // Draw crosshair
        draw_circle_lines(screen_width() / 2., screen_height() / 2., 5.0, 1.0, RED);
        draw_circle_lines(screen_width() / 2., screen_height() / 2., 0.0, 2.0, RED);

        tb.draw_toolbar(&icon_atlas);
        if state.mode == EditorMode::ToolSettings
        {
            tb.get_current_tool_mut().open_settings();
        }
        else if state.mode == EditorMode::Paused
        {
            draw_paused(&file_settings, &mut m);
        }

        if state.debug
        {
            draw_debug(&p, &m);
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