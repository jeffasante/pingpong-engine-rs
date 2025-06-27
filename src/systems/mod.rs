// src/systems/mod.rs
pub mod movement;
pub mod collision;
pub mod scoring;

pub use movement::MovementSystem;
pub use collision::CollisionSystem;
pub use scoring::ScoringSystem;

