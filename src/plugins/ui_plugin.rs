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

use crate::systems::camera_management::spawn_camera::spawn_camera;
use crate::systems::ui::button_router::button_router;
use crate::systems::ui::spawn_hud::spawn_hud;

/// Plugin that defines the game's user interface. Currently, the UIPlugin:
///     - TODO:
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // Add GameState::PlayerInit exit scheduled systems to the main
        // application.
        app.add_systems(
            OnExit(GameState::PlayerInit),
            spawn_hud
                .after(spawn_camera)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame))),
        );

        // Add Update scheduled systems to the main application.
        app.add_systems(
            Update,
            button_router
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame))),
        );
    }
}
