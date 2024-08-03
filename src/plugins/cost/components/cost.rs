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

use crate::common::components::labels::Label;

#[derive(Component, Debug)]
pub struct Cost {
    pub fruit: Label,
    cumulative: u32,
    modifier: f32,
    principal: u32,
    spent: u32,
}

impl Cost {
    pub fn new(fruit: &Label, principal: &u32) -> Self {
        Cost {
            fruit: *fruit,
            cumulative: 0,
            modifier: 1.0,
            principal: *principal,
            spent: 0,
        }
    }
}
