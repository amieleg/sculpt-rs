use macroquad::prelude::*;
use macroquad::amiel::*;
use macroquad::ui::Skin;
use macroquad::ui::Style;
use macroquad::ui::StyleBuilder;
use macroquad::ui::widgets::Group;
use macroquad::ui::{self, hash, widgets, root_ui, Id};

mod my_model;
mod utils;
mod player;
mod tools;
mod toolbar;
mod aligners;

use crate::aligners::Aligner;
use crate::aligners::GridAligner;
use crate::my_model::*;
use crate::player::*;
use crate::tools::*;
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
    let texture = load_texture("assets/checker128.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    let icon_atlas = TextureAtlas::icons_atlas().await;

    let mut f3 = false;

    let mut m = Model::new(texture);

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
        draw_circle_lines(screen_height() / 2., screen_width() / 2., 0.0, 1.0, RED);

        // Draw debug if needed
        if f3
        {
            draw_debug(&p, &m);
        }

        draw_toolbar(&tb, &icon_atlas);
        draw_info(&p, &m);
        //draw_info2();

        next_frame().await;
    }
}

pub fn draw_toolbar(tb: &Toolbar, icon_atlas: &TextureAtlas)
{
    let selected_skin = Skin {
        label_style: {root_ui().style_builder().text_color(RED).build()},
        ..root_ui().default_skin()
    };

    if is_key_pressed(KeyCode::F4)
    {
        println!("{}", root_ui().default_skin().margin);
    }

    root_ui().window((screen_width() * screen_height()) as u64, Vec2::new((screen_width() / 2.) - 200., screen_height() as f32 - 80.), Vec2::new(400., 70.), |ui| {
        for i in 1..tb.tools.len()
        {
            let tool = &tb.tools[i];

            if i == tb.selected_tool
            {
                ui.push_skin(&selected_skin);
                ui.texture(icon_atlas.get_texture(tool.get_texture_index()), 64., 64.);
                ui.label(Vec2::new((i-1) as f32 * 64.,0.), &format!("[{}]",i));
                ui.pop_skin();
            }
            else
            {
                ui.texture(icon_atlas.get_texture(tool.get_texture_index()), 64., 64.);  

                ui.label(Vec2::new((i-1) as f32 * 64.,0.), &format!("[{}]",i));
            }
            ui.same_line(0.0);
        }
    });
}

pub fn draw_info(p: &Player, m: &Model)
{
    let looking_at = m.send_ray(p.mi.position, p.mi.front);

    widgets::Window::new(/*(screen_width() * screen_height() + 1.) as u64*/ hash!(), Vec2::new(screen_width() - 310., screen_height() - 210.), Vec2::new(300., 200.)).titlebar(false).ui(&mut *root_ui(), |ui|
    {
        if let Some((_, tri)) = looking_at
        {
            let looking_at_vertices = m.tri_as_vertices(tri);
            
            Group::new(hash!(), Vec2::new(290., 40.)).ui(ui, |ui| {
                ui.label(Vec2::new(0., 0.), "Looking at triangle:");
            });

            Group::new(hash!(), Vec2::new(290., 150.)).ui(ui, |ui| {
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

pub fn draw_info2()
{
    widgets::Window::new(hash!(), Vec2::new(screen_width() - 310., screen_height() - 210.), Vec2::new(300., 200.)).titlebar(false).ui(&mut *root_ui(), |ui|
    {
        Group::new(hash!(), Vec2::new(290., 40.)).ui(ui, |ui| {
            ui.label(Vec2::new(0., 0.), "Looking at triangle:");
        });

        Group::new(hash!(), Vec2::new(290., 80.)).ui(ui, |ui| {
            ui.label(Vec2::new(0., 0.), &format!("Vertex {}:", 1));
        });
    });
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