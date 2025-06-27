// src/engine/time.rs
use std::time::{Duration, Instant};


pub struct Timer {
    last_frame: Instant,
    target_frame_time: Duration,
    // delta: Duration,
}


#[allow(dead_code)]
impl Timer {
    pub fn new(fps_target: u64) -> Self {
        Self {
            last_frame: Instant::now(),
            target_frame_time: Duration::from_nanos(1_000_000_000u64 / fps_target),
        }
    }

    pub fn delta_time(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);
        self.last_frame = now;
        delta.as_secs_f32()
    }

    pub fn should_update(&self) -> bool {
        self.last_frame.elapsed() >= self.target_frame_time
    }

    pub fn get_delta_time(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);
        self.last_frame = now;
        delta.as_secs_f32()
    }
}
