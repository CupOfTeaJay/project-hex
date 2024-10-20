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

// Find a random land tile on the map. Spawn a unit there.

use bevy::prelude::*;
use indexmap::IndexMap;
use rand::{thread_rng, Rng};

use crate::common::components::labels::Label;
use crate::common::components::movement::HexPos;
use crate::common::events::pickable_spawn_event::PickableSpawnEvent;
use crate::common::resources::asset_handles::AssetHandles;
use crate::common::states::game_state::GameState;
use crate::common::states::ui_state::UiState;
use crate::plugins::training::systems::build_unit;
use crate::plugins::map::components::terrain::Terrain;

pub fn init_player(
    assets: Res<AssetHandles>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    query: Query<(&HexPos, &Terrain)>,
    mut unit_spawn_event: EventWriter<PickableSpawnEvent>,
) {
    let mut trans_terr_map: IndexMap<&HexPos, &Terrain> = IndexMap::new();

    for (hex_pos, terrain) in query.iter() {
        trans_terr_map.insert(hex_pos, terrain);
    }

    let mut random_hex_pos: &HexPos = &HexPos::new(&0, &0, &0);
    let mut pos_found: bool = false;
    let mut random_index: usize;
    while !pos_found {
        random_index = thread_rng().gen_range(0..trans_terr_map.keys().len());
        match trans_terr_map.get_index(random_index).unwrap().1 {
            &Terrain::Coastal => (),
            &Terrain::Debug => (),
            &Terrain::Desert => pos_found = true,
            &Terrain::Grassland => pos_found = true,
            &Terrain::Ice => (),
            &Terrain::Mountain => (),
            &Terrain::Ocean => (),
            &Terrain::Snow => (),
            &Terrain::Steppe => pos_found = true,
            &Terrain::Tundra => (),
        }
        if pos_found {
            random_hex_pos = trans_terr_map.get_index(random_index).unwrap().0;
        }
    }

    let entity = commands
        .spawn(build_unit(&assets, &Label::Pilgrim, &random_hex_pos))
        .id();

    unit_spawn_event.send(PickableSpawnEvent::new(entity));

    next_game_state.set(GameState::PlayerTurn);
    next_ui_state.set(UiState::Hud);
}
