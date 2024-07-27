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

use crate::common::states::pickable_buffers_state::PickableBuffersState;

#[rustfmt::skip]
use crate::common::states::{
    app_state::AppState,
    assets_state::AssetsState,
    boot_state::BootState,
    game_state::GameState,
};

#[rustfmt::skip]
use crate::plugins::selection::systems::{
    make_entity_pickable::make_entity_pickable,
    process_scenes_not_instanced::process_scenes_not_instanced,
    process_scenes_not_ready::process_scenes_not_ready,
};

/// Plugin that ensures relevant game assets are pickable by the player. Currently, the
/// SelectionPlugin:
///     - Makes tiles pickable upon spawning into the world.
///     - Makes units pickable upon spawning into the world.
pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        // Register states with the main application.
        app.add_systems(
            Update,
            (make_entity_pickable)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame)))
                .run_if(
                    in_state(PickableBuffersState::Populated)
                        .or_else(in_state(PickableBuffersState::Empty)),
                ),
        )
        .add_systems(
            Update,
            (process_scenes_not_instanced, process_scenes_not_ready)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame)))
                .run_if(in_state(PickableBuffersState::Populated)),
        );
    }
}
