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
use bevy_mod_picking::prelude::*;
use indexmap::IndexMap;
use rand::{thread_rng, Rng};

use crate::{
    components::selection::label::Label,
    components::{
        combat::unit_bundle::UnitBundle, common::hex_pos::HexPos, map_generation::terrain::Terrain,
    },
    events::pickable_spawn_event::PickableSpawnEvent,
    states::game_state::GameState,
    systems::selection::clear_selection_focus::clear_selection_focus,
    systems::selection::select_ancestor_only::select_ancestor_only,
    utils::coord_conversions::cube_to_cartesian,
};

pub fn init_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    query: Query<(&HexPos, &Terrain)>,
    mut unit_spawn_event: EventWriter<PickableSpawnEvent>,
) {
    let mut trans_terr_map: IndexMap<&HexPos, &Terrain> = IndexMap::new();

    for (hex_pos, terrain) in query.iter() {
        trans_terr_map.insert(hex_pos, terrain);
    }

    let mut random_hex_pos: &HexPos = &HexPos::new(0, 0, 0);
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

    let (x, y, z) = cube_to_cartesian(
        random_hex_pos.q as f32,
        random_hex_pos.r as f32,
        random_hex_pos.s as f32,
    );
    let mut my_transform = Transform::from_xyz(x, y, z);
    my_transform.scale = Vec3::new(0.5, 0.5, 0.5);
    let unit_model = SceneBundle {
        scene: asset_server.load("units/debug.glb#Scene0"),
        transform: my_transform,
        ..Default::default()
    };

    let entity = commands
        .spawn((
            Name::new("Unit"),
            Label::Pilgrim,
            UnitBundle::new(*random_hex_pos, unit_model),
            PickSelection { is_selected: false },
            On::<Pointer<Select>>::run(select_ancestor_only),
            // On::<Pointer<Deselect>>::run(clear_selection_focus.pipe(show_default_brw_view)),
        ))
        .id();

    unit_spawn_event.send(PickableSpawnEvent::new(entity));

    next_game_state.set(GameState::PlayerTurn);
}
