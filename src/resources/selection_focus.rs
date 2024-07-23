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

#[derive(Resource)]
pub struct SelectionFocus {
    pub focus: Option<Entity>,
    pub label: Label,
}

impl SelectionFocus {
    pub fn new() -> Self {
        SelectionFocus {
            focus: None,
            label: Label::Void,
        }
    }

    pub fn clear_focus(&mut self) {
        self.focus = None;
        self.label = Label::Void;
    }

    pub fn set_focus(&mut self, entity: &Entity, label: &Label) {
        self.focus = Some(*entity);
        self.label = *label;
    }
}
