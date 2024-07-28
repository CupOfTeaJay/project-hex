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
use bevy_mod_picking::prelude::*;

use crate::common::bundles::constituents::City;
use crate::common::bundles::constituents::Unit;
use crate::common::components::labels::Label;
use crate::common::components::movement::HexPos;
use crate::common::resources::asset_handles::AssetHandles;
use crate::common::systems::utils::hexpos_to_vec3;
use crate::plugins::selection::systems::clear_selection_focus::clear_selection_focus;
use crate::plugins::selection::systems::set_selection_focus::set_city_focus;
use crate::plugins::selection::systems::set_selection_focus::set_pilgrim_focus;

/// Builds a new 'City' bundle given a 'label' that corresponds to a city.
pub fn city_builder(assets: &Res<AssetHandles>, label: &Label, position: &HexPos) -> City {
    match label {
        &Label::City => City::new(
            &SceneBundle {
                scene: assets.scenes.city_center.clone().unwrap(),
                transform: Transform::from_translation(hexpos_to_vec3(position)),
                ..Default::default()
            },
            On::<Pointer<Deselect>>::run(clear_selection_focus),
            On::<Pointer<Over>>::run(|| {}),
            On::<Pointer<Select>>::run(set_city_focus),
            position,
        ),
        _ => {
            panic!("Error: invalid 'label' passed to 'city_builder'.");
        }
    }
}

/// Builds a new 'Unit' bundle given a 'label' that corresponds to a unit.
pub fn unit_builder(assets: &Res<AssetHandles>, label: &Label, position: &HexPos) -> Unit {
    match label {
        &Label::Pilgrim => Unit::new(
            &SceneBundle {
                scene: assets.scenes.unit_unit.clone().unwrap(),
                transform: Transform::from_translation(hexpos_to_vec3(position)),
                ..Default::default()
            },
            On::<Pointer<Deselect>>::run(clear_selection_focus),
            On::<Pointer<Over>>::run(|| {}),
            On::<Pointer<Select>>::run(set_pilgrim_focus),
            position,
        ),
        _ => {
            panic!("Error: invalid 'label' passed to 'unit_builder'.");
        }
    }
}
