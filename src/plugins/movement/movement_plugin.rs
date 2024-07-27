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

use ::bevy::prelude::*;

#[rustfmt::skip]
use crate::common::states::{
    app_state::AppState,
    assets_state::AssetsState,
    boot_state::BootState,
    game_state::GameState,
    pickable_buffers_state::PickableBuffersState,
};

use crate::plugins::movement::systems::build_path::build_path;
use crate::plugins::movement::systems::move_unit::move_unit;
use crate::plugins::movement::systems::pathfind::pathfind;
use crate::plugins::movement::systems::send_movement_event::post_movement_event;

/// TODO
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (post_movement_event, pathfind, build_path, move_unit)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(AssetsState::Loaded))
                .run_if(in_state(BootState::NotInBoot))
                .run_if(not(in_state(GameState::NotInGame)))
                .run_if(
                    in_state(PickableBuffersState::Empty)
                        .or_else(in_state(PickableBuffersState::Populated)),
                ),
        );
    }
}
