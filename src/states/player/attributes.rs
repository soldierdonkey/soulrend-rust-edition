pub struct Attributes {
    pub max_hp: f32,
    pub max_mp: f32,
    pub max_stamina: f32,
    pub stamina_regen: f32,
}
impl Attributes {
    pub fn new() -> Self {
        Self {
            max_hp: 100.0,
            max_mp: 100.0,
            max_stamina: 100.0,
            stamina_regen: 5.0, // Stamina regenerates at 5 points per second
        }
    }
}