// src/components/paddl.rs
#[derive(Debug, Clone)]
pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub velocity_y: f32,
    pub is_player: bool,
}

impl Paddle {
    pub fn new(x: f32, y: f32, is_player: bool) -> Self {
        Self {
            x,
            y,
            velocity_y: 0.0,
            is_player,
        }
    }
}
