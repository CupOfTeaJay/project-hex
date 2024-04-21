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
use rand::prelude::*;

use crate::components::wave_func::WaveFunc;

fn wave_func_collapse(
    query: Query<&WaveFunc>,
) {
    for wave_func in &query {
        let mut rng = thread_rng();
        wave_func.domain
            .choose_weighted(&mut rng, |item| item.1)
            .unwrap()
            .0
            .clone();
    }
}
