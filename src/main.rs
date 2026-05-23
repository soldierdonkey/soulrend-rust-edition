
// load Macroquad
use macroquad::prelude::*;

mod states;
use states::*;

mod helper;
use self::helper::*;

mod runtime;
use self::runtime::*;

mod render;

// 1. Tell Rust to look for the sprites.rs file
mod sprites;


// Define your game's internal resolution (the logical resolution)
const VIRTUAL_WIDTH: f32 = 1920.0;
const VIRTUAL_HEIGHT: f32 = 1080.0;
pub const LOGICAL_WIDTH: f32 = 1920.0;
pub const LOGICAL_HEIGHT: f32 = 1080.0;
const TILE_SIZE: f32 = 128.0;



#[macroquad::main("Soulrend Rust Edition")]
async fn main() {
    // Initialize the game
    let mut game = Runtime::new();

    // 1. Create the fixed 1080p canvas
    let render_target = render_target(LOGICAL_WIDTH as u32, LOGICAL_HEIGHT as u32);
    
    // Crucial: Set texture filtering to Nearest so your scaled images stay crispy!
    render_target.texture.set_filter(FilterMode::Nearest);

    // Create a camera mapped specifically to our render target dimensions
    let mut game_camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, LOGICAL_WIDTH, LOGICAL_HEIGHT));
    game_camera.render_target = Some(render_target.clone());

    // 3. Populate the global registry right after Macroquad starts up (GPU context is ready)
    sprites::init();
    sprites::print_all();
    
    loop {
        // --- 1. DRAW GAME AT FIXED RESOLUTION ---
        // Switch rendering to our virtual canvas
        set_camera(&game_camera);

        clear_background(Color::from_rgba(20, 20, 25, 255));
        
        //update mouse coordinates in runtime
        game.update();
        match &mut game.current_state {
            GameState::MainMenu => {
                main_menu(&mut game);
            }
            GameState::InstanceManager => {
                instance_manager(&mut game);
            }
            GameState::InstanceCreator => {
                instance_creator(&mut game);
            }
            GameState::InGame(in_game_state) => {
                in_game_state.in_game();
                // Call the method defined in states.rs
                draw_text("Press Escape to exit the instance!", 10.0, 40.0, 80.0, RED);
                if is_key_pressed(KeyCode::Escape) {
                    game.current_state = GameState::InstanceManager;
                }
            }
        }
        let fps = macroquad::time::get_fps();
        draw_text(&format!("FPS: {}", fps), 10.0, 230.0, 80.0, GREEN);
        // --- 2. SCALE CANVAS TO FIT WINDOW ---
        // Switch back to the default window camera
        set_default_camera();

        clear_background(BLACK); // Black bars for letterboxing
        
        // Calculate aspect ratio scaling bounds
        let scale = (screen_width() / LOGICAL_WIDTH).min(screen_height() / LOGICAL_HEIGHT);
        
        let dest_width = LOGICAL_WIDTH * scale;
        let dest_height = LOGICAL_HEIGHT * scale;
        
        // Center the 1080p canvas perfectly inside the physical window
        let dest_x = (screen_width() - dest_width) / 2.0;
        let dest_y = (screen_height() - dest_height) / 2.0;

        // Draw the finished virtual frame onto the physical monitor screen
        draw_texture_ex(
            &render_target.texture,
            dest_x,
            dest_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_width, dest_height)),
                // Flip the Y axis because render targets are natively inverted in OpenGL
                flip_y: true, 
                ..Default::default()
            },
        );

        next_frame().await
    }
}