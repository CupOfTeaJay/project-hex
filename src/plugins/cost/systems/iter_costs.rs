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

use crate::plugins::cost::components::cost::Cost;
use crate::plugins::cost::components::markers::ActiveCostMarker;
use crate::plugins::cost::components::markers::PassiveCostMarker;

pub fn incr_active_costs(mut active_costs: Query<&mut Cost, With<ActiveCostMarker>>) {
    for mut cost in active_costs.iter_mut() {
        cost.spent += 1;
    }
}

pub fn decr_passive_costs(mut passive_costs: Query<&mut Cost, With<PassiveCostMarker>>) {
    for mut cost in passive_costs.iter_mut() {
        if cost.spent != 0 {
            cost.spent -= 1;
        }
    }
}
