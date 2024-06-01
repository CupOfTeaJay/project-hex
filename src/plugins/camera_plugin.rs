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

#[rustfmt::skip]
use crate::systems::{
    camera_management::spawn_camera::spawn_camera,
    camera_management::translate_camera::translate_camera,
    camera_management::zoom_camera::zoom_camera,
};

/// Plugin that manages the player's camera. Currently, the CameraPlugin:
///     - Spawns the player's camera.
///     - Zooms the camera in response to player input.
///     - Translates the camera in response to player input.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // Add GameState::PlayerInit exit scheduled systems to the main application.
        app.add_systems(
            OnExit(GameState::PlayerInit),
            spawn_camera
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(in_state(GameState::PlayerTurn)),
        );
        // Add Update scheduled systems to the main application.
        app.add_systems(
            Update,
            (zoom_camera, translate_camera)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(in_state(GameState::OpponentTurn).or_else(in_state(GameState::PlayerTurn))),
        );
    }
}
