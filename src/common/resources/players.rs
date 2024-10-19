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

use crate::plugins::player::components::Player;

/// TODO: Document.
/// TODO: New should return empty buffer after implementing pre-game menu.
///       Players added in pre-game menu?
#[derive(Resource)]
pub struct Players {
    pub buffer: Vec<Player>,
}

impl Players {
    pub fn new() -> Self {
        Players {
            buffer: vec![
                Player::PlayerOne,
                Player::PlayerTwo,
                Player::PlayerThree,
                Player::PlayerFour,
                Player::PlayerFive,
                Player::PlayerSix,
                Player::PlayerSeven,
                Player::PlayerEight,
                Player::PlayerNine,
                Player::PlayerTen,
                Player::PlayerEleven,
                Player::PlayerTwelve,
            ],
        }
    }
}
