// states.rs
mod menu;
pub use self::menu::*;

mod player;
pub use self::player::*;

mod environment;
pub use self::environment::*;

mod movement;
pub use self::movement::*;

// The enum must be public, and its variants are automatically public
pub enum GameState {
    MainMenu,
    InstanceManager,
    InstanceCreator,
    InGame(InGameState), // This variant holds the actual game data
}

pub struct InGameState {
    pub player: Player,
    pub environment: Environment,
}

impl InGameState {
    // The constructor must be public too
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            environment: Environment::new(),
        }
    }

    pub fn update_and_draw(&mut self) {
        // Your logic here...
    }
}