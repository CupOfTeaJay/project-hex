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

use crate::common::states::app_state::AppState;
use crate::common::states::assets_state::AssetsState;
use crate::common::states::boot_state::BootState;
use crate::common::states::game_state::GameState;
use crate::common::states::placement_state::PlacementState;

use crate::plugins::placement::systems::listen_for_placement_event::listen_for_placement_event;
use crate::plugins::placement::systems::place::place;

pub struct PlacementPlugin;

impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        // Add "Update" scheduled systems to the main application.
        app.add_systems(
            Update,
            (listen_for_placement_event, place)
                .chain()
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame)))
                .run_if(in_state(PlacementState::Active)),
        );
    }
}
