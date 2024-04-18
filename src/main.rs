/*
    Such is Life
    Copyright (C) 2024 Clevermeld LLC

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

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

fn main() {
    App::new()
        // Default plugins.
        .add_plugins(DefaultPlugins)
        // Custom plugins.
        .add_plugins((CameraPlugin, MapPlugin, WorldPlugin))
        // "Editor"
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
