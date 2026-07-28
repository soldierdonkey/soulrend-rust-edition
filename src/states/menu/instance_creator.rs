use macroquad::prelude::*;
use crate::runtime::Runtime;
use crate::states::hills::generate_hills;
use crate::states::{GameState, InGameState, Item};
pub fn instance_creator(game: &mut Runtime) {
    draw_text("Press Space to create an instance!", 0.0, 40.0, 80.0, RED);
    draw_text("Press Escape to exit to the manager!", 0.0, 150.0, 80.0, RED);
    if is_key_pressed(KeyCode::Space) {
        // Initialize the inner state data when transitioning
        let mut in_game_state = InGameState::new();
        in_game_state.player.creature.world = 0; // Set to the first world
        in_game_state.player.creature.scene = 0; // Set to the first scene
        in_game_state.environment.worlds.push(crate::states::environment::World::new());
        in_game_state.environment.worlds[0].add_scene(crate::states::environment::Scene::new(
            "first_scene".to_string(),
            "First Scene".to_string(),
            "This is the first scene.".to_string(),
            (100, 50)
        ));
        in_game_state.player.inventory.armor[0] = Some(Item::new("test:iron_axe".to_string()));
        // println!("Created instance: {:?}", in_game_state.environment.worlds[0].scenes[0].tiles.tiles);
        game.current_state = GameState::InGame(in_game_state);
    }
    if is_key_pressed(KeyCode::Escape) {
        // Initialize the inner state data when transitioning
        game.current_state = GameState::InstanceManager;
    }
}