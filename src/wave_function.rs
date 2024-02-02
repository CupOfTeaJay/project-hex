/*
 * wave_function.rs
 */

use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component)]
pub struct WaveFunction {
    domain: [(String, f32); 4]
}

impl WaveFunction {
    pub fn new() -> Self {
        let domain_size = 4.0;
        let uniform_prob = 1.0 / domain_size;
        WaveFunction {
            domain: [
                ("tiles/blueTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/greenTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/whiteTile.glb#Scene0".to_string(), uniform_prob),
                ("tiles/yellowTile.glb#Scene0".to_string(), uniform_prob)
            ]
        }
    }
    pub fn collapse(&self) -> String {
        let mut rng = thread_rng();
        self.domain.choose_weighted(&mut rng, |item| item.1).unwrap().0.clone()
    }
}
