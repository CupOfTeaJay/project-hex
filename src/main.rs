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

// External plugins.
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;

// Internal plugins.
use project_hex::plugins::ai::ai_plugin::AiPlugin;
use project_hex::plugins::ambience::ambience_plugin::StageSettingPlugin;
use project_hex::plugins::boot::boot_plugin::BootPlugin;
use project_hex::plugins::camera::camera_plugin::CameraPlugin;
use project_hex::plugins::city::city_plugin::CityPlugin;
use project_hex::plugins::cost::cost_plugin::CostPlugin;
use project_hex::plugins::debug::debug_plugin::DebugPlugin;
use project_hex::plugins::events::events_plugin::EventsPlugin;
use project_hex::plugins::map::map_plugin::MapPlugin;
use project_hex::plugins::movement::movement_plugin::MovementPlugin;
use project_hex::plugins::names::names_plugin::NamesPlugin;
use project_hex::plugins::placement::placement_plugin::PlacementPlugin;
use project_hex::plugins::resources::resources_plugin::ResourcesPlugin;
use project_hex::plugins::selection::selection_plugin::SelectionPlugin;
use project_hex::plugins::start::start_plugin::GameStartPlugin;
use project_hex::plugins::states::states_plugin::StatesPlugin;
use project_hex::plugins::ui::ui_plugin::UiPlugin;

fn main() {
    App::new()
        // External plugins.
        .add_plugins((DefaultPlugins, DefaultPickingPlugins, BillboardPlugin))
        // Internal plugins.
        .add_plugins((
            AiPlugin,
            BootPlugin,
            CameraPlugin,
            CityPlugin,
            CostPlugin,
            DebugPlugin,
            EventsPlugin,
            GameStartPlugin,
            MapPlugin,
            MovementPlugin,
            NamesPlugin,
            PlacementPlugin,
            ResourcesPlugin,
            SelectionPlugin,
            StatesPlugin,
        ))
        .add_plugins((UiPlugin, StageSettingPlugin))
        // Execute execute execute!
        .run();
}
