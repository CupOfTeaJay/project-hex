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

#[derive(Component)]
enum SeaUnitClass {
    Capital,      // Flagships, command and control.
    HeavyWarship, // Strongest naval class. For frontal assault and defense.
    Recon,        // Light navalcraft. Best for reconnaissance purposes.
    Submersive,   // Submarines, submersibles... Primary naval stealth class.
    Support,      // Hospital ships, minelayers, etc.
    Warship,      // Default naval class. Jack of all trades, master of none.
}
