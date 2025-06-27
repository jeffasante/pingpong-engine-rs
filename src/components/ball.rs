// src/components/mod.rs
#[derive(Debug, Clone)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    // pub radius: f32,
    // pub color: String,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x,
            y: y,
            velocity_x: 200.0, // Initial velocity
            velocity_y: 100.0,
        }
    }

    pub fn reset(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.velocity_x = if self.velocity_x > 0.0 { -200.0 } else { 200.0 };
        self.velocity_y = 100.0;
    }
}

