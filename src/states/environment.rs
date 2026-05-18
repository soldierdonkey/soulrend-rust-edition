// src/states/environment.rs
// import the World struct from world.rs
mod world;
pub use self::world::*;

mod tile;
pub use self::tile::*;

pub struct Environment {
    pub name: String,
    pub worlds: Vec<World>
}
impl Environment {
    pub fn new() -> Self {
        Self {
            name: "Default Environment".to_string(),
            worlds: Vec::new(),
        }
    }
}