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

use crate::components::selection::label::Label;
use crate::resources::selection_focus::SelectionFocus;

pub fn settle(selection_focus: ResMut<SelectionFocus>) {
    if let Some(entity) = selection_focus.focus {
        match selection_focus.label {
            Label::Pilgrim => println!("Settled!"),
            _ => println!("Something's not right!"),
        }
    }
}
