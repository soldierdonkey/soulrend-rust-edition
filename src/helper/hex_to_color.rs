use macroquad::prelude::*;

pub fn hex_to_color(hex: u32) -> Color {
    Color::from_rgba(
        (((hex >> 16) & 0xFF) as f32 / 255.0) as u8, // Red
        (((hex >> 8) & 0xFF) as f32 / 255.0) as u8,  // Green
        ((hex & 0xFF) as f32 / 255.0) as u8,         // Blue
        255,                                 // Alpha
    )
}