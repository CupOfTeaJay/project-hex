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
use bevy_mod_picking::DefaultPickingPlugins;

use project_hex::plugins::ai::ai_plugin::AiPlugin;
use project_hex::plugins::boot_plugin::BootPlugin;
use project_hex::plugins::camera_plugin::CameraPlugin;
use project_hex::plugins::debug_plugin::DebugPlugin;
use project_hex::plugins::events_plugin::EventsPlugin;
use project_hex::plugins::game_start_plugin::GameStartPlugin;
use project_hex::plugins::map_plugin::MapPlugin;
use project_hex::plugins::movement_plugin::MovementPlugin;
use project_hex::plugins::resources_plugin::ResourcesPlugin;
use project_hex::plugins::selection_plugin::SelectionPlugin;
use project_hex::plugins::stage_setting_plugin::StageSettingPlugin;
use project_hex::plugins::states_plugin::StatesPlugin;
use project_hex::plugins::ui::ui_plugin::UiPlugin;

fn main() {
    App::new()
        // External plugins.
        .add_plugins((DefaultPlugins, DefaultPickingPlugins))
        // Internal plugins.
        .add_plugins((
            AiPlugin,
            BootPlugin,
            CameraPlugin,
            DebugPlugin,
            GameStartPlugin,
            MapPlugin,
            MovementPlugin,
            EventsPlugin,
            ResourcesPlugin,
            SelectionPlugin,
            StatesPlugin,
            StageSettingPlugin,
            UiPlugin,
        ))
        // Execute execute execute!
        .run();
}
