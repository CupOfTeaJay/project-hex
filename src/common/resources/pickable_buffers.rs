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

use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Component, Resource)]
pub struct PickableBuffers {
    pub scenes_not_instanced: HashSet<Entity>,
    pub scenes_not_ready: HashSet<Entity>,
}

impl PickableBuffers {
    pub fn new() -> Self {
        PickableBuffers {
            scenes_not_instanced: HashSet::new(),
            scenes_not_ready: HashSet::new(),
        }
    }
}

#[derive(Component, Resource)]
pub struct PickableBufferHelpers {
    pub scenes_instanced: Vec<Entity>,
    pub scenes_ready: Vec<Entity>,
}

impl PickableBufferHelpers {
    pub fn new() -> Self {
        PickableBufferHelpers {
            scenes_instanced: Vec::new(),
            scenes_ready: Vec::new(),
        }
    }
}
