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
use crate::plugins::ui::frontend::systems::init_hud::init_hud;
use crate::plugins::ui::frontend::systems::view_toggles::toggle_end_turn_button_opponent_turn_view;
use crate::plugins::ui::frontend::systems::view_toggles::toggle_end_turn_button_player_turn_view;

// TODO: Decouple camera plugin.
use crate::plugins::camera::systems::spawn_camera::spawn_camera;

/// Plugin that defines the game's user interface. Currently, the UIPlugin:
///     - Initializes the player's HUD (Heads Up Display) at the start of the
///       game.
///     - Toggles HUD node views upon GameState transitions.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Add GameState::PlayerInit exit scheduled systems to the main
        // application.
        app.add_systems(
            OnExit(GameState::PlayerInit),
            init_hud
                // TODO: Decouple camera plugin?
                .after(spawn_camera)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame))),
        );
        // Add GameState::PlayerTurn exit scheduled systems to the main
        // application.
        app.add_systems(
            OnExit(GameState::PlayerTurn),
            toggle_end_turn_button_player_turn_view
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame))),
        );
        // Add GameState::OpponentTurn exit scheduled systems to the main
        // application.
        app.add_systems(
            OnExit(GameState::OpponentTurn),
            toggle_end_turn_button_opponent_turn_view
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame))),
        );
    }
}
