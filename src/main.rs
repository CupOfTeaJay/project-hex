use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod hex_grid;
mod map;
mod tile;
mod wave_function;
mod world;

use camera::CameraPlugin;
use map::MapPlugin;
use world::WorldPlugin;

fn main() 
{
    App::new()
        // Default plugins.
        .add_plugins(DefaultPlugins)
        // Custom plugins.
        .add_plugins(
            (
                CameraPlugin,
                MapPlugin,
                WorldPlugin
            )
        )
        // Editor plugin.
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
