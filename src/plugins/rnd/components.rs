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

#[derive(Component, Hash, Eq, PartialEq)]
pub enum Tech {
    // Prehistoric-Age technologies.
    AnimalHusbandry,
    Firemaking,
    Irrigation,
    Metallurgy,
    Writing,
}

#[derive(Component)]
pub struct TechTree {
    researched: IndexMap<Tech, bool>,
}

impl TechTree {
    pub fn new() -> Self {
        TechTree {
            researched: IndexMap::from([
                // Prehistoric-Age technologies.
                (Tech::Writing, false),
                (Tech::Firemaking, false),
                (Tech::Irrigation, false),
                (Tech::Metallurgy, false),
                (Tech::AnimalHusbandry, false),
            ]),
        }
    }
}
