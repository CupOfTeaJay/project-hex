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

#[derive(Component, Clone, Copy)]
pub struct City {
    pub id: Entity,
}

impl City {
    pub fn new(id: &Entity) -> Self {
        City { id: *id }
    }
}

#[derive(Component, Clone)]
pub struct Zones {
    pub ids: Vec<Entity>,
}

impl Zones {
    pub fn new() -> Self {
        Zones { ids: Vec::new() }
    }
}
