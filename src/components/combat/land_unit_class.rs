/*
    Such is Life
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

#[derive(Component)]
pub enum LandUnitClass {
    Artillery,     // Far-ranged units that provide indirect fire, level fortifications.
    Cavalry,       // Strong land class. Bad without support from other classes.
    Command,       // Generals, officers, command and control.
    HeavyCavalry,  // Strongest land class. Bad without support from other classes.
    HeavyInfantry, // Stronger land class. For frontal assault and defense.
    Infantry,      // Default land class. Jack of all trades, master of none.
    Recon,         // Light land units. Best for reconniassance purposes.
    Support,       // Medics, engineers, etc.
    Suppressive,   // Ranged units that lower opponent's combat effectiveness.
}
