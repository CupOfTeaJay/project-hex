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
use crate::plugins::rnd::components::TechTree;

pub fn spawn_tech_trees(
    mut commands: Commands,
    players: Res<Players>,
    player_one_query: Query<Entity, With<PlayerOneMarker>>,
    player_two_query: Query<Entity, With<PlayerTwoMarker>>,
    player_three_query: Query<Entity, With<PlayerThreeMarker>>,
    player_four_query: Query<Entity, With<PlayerFourMarker>>,
    player_five_query: Query<Entity, With<PlayerFiveMarker>>,
    player_six_query: Query<Entity, With<PlayerSixMarker>>,
    player_seven_query: Query<Entity, With<PlayerSevenMarker>>,
    player_eight_query: Query<Entity, With<PlayerEightMarker>>,
    player_nine_query: Query<Entity, With<PlayerNineMarker>>,
    player_ten_query: Query<Entity, With<PlayerTenMarker>>,
    player_eleven_query: Query<Entity, With<PlayerElevenMarker>>,
    player_twelve_query: Query<Entity, With<PlayerTwelveMarker>>,
) {
    for player in players.buffer.iter() {
        match player.0 {
            Player::PlayerOne => {
                commands
                    .entity(player_one_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
            Player::PlayerTwo => {
                commands
                    .entity(player_two_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
            Player::PlayerThree => {
                commands
                    .entity(player_three_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
            Player::PlayerFour => {
                commands
                    .entity(player_four_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
            Player::PlayerFive => {
                commands
                    .entity(player_five_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
            Player::PlayerSix => {
                commands
                    .entity(player_six_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
            Player::PlayerSeven => {
                commands
                    .entity(player_seven_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
            Player::PlayerEight => {
                commands
                    .entity(player_eight_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
            Player::PlayerNine => {
                commands
                    .entity(player_nine_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
            Player::PlayerTen => {
                commands
                    .entity(player_ten_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
            Player::PlayerEleven => {
                commands
                    .entity(player_eleven_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
            Player::PlayerTwelve => {
                commands
                    .entity(player_twelve_query.get_single().unwrap())
                    .insert(TechTree::new());
            }
        }
    }
}
