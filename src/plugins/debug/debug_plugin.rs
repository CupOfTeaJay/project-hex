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

#[rustfmt::skip]
use crate::plugins::debug::systems::{
    debug_app_state_transition::*,
    debug_assets_state_transition::*,
    debug_boot_state_transition::*,
    debug_game_state_transition::*,
    debug_pickable_buffers_state_transition::*,
};

/// Plugin that is used for debugging purposes. Currently, the DebugPlugin
///     - Null.
pub struct DebugPlugin;

#[rustfmt::skip]
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // NOTE: Debug state transitions.

        // AppState
        // app.add_systems(OnEnter(AppState::InBoot), debug_in_boot_entry);
        // app.add_systems(OnExit(AppState::InBoot), debug_in_boot_exit);
        // app.add_systems(OnEnter(AppState::InGame), debug_in_game_entry);
        // app.add_systems(OnExit(AppState::InGame), debug_in_game_exit);
        // app.add_systems(OnEnter(AppState::LoadGame), debug_load_game_entry);
        // app.add_systems(OnExit(AppState::LoadGame), debug_load_game_exit);
        // app.add_systems(OnEnter(AppState::MainMenu), debug_main_menu_entry);
        // app.add_systems(OnExit(AppState::MainMenu), debug_main_menu_exit);

        // AssetsState
        // app.add_systems(OnEnter(AssetsState::Loaded), debug_loaded_entry);
        // app.add_systems(OnExit(AssetsState::Loaded), debug_loaded_exit);
        // app.add_systems(OnEnter(AssetsState::NotLoaded), debug_not_loaded_entry);
        // app.add_systems(OnExit(AssetsState::NotLoaded), debug_not_loaded_exit);

        // BootState
        // app.add_systems(OnEnter(BootState::LoadingAssets), debug_loading_assets_entry,);
        // app.add_systems(OnExit(BootState::LoadingAssets), debug_loading_assets_exit);
        // app.add_systems(OnEnter(BootState::NotInBoot), debug_not_in_boot_entry);
        // app.add_systems(OnExit(BootState::NotInBoot), debug_not_in_boot_exit);

        // GameState
        // app.add_systems(OnEnter(GameState::MapGen), debug_map_gen_entry);
        // app.add_systems(OnExit(GameState::MapGen), debug_map_gen_exit);
        // app.add_systems(OnEnter(GameState::NotInGame), debug_not_in_game_entry);
        // app.add_systems(OnExit(GameState::NotInGame), debug_not_in_game_exit);
        app.add_systems(OnEnter(GameState::OpponentTurn), debug_opponent_turn_entry);
        app.add_systems(OnExit(GameState::OpponentTurn), debug_opponent_turn_exit);
        // app.add_systems(OnEnter(GameState::PlayerInit), debug_player_init_entry);
        // app.add_systems(OnExit(GameState::PlayerInit), debug_player_init_exit);
        app.add_systems(OnEnter(GameState::PlayerTurn), debug_player_turn_entry);
        app.add_systems(OnExit(GameState::PlayerTurn), debug_player_turn_exit);

        // PickableBuffersState
        // app.add_systems(OnEnter(PickableBuffersState::Empty), debug_empty_entry);
        // app.add_systems(OnExit(PickableBuffersState::Empty), debug_empty_exit);
        // app.add_systems(OnEnter(PickableBuffersState::Populated), debug_populated_entry);
        // app.add_systems(OnExit(PickableBuffersState::Populated), debug_populated_exit);
    }
}
