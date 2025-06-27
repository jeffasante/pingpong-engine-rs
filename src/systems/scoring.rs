// src/systems/scoring.rs
use crate::components::{Ball, Score};
use crate::engine::Config;

pub struct ScoringSystem;

impl ScoringSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, ball: &mut Ball, score: &mut Score, config: &Config) {
        // Check if ball went off screen
        if ball.x < 0.0 {
            score.ai_scores();
            ball.reset(config.window_width as f32 / 2.0, config.window_height as f32 / 2.0);
        } else if ball.x > config.window_width as f32 {
            score.player_scores();
            ball.reset(config.window_width as f32 / 2.0, config.window_height as f32 / 2.0);
        }
    }

}