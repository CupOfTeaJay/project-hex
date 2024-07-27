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

use crate::common::events::settle_event::SettleEvent;
use crate::common::states::game_state::GameState;

/// Callback function for the "End turn" button.
pub fn end_turn(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::OpponentTurn);
}

/// Callback function for the "Settle" button.
pub fn send_settle_event(mut settle_event: EventWriter<SettleEvent>) {
    settle_event.send(SettleEvent);
}
