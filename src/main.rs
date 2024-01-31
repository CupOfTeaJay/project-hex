use::bevy::prelude::*;
use::bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod world;

use crate::camera::CameraPlugin;
use crate::world::WorldPlugin;

fn main() 
{
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, WorldPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
