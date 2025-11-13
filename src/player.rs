use crate::utils::drop_y_normalize;
use macroquad::prelude::*;


pub const MOVE_SPEED: f32 = 0.02;
pub const LOOK_SPEED: f32 = 0.1;

#[derive(PartialEq)]
pub enum PlayerState
{
    None,
    AddingPoint
}

pub struct Player
{
    pub mi: BigMovementInfo,
    pub state: PlayerState,
}

impl Player 
{
    pub fn new() -> Player
    {
        Player
        {
            mi: BigMovementInfo::new(),
            state: PlayerState::None,
        }
    }

    pub fn update(&mut self, delta: f32)
    {
        self.mi.update(delta);
        self.movement_update();
    }

    pub fn movement_update(&mut self)
    {
        if is_key_down(KeyCode::W) 
        {
            self.mi.position += drop_y_normalize(self.mi.front) * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) 
        {
            self.mi.position -= drop_y_normalize(self.mi.front) * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) 
        {
            self.mi.position -= drop_y_normalize(self.mi.right) * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) 
        {
            self.mi.position += drop_y_normalize(self.mi.right) * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Slash) || is_key_down(KeyCode::Space)
        {
            self.mi.position += self.mi.world_up * MOVE_SPEED;
        }
        if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)
        {
            self.mi.position -= self.mi.world_up * MOVE_SPEED;
        }
    }
}

pub struct BigMovementInfo
{
    pub x: f64,
    pub switch: bool,
    pub bounds: f64,
    pub world_up: Vec3, // up direction of the world
    pub yaw: f32,
    pub pitch: f32,
    pub front: Vec3, // front direction of the camera
    pub right: Vec3, // up direction of the camera
    pub up: Vec3, // up direction of the camera
    pub position: Vec3, // camera position
    pub last_mouse_position: Vec2, // position of mouse on previous frame
    pub grabbed: bool // whether cursor is grabbed
}

// From the macroquad example
impl BigMovementInfo
{
    pub fn new() -> BigMovementInfo
    {
        let x = 0.0;
        let  switch = false;
        let bounds = 8.0;

        let world_up = vec3(0.0, 1.0, 0.0);
        let yaw: f32 = 1.18;
        let pitch: f32 = 0.0;

        let front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();
        let right = front.cross(world_up).normalize();
        let up = right.cross(front).normalize();

        let position = vec3(0.0, 1.0, 0.0);
        let last_mouse_position: Vec2 = mouse_position().into();

        let grabbed = true;
        BigMovementInfo
        {
            x: x,
            switch: switch,
            bounds: bounds,
            world_up: world_up,
            yaw: yaw,
            pitch: pitch,
            front: front,
            right: right,
            up: up,
            position: position,
            last_mouse_position: last_mouse_position,
            grabbed: grabbed,
        }
    }

    pub fn update(&mut self, delta: f32)
    {
        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - self.last_mouse_position;

        self.last_mouse_position = mouse_position;

        if self.grabbed {
            self.yaw += mouse_delta.x * delta * LOOK_SPEED;
            self.pitch += mouse_delta.y * delta * -LOOK_SPEED;

            self.pitch = if self.pitch > 1.5 { 1.5 } else { self.pitch };
            self.pitch = if self.pitch < -1.5 { -1.5 } else { self.pitch };

            self.front = vec3(
                self.yaw.cos() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.sin() * self.pitch.cos(),
            )
            .normalize();

            self.right = self.front.cross(self.world_up).normalize();
            self.up = self.right.cross(self.front).normalize();

            self.x += if self.switch { 0.04 } else { -0.04 };
            if self.x >= self.bounds || self.x <= -self.bounds {
                self.switch = !self.switch;
            }
        }
    }
}

