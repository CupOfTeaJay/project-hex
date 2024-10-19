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

use crate::common::resources::players::Players;
use crate::plugins::player::components::Player;
use crate::plugins::player::components::PlayerEightMarker;
use crate::plugins::player::components::PlayerElevenMarker;
use crate::plugins::player::components::PlayerFiveMarker;
use crate::plugins::player::components::PlayerFourMarker;
use crate::plugins::player::components::PlayerNineMarker;
use crate::plugins::player::components::PlayerOneMarker;
use crate::plugins::player::components::PlayerSevenMarker;
use crate::plugins::player::components::PlayerSixMarker;
use crate::plugins::player::components::PlayerTenMarker;
use crate::plugins::player::components::PlayerThreeMarker;
use crate::plugins::player::components::PlayerTwelveMarker;
use crate::plugins::player::components::PlayerTwoMarker;

/// TODO:
pub fn spawn_players(mut commands: Commands, players: Res<Players>) {
    for player in players.buffer.iter() {
        match player {
            &Player::PlayerOne => {
                commands.spawn(PlayerOneMarker);
            }
            &Player::PlayerTwo => {
                commands.spawn(PlayerTwoMarker);
            }
            &Player::PlayerThree => {
                commands.spawn(PlayerThreeMarker);
            }
            &Player::PlayerFour => {
                commands.spawn(PlayerFourMarker);
            }
            &Player::PlayerFive => {
                commands.spawn(PlayerFiveMarker);
            }
            &Player::PlayerSix => {
                commands.spawn(PlayerSixMarker);
            }
            &Player::PlayerSeven => {
                commands.spawn(PlayerSevenMarker);
            }
            &Player::PlayerEight => {
                commands.spawn(PlayerEightMarker);
            }
            &Player::PlayerNine => {
                commands.spawn(PlayerNineMarker);
            }
            &Player::PlayerTen => {
                commands.spawn(PlayerTenMarker);
            }
            &Player::PlayerEleven => {
                commands.spawn(PlayerElevenMarker);
            }
            &Player::PlayerTwelve => {
                commands.spawn(PlayerTwelveMarker);
            }
        }
    }
}
