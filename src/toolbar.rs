use macroquad::prelude::*;
use crate::my_model::Model;
use crate::tools::*;
use crate::tools::hand_tool::*;
use crate::tools::placer_tool::*;
use crate::tools::single_vertex_tool::*;

pub const TOOLBAR_SIZE: usize = 5;
pub struct Toolbar
{
    pub tools: Vec<Box<dyn ModelTool>>,
    pub selected_tool: usize,
}

impl Toolbar
{
    pub fn new() -> Toolbar
    {
        let mut tools: Vec<Box<dyn ModelTool>> = Vec::with_capacity(TOOLBAR_SIZE + 1); // +1 to accomodate the handtool that is always present
        tools.push(Box::new(HandTool::new()));

        let tempmodel = Model::gen_cube_model(None);
        let cube_tool: Box<dyn ModelTool> = Box::new(PlacerTool::new(String::from("Cube Placer Tool"), ModelAddition{vertices: tempmodel.vertices, triangles: tempmodel.triangles}));
        tools.push(cube_tool);

        let sv_tool: Box<dyn ModelTool> = Box::new(SingleVertexTool::new());
        tools.push(sv_tool);
        Toolbar
        {
            tools: tools,
            selected_tool: 0,
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

    pub fn update(&mut self) -> bool
    {
        let tool_before = self.selected_tool;
        if is_key_pressed(KeyCode::Key0)
        {
            self.selected_tool = 0;
        }
        if is_key_pressed(KeyCode::Key1)
        {
            self.selected_tool = 1;
        }
        else if is_key_pressed(KeyCode::Key2)
        {
            self.selected_tool = 2;
        }
        else if is_key_pressed(KeyCode::Key3)
        {
            self.selected_tool = 3;
        }
        else if is_key_pressed(KeyCode::Key4)
        {
            self.selected_tool = 4;
        }
        else if is_key_pressed(KeyCode::Key5)
        {
            self.selected_tool = 5;
        }

        if tool_before != self.selected_tool
        {
            return true;
        }
        return false;
    }
}