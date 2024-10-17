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

/// Player enumerations. Each enumeration should have an associated marker
/// component.
///
/// TODO: If something like Box<dyn Component> becomes possible in the future,
///       then I believe we can get rid of this.
#[derive(Component)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
    PlayerThree,
    PlayerFour,
    PlayerFive,
    PlayerSix,
    PlayerSeven,
    PlayerEight,
    PlayerNine,
    PlayerTen,
    PlayerEleven,
    PlayerTwelve,
}

/*
 * Marker components.
 */

/// Marker component for player one.
#[derive(Component)]
pub struct PlayerOneMarker;

/// Marker component for player two.
#[derive(Component)]
pub struct PlayerTwoMarker;

/// Marker component for player three.
#[derive(Component)]
pub struct PlayerThreeMarker;

/// Marker component for player four
#[derive(Component)]
pub struct PlayerFourMarker;

/// Marker component for player five.
#[derive(Component)]
pub struct PlayerFiveMarker;

/// Marker component for player six.
#[derive(Component)]
pub struct PlayerSixMarker;

/// Marker component for player seven.
#[derive(Component)]
pub struct PlayerSevenMarker;

/// Marker component for player eight.
#[derive(Component)]
pub struct PlayerEightMarker;

/// Marker component for player nine.
#[derive(Component)]
pub struct PlayerNineMarker;

/// Marker component for player ten.
#[derive(Component)]
pub struct PlayerTenMarker;

/// Marker component for player eleven.
#[derive(Component)]
pub struct PlayerElevenMarker;

/// Marker component for player twelve.
#[derive(Component)]
pub struct PlayerTwelveMarker;

