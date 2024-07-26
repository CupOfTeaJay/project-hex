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
use crate::states::{
    app_state::AppState,
    assets_state::AssetsState,
    boot_state::BootState,
    game_state::GameState,
};

/// Plugin that defines the game's user interface. Currently, the UIPlugin:
///     - Null.
pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        // Add GameState::OpponentTurn entry scheduled systems to the main
        // application.
        app.add_systems(
            OnEnter(GameState::OpponentTurn),
            (|mut next_game_state: ResMut<NextState<GameState>>| {
                println!("AI thinking really hard!");
                println!("Phew! All done.");
                next_game_state.set(GameState::PlayerTurn);
            })
            .run_if(in_state(AppState::InGame))
            .run_if(in_state(AssetsState::Loaded))
            .run_if(in_state(BootState::NotInBoot))
            .run_if(not(in_state(GameState::NotInGame))),
        );
    }
}

