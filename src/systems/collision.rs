// src/systems/collision.rs
use crate::components::{Ball, Paddle};
use crate::engine::Config;

pub struct CollisionSystem;

#[allow(dead_code)]
impl CollisionSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn update(
        &self,
        ball: &mut Ball,
        player_paddle: &Paddle,
        ai_paddle: &Paddle,
        config: &Config,
    ) {
        // Wall collisions (top and bottom)
        if ball.y <= config.ball_size / 2.0
            || ball.y >= config.window_height as f32 - config.ball_size / 2.0
        {
            ball.velocity_y = -ball.velocity_y;
        }

        // Paddle collisions
        self.check_paddle_collision(ball, player_paddle, config);
        self.check_paddle_collision(ball, ai_paddle, config);
    }

    fn check_paddle_collision(&self, ball: &mut Ball, paddle: &Paddle, config: &Config) {
        let ball_left = ball.x - config.ball_size / 2.0;
        let ball_right = ball.x + config.ball_size / 2.0;
        let ball_top = ball.y - config.ball_size / 2.0;
        let ball_bottom = ball.y + config.ball_size / 2.0;

        let paddle_left = paddle.x;
        let paddle_right = paddle.x + config.paddle_width;
        let paddle_top = paddle.y;
        let paddle_bottom = paddle.y + config.paddle_height;

        if ball_right >= paddle_left
            && ball_left <= paddle_right
            && ball_bottom >= paddle_top
            && ball_top <= paddle_bottom
        {
            ball.velocity_x = -ball.velocity_x;

            // Add some variation to the bounce angle based on where the ball hits the paddle
            let paddle_center = paddle.y + config.paddle_height / 2.0;
            let hit_pos = (ball.y - paddle_center) / (config.paddle_height / 2.0);
            ball.velocity_y += hit_pos * 100.0;
        }
    }

    fn constrain_paddle(&self, paddle: &mut Paddle, config: &Config) {
        if paddle.y < 0.0 {
            paddle.y = 0.0;
        } else if paddle.y > config.window_height as f32 - config.paddle_height {
            paddle.y = config.window_height as f32 - config.paddle_height;
        }
    }
}
