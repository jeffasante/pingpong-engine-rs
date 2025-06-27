
// engine/config.rs

#[allow(dead_code)]
#[derive(Clone)]
pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
    pub paddle_speed: f32,
    pub ball_speed: f32,
    pub ball_radius: f32,
    pub paddle_width: f32,
    pub paddle_height: f32,
    pub ball_color: [f32; 4], // RGBA format
    pub paddle_color: [f32; 4], // RGBA format
    pub ball_size: f32,
    pub fps_target: u64,
    pub background_color: [f32; 4], // RGBA format
    pub font_path: String,
    pub font_size: u32,
    pub score_color: [f32; 4], // RGBA format
    pub score_position: [f32; 2], // [x, y] coordinates
    pub score_font_size: u32,
    pub score_spacing: f32,
    pub score_font_path: String,
    pub score_font_color: [f32; 4], // RGBA format
    pub score_font_outline_color: [f32; 4], // RGBA format
    pub score_font_outline_thickness: f32,
    pub score_font_outline_offset: [f32; 2], // [x, y] offset for outline
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            window_title: "Ping Pong".to_string(),
            paddle_speed: 300.0,
            ball_speed: 200.0,
            ball_radius: 10.0,
            paddle_width: 20.0,
            paddle_height: 100.0,
            ball_color: [1.0, 1.0, 1.0, 1.0], // White
            paddle_color: [1.0, 1.0, 1.0, 1.0], // White
            background_color: [0.0, 0.0, 0.0, 1.0], // Black
            font_path: "assets/fonts/Roboto-Regular.ttf".to_string(),
            font_size: 48,
            score_color: [1.0, 1.0, 1.0, 1.0], // White
            score_position: [400.0, 50.0], // Center top
            score_font_size: 48,
            score_spacing: 10.0,
            score_font_path: "assets/fonts/Roboto-Regular.ttf".to_string(),
            score_font_color: [1.0, 1.0, 1.0, 1.0], // White
            score_font_outline_color: [0.0, 0.0, 0.0, 1.0], // Black
            score_font_outline_thickness: 2.0,
            score_font_outline_offset: [1.0, 1.0],
            ball_size: 10.0,
            fps_target: 60,
        }
    }
}
