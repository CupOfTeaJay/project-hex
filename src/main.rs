/*
    Such is Life
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
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

use such_is_life::plugins::camera_plugin::CameraPlugin;
use such_is_life::plugins::game_start_plugin::GameStartPlugin;
use such_is_life::plugins::map_plugin::MapPlugin;
use such_is_life::plugins::stage_setting_plugin::StageSettingPlugin;
use such_is_life::states::app_state::AppState;
use such_is_life::states::game_state::GameState;

fn main() {
    App::new()
        // Initialize states.
        .insert_state(AppState::InGame)
        .insert_state(GameState::MapGen)
        // Default, community plugins.
        .add_plugins((DefaultPlugins, DefaultPickingPlugins))
        // Custom plugins.
        .add_plugins((CameraPlugin, GameStartPlugin, StageSettingPlugin, MapPlugin))
        // "Editor"
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
