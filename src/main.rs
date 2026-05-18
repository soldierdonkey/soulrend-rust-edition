
// load Macroquad
use macroquad::prelude::*;

mod states;
use states::*;

mod helper;
use self::helper::*;

mod runtime;
use self::runtime::*;


// 1. Tell Rust to look for the sprites.rs file
mod sprites;


// Define your game's internal resolution (the logical resolution)
const VIRTUAL_WIDTH: f32 = 480.0;
const VIRTUAL_HEIGHT: f32 = 270.0;



#[macroquad::main("Soulrend 2")]
async fn main() {
    // Initialize the game
    let mut game = Runtime::new();

    // Create the off-screen render target
    let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
    
    // Ensure the texture filters cleanly for pixel art (no blurry linear scaling)
    render_target.texture.set_filter(FilterMode::Nearest);

    // 3. Populate the global registry right after Macroquad starts up (GPU context is ready)
    sprites::init();
    sprites::print_all();

    loop {
        // --- 1. DRAW GAME AT FIXED RESOLUTION ---
        // Switch rendering to our virtual canvas
        set_camera(&Camera2D {
            zoom: vec2(2.0 / VIRTUAL_WIDTH, -2.0 / VIRTUAL_HEIGHT),
            target: vec2(VIRTUAL_WIDTH / 2.0, VIRTUAL_HEIGHT / 2.0),
            render_target: Some(render_target.clone()),
            ..Default::default()
        });

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
            GameState::InGame(state) => {
                // Call the method defined in states.rs
                state.update_and_draw();
                draw_text("Press Escape to exit the instance!", 10.0, 10.0, 20.0, RED);
                if is_key_pressed(KeyCode::Escape) {
                    game.current_state = GameState::InstanceManager;
                }
            }
        }

        // --- 2. SCALE CANVAS TO FIT WINDOW ---
        // Switch back to the default window camera
        set_default_camera();

        clear_background(BLACK); // Black bars for letterboxing

        // Calculate aspect ratios to fit the window without stretching
        let window_width = screen_width();
        let window_height = screen_height();
        
        let scale = (window_width / VIRTUAL_WIDTH).min(window_height / VIRTUAL_HEIGHT);

        // Determine dimensions and centering offsets (Letterboxing)
        let draw_width = VIRTUAL_WIDTH * scale;
        let draw_height = VIRTUAL_HEIGHT * scale;
        let x_offset = (window_width - draw_width) / 2.0;
        let y_offset = (window_height - draw_height) / 2.0;

        // Draw the virtual canvas texture directly onto the screen
        draw_texture_ex(
            &render_target.texture,
            x_offset,
            y_offset,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_width, draw_height)),
                // This flips the OpenGL texture right-side up without breaking your coordinates!
                source: Some(Rect::new(0.0, VIRTUAL_HEIGHT, VIRTUAL_WIDTH, -VIRTUAL_HEIGHT)), 
                ..Default::default()
            },
        );

        next_frame().await
    }
}