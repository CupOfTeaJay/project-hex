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

pub fn debug_not_in_game_entry() {
    println!("GameState entering state 'NotInGame'.");
}

pub fn debug_not_in_game_exit() {
    println!("GameState exiting state 'NotInGame'.");
}

pub fn debug_map_gen_entry() {
    println!("GameState entering state 'MapGen'.");
}

pub fn debug_map_gen_exit() {
    println!("GameState exiting state 'MapGen'.");
}

pub fn debug_player_init_entry() {
    println!("GameState entering state 'PlayerInit'.");
}

pub fn debug_player_init_exit() {
    println!("GameState exiting state 'PlayerInit'.");
}

pub fn debug_player_turn_entry() {
    println!("GameState entering state 'PlayerTurn'.");
}

pub fn debug_player_turn_exit() {
    println!("GameState exiting state 'PlayerTurn'.");
}

pub fn debug_opponent_turn_entry() {
    println!("GameState entering state 'OpponentTurn'.");
}

pub fn debug_opponent_turn_exit() {
    println!("GameState exiting state 'OpponentTurn'.");
}
