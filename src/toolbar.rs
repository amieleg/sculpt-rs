use std::path::Path;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};
use crate::material_atlas::MaterialAtlas;
use crate::my_model::Model;
use crate::{tools::*};
use crate::tools::hand_tool::*;
use crate::tools::placer_tool::*;
use crate::tools::single_vertex_tool::*;
use crate::tools::single_vertex_mover::*;
use crate::tools::brush_tool::*;
use crate::tools::inspect_tool::*;
use crate::tools::deleter_tool::*;
use crate::tools::single_poly_tool::*;
use crate::tools::poly_brush_tool::*;
use crate::utils::TextureAtlas;

const TOOLBAR_SIZE: usize = 9;
const ICON_SIZE: f32 = 64.;
const TOOLBAR_WIDTH: f32 = ICON_SIZE * TOOLBAR_SIZE as f32 + ((TOOLBAR_SIZE + 1) as f32 * 3.);
const TOOLBAR_HEIGHT: f32 = ICON_SIZE + 6.;
pub struct Toolbar
{
    pub tools: Vec<Box<dyn ModelTool>>,
    pub last_tool: usize,
    pub selected_tool: usize,
}

impl Toolbar
{
    pub async fn new(atlas: MaterialAtlas) -> Toolbar
    {
        let mut tools: Vec<Box<dyn ModelTool>> = Vec::with_capacity(TOOLBAR_SIZE + 1); // +1 to accomodate the handtool that is always present
        tools.push(Box::new(HandTool::new()));

        let tempmodel = Model::gen_cube_model(None);
        let cube_tool: Box<dyn ModelTool> = Box::new(PlacerTool::new(String::from("Cube Placer Tool"), ModelAddition{vertices: tempmodel.vertices, polys: tempmodel.polys}));

        let tempmodel = Model::new(None);
        let triangle_tool: Box<dyn ModelTool> = Box::new(PlacerTool::new(String::from("Other Placer Tool"), ModelAddition{vertices: tempmodel.vertices, polys: tempmodel.polys}));

        let sv_tool: Box<dyn ModelTool> = Box::new(SingleVertexTool::new());

        let sp_tool: Box<dyn ModelTool> = Box::new(SinglePolyTool::new());

        let svm_tool: Box<dyn ModelTool> = Box::new(SingleVertexMover::new());

        let inspect_tool: Box<dyn ModelTool> = Box::new(InspectTool::new());

        let brush_tool: Box<dyn ModelTool> = Box::new(BrushTool::new("assets/cubetexturecolor64-48.png").await);

        let material_brush_tool: Box<dyn ModelTool> = Box::new(PolyBrushTool::new(atlas).await);

        let deleter_tool: Box<dyn ModelTool> = Box::new(DeleterTool::new());

        tools.append(&mut vec![cube_tool, triangle_tool, sv_tool, sp_tool, svm_tool, inspect_tool, brush_tool, material_brush_tool, deleter_tool]);
        
        Toolbar
        {
            tools: tools,
            selected_tool: 0,
            last_tool: 0,
        }
    }

    pub fn get_current_tool_mut(&mut self) -> &mut Box<dyn ModelTool>
    {
        if self.selected_tool >= self.tools.len()
        {
            return &mut self.tools[0];
        }
        return &mut self.tools[self.selected_tool]
    }

    pub fn get_current_tool(&self) -> & Box<dyn ModelTool>
    {
        if self.selected_tool >= self.tools.len()
        {
            return &self.tools[0];
        }
        return &self.tools[self.selected_tool]
    }

    pub fn switch_tool_to(&mut self, index: usize)
    {
        if self.selected_tool == index
        {
            self.selected_tool = 0;
        }
        else if index < self.tools.len()
        {
            self.selected_tool = index;
        }
    }

    pub fn update(&mut self) -> bool
    {
        if is_key_pressed(KeyCode::Key0)
        {
            self.switch_tool_to(0);
        }
        else if is_key_pressed(KeyCode::Key1)
        {
            self.switch_tool_to(1);
        }
        else if is_key_pressed(KeyCode::Key2)
        {
            self.switch_tool_to(2);
        }
        else if is_key_pressed(KeyCode::Key3)
        {
            self.switch_tool_to(3);
        }
        else if is_key_pressed(KeyCode::Key4)
        {
            self.switch_tool_to(4);
        }
        else if is_key_pressed(KeyCode::Key5)
        {
            self.switch_tool_to(5);
        }
        else if is_key_pressed(KeyCode::Key6)
        {
            self.switch_tool_to(6);
        }
        else if is_key_pressed(KeyCode::Key7)
        {
            self.switch_tool_to(7);
        }
        else if is_key_pressed(KeyCode::Key8)
        {
            self.switch_tool_to(8);
        }
        else if is_key_pressed(KeyCode::Key9)
        {
            self.switch_tool_to(9);
        }

        if self.last_tool != self.selected_tool
        {
            self.last_tool = self.selected_tool;
            return true;
        }
        return false;
    }

    pub fn draw_toolbar(&self, icon_atlas: &TextureAtlas)
    {
        let selected_skin = Skin {
            label_style: {root_ui().style_builder().text_color(RED).build()},
            ..root_ui().default_skin()
        };

        
        let current_tool = self.get_current_tool();
        draw_text(current_tool.get_name(), screen_width() / 2. - (current_tool.get_name().len() * 5) as f32, screen_height() - TOOLBAR_HEIGHT - 10. - 10., 20., BLACK);
        

        root_ui().window(hash!(screen_width() as u64, screen_height() as u64, "toolbar"), Vec2::new((screen_width() / 2.) - TOOLBAR_WIDTH / 2., screen_height() - TOOLBAR_HEIGHT - 10.), Vec2::new(TOOLBAR_WIDTH, TOOLBAR_HEIGHT), |ui| {
            for i in 1..self.tools.len()
            {
                let tool = &self.tools[i];

                if i == self.selected_tool
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
}