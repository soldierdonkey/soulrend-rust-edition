// states.rs
pub mod menu;
pub use self::menu::*;

mod player;
pub use self::player::*;

pub mod environment;
pub use self::environment::*;

pub mod in_game;
pub use self::in_game::*;

mod movement;
pub use self::movement::*;
// Import Items
pub mod items;
pub use self::items::*;


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
}