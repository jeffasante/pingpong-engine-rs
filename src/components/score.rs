// src/components/score.rs
#[derive(Debug, Clone)]
pub struct Score {
    pub player_score: u32,
    pub ai_score: u32,
    // pub computer_score: u32,
}

#[allow(dead_code)]
impl Score {
    pub fn new() -> Self {
        Self {
            player_score: 0,
            ai_score: 0,
        }
    }
    
    pub fn player_scores(&mut self) {
        self.player_score += 1;
    }
    
    pub fn ai_scores(&mut self) {
        self.ai_score += 1;
    }
    
    pub fn reset(&mut self) {
        self.player_score = 0;
        self.ai_score = 0;
    }
}
