use crate::states::environment::SceneMap;
use noise::{Fbm, NoiseFn, OpenSimplex, MultiFractal};
use crate::states::Tile;
use crate::states::environment::tile::TileType;
// Layer: 0 - Terrain Generation
pub enum HillType {
    Plains,
    Hills
}
pub fn generate_hills(width: usize, height: usize, seed: u32) -> SceneMap {
    let mut fbm = Fbm::<OpenSimplex>::new(seed);
    fbm = fbm.set_octaves(4); // Layers of detail (4 is usually plenty for 2D)
    let mut heights = Vec::with_capacity(width);

    let frequency: f64; // Lower = wider hills
    let amplitude: f32; // Higher = taller hills
    let base_height: f32; // The average Y coordinate of the ground
    // --- TERRAIN TUNING PARAMETERS ---
    match HillType::Plains {
        HillType::Plains => {
            frequency = 0.09;
            amplitude = 10.0;
            base_height = 10.0;
        },
        HillType::Hills => {
            frequency = 0.2;
            amplitude = 40.0;
            base_height = 50.0;
        },
    }
    let noise = fbm.set_frequency(frequency); // Base frequency of the largest hills
    for x in 0..width {
        // 1. Sample the noise at (x, 0.0) to get a 1D slice.
        // We multiply x by frequency to stretch/squash the noise graph.
        let noise_val = noise.get([x as f64 * frequency, 0.0]);

        // 2. The noise crate returns values between -1.0 and 1.0. 
        // We multiply by our amplitude to make it taller, then add our base height.
        let y = base_height + (noise_val as f32 * amplitude);
        
        heights.push(y);
    }
    let mut output_map = SceneMap::new(width as u32, height as u32);
    for x in 0..width {
        for y in 0..height {
            if (y as f32) > heights[x] {
                // println!("grass! at ({}, {}) with height {}", x, y, heights[x]);
                output_map.tiles[y][x].update("test:grass".to_string());
            }
        }
    }
    println!("Generated hills with parameters: frequency={}, amplitude={}, base_height={}", frequency, amplitude, base_height);
    output_map
}