// src/systems/movement.rs
use crate::components::{Ball, Paddle};
use crate::engine::Config;

pub struct MovementSystem;

impl MovementSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn update(
        &self,
        ball: &mut Ball,
        player_paddle: &mut Paddle,
        ai_paddle: &mut Paddle,
        delta_time: f32,
        config: &Config,
    ) {
        // Update ball position
        ball.x += ball.velocity_x * delta_time;
        ball.y += ball.velocity_y * delta_time;

        // Update player paddle (keep within bounds)
        player_paddle.y += player_paddle.velocity_y * delta_time;

        // Constrain player paddle to stay within screen bounds
        let min_y = 0.0;
        let max_y = config.window_height as f32 - config.paddle_height;
        player_paddle.y = player_paddle.y.clamp(min_y, max_y);

        // Simple AI for computer paddle
        let paddle_center = ai_paddle.y + 50.0;
        if ball.y < paddle_center - 10.0 {
            ai_paddle.velocity_y = -200.0;
        } else if ball.y > paddle_center + 10.0 {
            ai_paddle.velocity_y = 200.0;
        } else {
            ai_paddle.velocity_y = 0.0;
        }

        ai_paddle.y += ai_paddle.velocity_y * delta_time;

        // Constrain AI paddle to stay within screen bounds
        let min_y = 0.0;
        let max_y = config.window_height as f32 - config.paddle_height;
        ai_paddle.y = ai_paddle.y.clamp(min_y, max_y);
    }
}
