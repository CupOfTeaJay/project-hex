use::bevy::prelude::*;

mod camera;
mod world;

use crate::camera::CameraPlugin;
use crate::world::WorldPlugin;

fn main() 
{
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, WorldPlugin))
        .run();
}
