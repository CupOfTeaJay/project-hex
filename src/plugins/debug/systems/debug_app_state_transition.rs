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

pub fn debug_in_boot_entry() {
    println!("AppState entering state 'InBoot'.");
}

pub fn debug_in_boot_exit() {
    println!("AppState exiting state 'InBoot'.");
}

pub fn debug_in_game_entry() {
    println!("AppState entering state 'InGame'.");
}

pub fn debug_in_game_exit() {
    println!("AppState exiting state 'InGame'.");
}

pub fn debug_load_game_entry() {
    println!("AppState entering state 'LoadGame'.");
}

pub fn debug_load_game_exit() {
    println!("AppState exiting state 'LoadGame'.");
}

pub fn debug_main_menu_entry() {
    println!("AppState entering state 'MainMenu'.");
}

pub fn debug_main_menu_exit() {
    println!("AppState exiting state 'MainMenu'.");
}
