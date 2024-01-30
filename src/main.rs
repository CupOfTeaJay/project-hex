use::bevy::prelude::*;

mod camera;
mod map;
mod world;

use crate::camera::CameraPlugin;
use crate::map::MapPlugin;
use crate::world::WorldPlugin;

fn main() 
{
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, MapPlugin, WorldPlugin))
        .run();
}
