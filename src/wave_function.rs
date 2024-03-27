/*
 * wave_function.rs
 */

use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component)]
pub struct WaveFunction {
    domain: [(String, f32); 6] // Remember to adjust this if adding or removing tiles.
}

impl WaveFunction {
    pub fn new() -> Self {
        let domain_size = 6.0; // Remember to adjust this if adding or removing tiles.
        let uniform_prob = 1.0 / domain_size;
        WaveFunction {
            domain: [
                ("tiles/coastalTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/desertTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/grasslandTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/oceanTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/snowTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/steppeTile.glb#Scene0".to_string(), uniform_prob)
            ]
        }
    }
    pub fn collapse(&self) -> String {
        let mut rng = thread_rng();
        self.domain.choose_weighted(&mut rng, |item| item.1).unwrap().0.clone()
    }
}

