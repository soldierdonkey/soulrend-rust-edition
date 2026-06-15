use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Soulrend Rust Edition".to_owned(),
        fullscreen: true,
        window_resizable: true, // Optional: lets the user manage the window if they exit fullscreen
        ..Default::default()
    }
}