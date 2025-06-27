// src/engine/game.rs
use crate::components::{Ball, Paddle, Score};
use crate::engine::Config;
use crate::engine::time::Timer;
use minifb::{Key, Window, WindowOptions};

pub struct Engine {
    window: Window,
    buffer: Vec<u32>,
    timer: Timer,
    pub config: Config,
    running: bool,
}

#[allow(dead_code)]
impl Engine {
    pub fn new(config: Config) -> Self {
        let mut window = Window::new(
            &config.window_title,
            config.window_width as usize,
            config.window_height as usize,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("Failed to create window: {}", e);
        });

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        // Initialize the game components
        let buffer = vec![0; (config.window_width * config.window_height) as usize];
        let timer = Timer::new(config.fps_target);

        Self {
            window,
            buffer,
            timer,
            config,
            running: true,
        }
    }

    pub fn is_running(&mut self) -> bool {
        self.running && self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn get_delta_time(&mut self) -> f32 {
        self.timer.get_delta_time()
    }

    pub fn handle_input(&mut self, paddle: &mut Paddle) {
        if paddle.is_player {
            let mut input_velocity = 0.0;
            if self.window.is_key_down(Key::Up) || self.window.is_key_down(Key::W) {
                input_velocity -= self.config.paddle_speed;
            }
            if self.window.is_key_down(Key::Down) || self.window.is_key_down(Key::S) {
                input_velocity += self.config.paddle_speed;
            }
            paddle.velocity_y = input_velocity;
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0x000000); // Black background
    }

    pub fn render_paddle(&mut self, paddle: &Paddle) {
        self.draw_rect(
            paddle.x as usize,
            paddle.y as usize,
            self.config.paddle_width as usize,
            self.config.paddle_height as usize,
            0xFFFFFF, // White color
        );
    }

    pub fn render_ball(&mut self, ball: &Ball) {
        self.draw_rect(
            (ball.x - self.config.ball_size / 2.0) as usize,
            (ball.y - self.config.ball_size / 2.0) as usize,
            self.config.ball_size as usize,
            self.config.ball_size as usize,
            0xFFFFFF, // White color
        );
    }

    // pub fn render_score(&mut self, score: &Score) {
    //     let left_score_x = (self.config.window_width / 4) as usize;
    //     let right_score_x = (self.config.window_height / 4) as usize;
    //     let score_y = 50;

    //     // Draw a simple score representation
    //     for i in 0..score.player_score {
    //         self.draw_rect(
    //             left_score_x + (i as usize) * 20,
    //             score_y as usize,
    //             10,
    //             10,
    //             0xFFFFFF, // White color
    //         );
    //     }

    //     for i in 0..score.ai_score {
    //         self.draw_rect(
    //             right_score_x + (i as usize) * 20,
    //             score_y as usize,
    //             10,
    //             10,
    //             0xFFFFFF, // White color
    //         );
    //     }
    // }

    pub fn render_score(&mut self, score: &Score) {
        let window_width = self.config.window_width as f32;
        let window_height = self.config.window_height as f32;

        // Draw center line first
        self.draw_center_line();

        // Score display settings
        let score_y = 80.0;
        let digit_width = 40.0;
        let digit_height = 60.0;
        let segment_thickness = 6;

        // Player score (left side)
        let player_score_x = window_width * 0.25 - digit_width / 2.0;
        self.draw_digital_number(
            score.player_score,
            player_score_x,
            score_y,
            digit_width,
            digit_height,
            segment_thickness,
        );

        // AI score (right side)
        let ai_score_x = window_width * 0.75 - digit_width / 2.0;
        self.draw_digital_number(
            score.ai_score,
            ai_score_x,
            score_y,
            digit_width,
            digit_height,
            segment_thickness,
        );

        // Draw "PLAYER" and "AI" labels
        self.draw_text_label("PLAYER", player_score_x, score_y - 40.0, 0xCCCCCC);
        self.draw_text_label(
            "AI",
            ai_score_x + digit_width / 4.0,
            score_y - 40.0,
            0xCCCCCC,
        );

        // Draw winning indicator if someone has high score
        if score.player_score >= 10 || score.ai_score >= 10 {
            if score.player_score > score.ai_score {
                self.draw_text_label("WINNER!", player_score_x - 10.0, score_y + 80.0, 0x00FF00);
            } else if score.ai_score > score.player_score {
                self.draw_text_label("WINNER!", ai_score_x - 10.0, score_y + 80.0, 0x00FF00);
            }
        }
    }

    fn draw_center_line(&mut self) {
        let window_width = self.config.window_width as usize;
        let window_height = self.config.window_height as usize;
        let center_x = window_width / 2;
        let dash_length = 20;
        let dash_gap = 15;
        let line_width = 4;

        let mut y = 0;
        while y < window_height {
            for dy in 0..dash_length.min(window_height - y) {
                for dx in 0..line_width {
                    let px = center_x - line_width / 2 + dx;
                    let py = y + dy;
                    if px < window_width && py < window_height {
                        let idx = py * window_width + px;
                        if idx < self.buffer.len() {
                            self.buffer[idx] = 0x444444; // Dark gray
                        }
                    }
                }
            }
            y += dash_length + dash_gap;
        }
    }

    fn draw_digital_number(
        &mut self,
        number: u32,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        thickness: usize,
    ) {
        // 7-segment display patterns for digits 0-9
        let patterns = [
            0b1111110, // 0: top, top-right, bottom-right, bottom, bottom-left, top-left
            0b0110000, // 1: top-right, bottom-right
            0b1101101, // 2: top, top-right, middle, bottom-left, bottom
            0b1111001, // 3: top, top-right, middle, bottom-right, bottom
            0b0110011, // 4: top-left, middle, top-right, bottom-right
            0b1011011, // 5: top, top-left, middle, bottom-right, bottom
            0b1011111, // 6: top, top-left, middle, bottom-left, bottom, bottom-right
            0b1110000, // 7: top, top-right, bottom-right
            0b1111111, // 8: all segments
            0b1111011, // 9: top, top-left, top-right, middle, bottom-right, bottom
        ];

        let digit = (number % 10) as usize;
        let pattern = if digit < patterns.len() {
            patterns[digit]
        } else {
            0
        };

        let seg_width = width;
        let seg_height = height / 2.0;
        let seg_thickness = thickness as f32;

        // Draw segments based on pattern
        if pattern & 0b1000000 != 0 {
            // top
            self.draw_horizontal_segment(x, y, seg_width, seg_thickness);
        }
        if pattern & 0b0100000 != 0 {
            // top-right
            self.draw_vertical_segment(x + seg_width - seg_thickness, y, seg_height, seg_thickness);
        }
        if pattern & 0b0010000 != 0 {
            // bottom-right
            self.draw_vertical_segment(
                x + seg_width - seg_thickness,
                y + seg_height,
                seg_height,
                seg_thickness,
            );
        }
        if pattern & 0b0001000 != 0 {
            // bottom
            self.draw_horizontal_segment(x, y + height - seg_thickness, seg_width, seg_thickness);
        }
        if pattern & 0b0000100 != 0 {
            // bottom-left
            self.draw_vertical_segment(x, y + seg_height, seg_height, seg_thickness);
        }
        if pattern & 0b0000010 != 0 {
            // top-left
            self.draw_vertical_segment(x, y, seg_height, seg_thickness);
        }
        if pattern & 0b0000001 != 0 {
            // middle
            self.draw_horizontal_segment(
                x,
                y + seg_height - seg_thickness / 2.0,
                seg_width,
                seg_thickness,
            );
        }
    }

    fn draw_horizontal_segment(&mut self, x: f32, y: f32, width: f32, thickness: f32) {
        self.draw_rect(
            x as usize,
            y as usize,
            width as usize,
            thickness as usize,
            0x00FFFF, // Cyan color for segments
        );
    }

    fn draw_vertical_segment(&mut self, x: f32, y: f32, height: f32, thickness: f32) {
        self.draw_rect(
            x as usize,
            y as usize,
            thickness as usize,
            height as usize,
            0x00FFFF, // Cyan color for segments
        );
    }

    fn draw_text_label(&mut self, text: &str, x: f32, y: f32, color: u32) {
        // Simple bitmap font rendering for labels
        let char_width = 8;
        let char_height = 12;
        let char_spacing = 2;

        for (i, ch) in text.chars().enumerate() {
            let char_x = x + (i as f32) * (char_width + char_spacing) as f32;
            self.draw_simple_char(ch, char_x, y, char_width, char_height, color);
        }
    }

    fn draw_simple_char(
        &mut self,
        ch: char,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
        color: u32,
    ) {
        // Very basic character patterns
        let pattern = match ch {
            'P' => vec![
                0b11111000, 0b10001000, 0b10001000, 0b11111000, 0b10000000, 0b10000000, 0b10000000,
                0b10000000,
            ],
            'L' => vec![
                0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000,
                0b11111000,
            ],
            'A' => vec![
                0b01110000, 0b10001000, 0b10001000, 0b10001000, 0b11111000, 0b10001000, 0b10001000,
                0b10001000,
            ],
            'Y' => vec![
                0b10001000, 0b10001000, 0b10001000, 0b01010000, 0b00100000, 0b00100000, 0b00100000,
                0b00100000,
            ],
            'E' => vec![
                0b11111000, 0b10000000, 0b10000000, 0b11110000, 0b10000000, 0b10000000, 0b10000000,
                0b11111000,
            ],
            'R' => vec![
                0b11111000, 0b10001000, 0b10001000, 0b11111000, 0b11000000, 0b10100000, 0b10010000,
                0b10001000,
            ],
            'I' => vec![
                0b11111000, 0b00100000, 0b00100000, 0b00100000, 0b00100000, 0b00100000, 0b00100000,
                0b11111000,
            ],
            'W' => vec![
                0b10001000, 0b10001000, 0b10001000, 0b10001000, 0b10101000, 0b10101000, 0b11011000,
                0b10001000,
            ],
            'N' => vec![
                0b10001000, 0b11001000, 0b10101000, 0b10101000, 0b10101000, 0b10011000, 0b10001000,
                0b10001000,
            ],
            '!' => vec![
                0b00100000, 0b00100000, 0b00100000, 0b00100000, 0b00100000, 0b00000000, 0b00100000,
                0b00100000,
            ],
            _ => vec![0; 8], // Space or unknown character
        };

        for (row, &bits) in pattern.iter().enumerate() {
            for col in 0..width {
                if bits & (0b10000000 >> col) != 0 {
                    let px = x + col as f32;
                    let py = y + row as f32;
                    if px >= 0.0 && py >= 0.0 {
                        self.draw_rect(px as usize, py as usize, 1, 1, color);
                    }
                }
            }
        }
    }

    pub fn present(&mut self) {
        self.window
            .update_with_buffer(
                &self.buffer,
                self.config.window_width as usize,
                self.config.window_height as usize,
            )
            .unwrap();
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let window_width = self.config.window_width as usize;
        let window_height = self.config.window_height as usize;

        for dy in 0..height {
            for dx in 0..width {
                let pixel_x = x + dx;
                let pixel_y = y + dy;

                if pixel_x < window_width && pixel_y < window_height {
                    let index = pixel_y * window_width + pixel_x;
                    if index < self.buffer.len() {
                        self.buffer[index as usize] = color;
                    }
                }
            }
        }
    }
}
