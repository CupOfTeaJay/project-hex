/*
    Such is Life
    Copyright (C) 2024 Clevermeld™ LLC

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

use crate::resources::map_parameters::MapParameters;
use crate::systems::map_generation::generate_map::generate_map;
use crate::systems::map_generation::make_tiles_pickable::make_tiles_pickable;

pub struct MapPlugin;

// TODO: why does make_tiles_pickable need to be in the update system???
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // Map settings set by user.
        let map_settings = MapParameters::new(106, 66, 0.20, 0.20, 0.20, 0.20, 0.20, 3.0, 5.0);
        // Insert resources into the app.
        app.insert_resource(map_settings);
        // Add startup scheduled systems to the app.
        app.add_systems(Startup, generate_map);
        // Add update scheduled systems to the app.
        app.add_systems(Update, make_tiles_pickable);
    }
}
