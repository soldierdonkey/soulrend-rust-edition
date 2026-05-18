mod scene;
pub use self::scene::*;

// Adds terrain generation methods
mod terrain;
pub use self::terrain::*;

pub struct World {
    pub name: String,
    pub description: String,
    pub scenes: Vec<Scene>,
}
impl World {
    pub fn new() -> Self {
        Self {
            name: "Default World".to_string(),
            description: "A world with no description.".to_string(),
            scenes: Vec::new(),
        }
    }
    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }
}