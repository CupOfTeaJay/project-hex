use bevy::prelude::*;

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
        fn build(&self, app: &mut App) {
        // app.add_systems(Startup, generate_map);
    }
}

#[derive(Component, Debug)]
struct HexPosition {
    file: u32,
    rank: u32
}
