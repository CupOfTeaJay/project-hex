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

pub struct AStarNode {
    f_cost: u32,
    g_cost: u32,
    h_cost: u32,
}

impl AStarNode {
    pub fn new(g_cost: u32, h_cost: u32) -> Self {
        AStarNode {
            f_cost: g_cost + h_cost,
            g_cost: g_cost,
            h_cost: h_cost,
        }
    }
}
