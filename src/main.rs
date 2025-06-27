// src/main.rs
mod components;
mod engine;
mod systems;

use engine::{Config, Engine};

use crate::components::Paddle;
use crate::systems::ScoringSystem;
use components::{Ball, Score};
use systems::{CollisionSystem, MovementSystem};

fn main() {
    let config = Config::default();
    let mut engine = Engine::new(config);

    // Initialize game objects
    let mut ball = Ball::new(400.0, 300.0);
    let mut player_paddle = Paddle::new(50.0, 250.0, true);
    let mut ai_paddle = Paddle::new(750.0, 250.0, false);
    let mut score = Score::new();

    // Game systems
    let movement_system = MovementSystem::new();
    let collision_system = CollisionSystem::new();
    let mut scoring_system = ScoringSystem::new();

    // Welcome message
    println!("Welcome to Pong!");

    // Main game loop
    while engine.is_running() {
        let delta_time = engine.get_delta_time();

        // Handle input
        engine.handle_input(&mut player_paddle);

        // Update systems
        // movement_system.update(&mut ball, &mut player_paddle, &mut ai_paddle, delta_time);
        movement_system.update(
            &mut ball,
            &mut player_paddle,
            &mut ai_paddle,
            delta_time,
            &engine.config,
        );

        collision_system.update(&mut ball, &player_paddle, &ai_paddle, &engine.config);
        scoring_system.update(&mut ball, &mut score, &engine.config);

        // Render
        engine.clear();
        engine.render_paddle(&player_paddle);
        engine.render_paddle(&ai_paddle);
        engine.render_ball(&ball);
        engine.render_score(&score);
        engine.present();
    }
}
