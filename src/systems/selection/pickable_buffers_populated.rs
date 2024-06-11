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

use crate::{
    resources::pickable_buffers::PickableBuffers,
    states::pickable_buffers_state::PickableBuffersState,
};

pub fn pickable_buffers_populated(pickable_buffers: &ResMut<PickableBuffers>) -> bool {
    if pickable_buffers.scenes_not_instanced.len() > 0
        || pickable_buffers.scenes_not_ready.len() > 0
    {
        true
    } else {
        false
    }
}
