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
use crate::common::states::ui_state::UiState;
use crate::plugins::ui::hud::systems::construct_hud::construct_hud;
use crate::plugins::ui::hud::systems::destruct_hud::destruct_hud;
use crate::plugins::ui::hud::systems::update_hud::update_hud;
use crate::plugins::ui::hud::systems::view_toggles::toggle_end_turn_button_opponent_turn_view;
use crate::plugins::ui::hud::systems::view_toggles::toggle_end_turn_button_player_turn_view;
use crate::plugins::ui::rnd::systems::construct_rnd_landing::construct_rnd_landing;
use crate::plugins::ui::rnd::systems::destruct_rnd_landing::destruct_rnd_landing;
use crate::plugins::ui::rnd::systems::exit::exit_on_escape;

// TODO: Decouple camera plugin.
use crate::plugins::camera::systems::spawn_camera::spawn_camera;

/// Plugin that defines the game's user interface. Currently, the UIPlugin:
///     - Initializes the player's HUD (Heads Up Display) at the start of the
///       game.
///     - Toggles HUD node views upon GameState transitions.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Add UiState::Hud entry scheduled systems to the main
        // application.
        app.add_systems(
            OnEnter(UiState::Hud),
            construct_hud
                // TODO: Decouple camera plugin?
                .after(spawn_camera)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame))),
        );

        // Add UiState::Hud exit scheduled systems to the main
        // application.
        app.add_systems(
            OnExit(UiState::Hud),
            destruct_hud
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame))),
        );

        // Add UiState::Rnd entry scheduled systems to the main
        // application.
        app.add_systems(
            OnEnter(UiState::RndLanding),
            construct_rnd_landing
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame))),
        );

        // Add UiState::Rnd exit scheduled systems to the main
        // application.
        app.add_systems(
            OnExit(UiState::RndLanding),
            destruct_rnd_landing
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

        // Add Update scheduled systems to the main application.
        app.add_systems(
            Update,
            (
                // Update scheduled systems for UiState::Hud.
                update_hud
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(AssetsState::Loaded))
                    .run_if(in_state(BootState::NotInBoot))
                    .run_if(not(in_state(GameState::NotInGame)))
                    .run_if(in_state(UiState::Hud)),
                // Update scheduled systems for UiState::RndLanding.
                exit_on_escape
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(AssetsState::Loaded))
                    .run_if(in_state(BootState::NotInBoot))
                    .run_if(not(in_state(GameState::NotInGame)))
                    .run_if(in_state(UiState::RndLanding)),
            ),
        );
    }
}
