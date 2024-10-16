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
use indexmap::IndexMap;

use crate::plugins::rnd::components::tech::Tech;

#[derive(Component)]
pub struct TechTable {
    table: IndexMap<Tech, bool>,
}

impl TechTable {
    pub fn new() -> Self {
        TechTable {
            table: IndexMap::from([
                // Prehistoric-Age technologies.
                (Tech::WRITING, false),
                (Tech::FIREMAKING, false),
                (Tech::IRRIGATION, false),
                (Tech::METALLURGY, false),
                (Tech::ANIMAL_HUSBANDRY, false),
            ]),
        }
    }
}
