use serde::Deserialize;

use super::attributes::Attributes;

#[derive(Debug, Clone, Deserialize)]
pub struct Stats {
    pub hp: f32,
    pub mp: f32,
    pub stamina : f32,
}
impl Stats {
    pub fn new() -> Self {
        Self {
            hp: 100.0,
            mp: 100.0,
            stamina: 100.0,
        }
    }
    pub fn update(&mut self, delta_time: f32, attributes: &Attributes) {
        // Regenerate stamina
        self.stamina += attributes.stamina_regen * delta_time;
        if self.stamina > attributes.max_stamina {
            self.stamina = attributes.max_stamina;
        }
    }
}