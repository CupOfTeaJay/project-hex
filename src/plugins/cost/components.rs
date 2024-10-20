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

/// The development of every product in Project Hex (units, research, etc.)
/// revolves around this component. Every product has an `adjusted` cost
/// (`principal * modifier`) that represents the number of turns needed to
/// create said product. The cost is fulfilled when `spent == adjusted`.
#[derive(Component, Debug)]
pub struct Cost {
    pub adjusted: u32,
    pub modifier: f32,
    pub principal: u32,
    pub spent: u32,
}

impl Cost {
    /// Initializes a new `Cost` component.
    pub fn new(modifier: &u32, principal: &u32) -> Self {
        Cost {
            adjusted: modifier*principal,
            modifier: *modifier,
            principal: *principal,
            spent: 0,
        }
    }
}

/*
 * Marker components.
 */

/// Marker component for costs actively being fulfilled.
#[derive(Component)]
pub struct ActiveCostMarker;

/// Marker component for costs not actively being fulfilled.
#[derive(Component)]
pub struct PassiveCostMarker;

