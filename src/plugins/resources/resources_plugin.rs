/*
    Project Hex
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

#[rustfmt::skip]
use crate::common::resources::{
    asset_handles::AssetHandles,
    city_names::CityNames,
    map_parameters::MapParameters,
    pickable_buffers::PickableBuffers,
    pickable_buffers::PickableBufferHelpers,
    placement_focus::PlacementFocus,
    pos_neighbors_map::PosNeighborsMap,
    selection_focus::SelectionFocus,
    traversability_maps::TraversabilityMaps,
};

/// Plugin that registers resources with the main application. Currently, the ResourcesPlugin:
///     - Registers the AssetHandles resource.
///     - Registers the MapParameters resource.
///     - Registers the PickableBuffers resource.
///     - Registers the PosNeighborsMap resource.
///     - Registers the TraversabilityMaps resource.
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        // Register resources with the main application.
        app.insert_resource(AssetHandles::new())
            .insert_resource(CityNames::new())
            .insert_resource(MapParameters::new())
            .insert_resource(PickableBuffers::new())
            .insert_resource(PickableBufferHelpers::new())
            .insert_resource(PlacementFocus::new())
            .insert_resource(PosNeighborsMap::new())
            .insert_resource(SelectionFocus::new())
            .insert_resource(TraversabilityMaps::new());
    }
}
