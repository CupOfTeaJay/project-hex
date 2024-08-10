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

#[derive(Bundle)]
pub struct BuildMartialZoneText {
    text: TextBundle,
}

impl BuildMartialZoneText {
    pub fn new() -> Self {
        BuildMartialZoneText {
            text: TextBundle::from_section("Martial Zone", TextStyle { ..default() }),
        }
    }
}

#[derive(Bundle)]
pub struct CityNameText {
    text: TextBundle,
}

impl CityNameText {
    pub fn new(name: String) -> Self {
        CityNameText {
            text: TextBundle::from_section(name, TextStyle { ..default() }),
        }
    }
}

#[derive(Bundle)]
pub struct EndTurnText {
    text: TextBundle,
}

impl EndTurnText {
    pub fn new() -> Self {
        EndTurnText {
            text: TextBundle::from_section("End turn", TextStyle { ..default() }),
        }
    }
}

#[derive(Bundle)]
pub struct OpponentTurnText {
    text: TextBundle,
}

impl OpponentTurnText {
    pub fn new() -> Self {
        OpponentTurnText {
            text: TextBundle::from_section("Opponent turn", TextStyle { ..default() }),
        }
    }
}

#[derive(Bundle)]
pub struct TrainPilgrimText {
    text: TextBundle,
}

impl TrainPilgrimText {
    pub fn new() -> Self {
        TrainPilgrimText {
            text: TextBundle::from_section("Train pilgrim", TextStyle { ..default() }),
        }
    }
}

#[derive(Bundle)]
pub struct SettleText {
    text: TextBundle,
}

impl SettleText {
    pub fn new() -> Self {
        SettleText {
            text: TextBundle::from_section("Settle", TextStyle { ..default() }),
        }
    }
}
