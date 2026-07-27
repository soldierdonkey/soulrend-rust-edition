pub mod approach;
pub mod hex_to_color;
pub mod levenshtein;
pub mod rich_text;
pub mod direction;

use serde::{Deserialize, Deserializer, Serialize};

use macroquad::math::Vec2;

// 1. Define the remote structure matching Vec2
#[derive(Serialize, Deserialize)]
#[serde(remote = "Vec2")]
pub struct Vec2Def {
    x: f32,
    y: f32,
}

// 2. Add a helper function for the sequence (Vec)
pub fn deserialize_vec2_array<'de, D>(deserializer: D) -> Result<Vec<Vec2>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(with = "Vec2Def")] Vec2);

    let v = Vec::<Wrapper>::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(w)| w).collect())
}