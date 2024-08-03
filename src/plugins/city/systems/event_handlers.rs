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

use crate::common::components::labels::Label;
use crate::common::events::train_unit_event::TrainUnitEvent;
use crate::plugins::city::components::markers::CityMarker;
use crate::plugins::cost::components::cost::Cost;
use crate::plugins::cost::components::markers::ActiveCostMarker;
use crate::plugins::cost::components::markers::PassiveCostMarker;

pub fn handle_train_unit_event(
    active_costs: Query<(&Parent, &Cost), With<ActiveCostMarker>>,
    cities: Query<Entity, With<CityMarker>>,
    mut commands: Commands,
    passive_costs: Query<(Entity, &Parent, &Cost), With<PassiveCostMarker>>,
    mut train_unit_event: EventReader<TrainUnitEvent>,
) {
    for event in train_unit_event.read() {
        if let Ok(city) = cities.get(event.city) {
            match event.unit {
                Label::Pilgrim => {
                    train_pilgrim(&active_costs, &city, &mut commands, &passive_costs)
                }
                _ => {}
            }
        } else {
            panic!("Error: no 'city' passed to 'handle_train_unit_event'.");
        }
    }
}

fn train_pilgrim(
    active_costs: &Query<(&Parent, &Cost), With<ActiveCostMarker>>,
    city: &Entity,
    commands: &mut Commands,
    passive_costs: &Query<(Entity, &Parent, &Cost), With<PassiveCostMarker>>,
) {
    // Init conditionals.
    let mut pilgrim_actively_training: bool = false;
    let mut pilgrim_passively_training: Option<Entity> = None;

    // First and foremost, check to see if the city is already actively
    // training a pilgrim.
    for (parent, active_cost) in active_costs.iter() {
        if **parent == *city {
            match active_cost.fruit {
                Label::Pilgrim => {
                    pilgrim_actively_training = true;
                    break;
                }
                _ => {}
            }
        }
    }

    // If the city is not actively training a pilgrim...
    if !pilgrim_actively_training {
        // Check to see if a pilgrim is being passively trained.
        for (entity, parent, passive_cost) in passive_costs.iter() {
            if **parent == *city {
                match passive_cost.fruit {
                    Label::Pilgrim => {
                        pilgrim_passively_training = Some(entity);
                        break;
                    }
                    _ => {}
                }
            }
        }

        // If the city is passively training a pilgrim...
        if let Some(cost) = pilgrim_passively_training {
            // Transfer it from passive to active.
            commands
                .entity(cost)
                .remove::<PassiveCostMarker>()
                .insert(ActiveCostMarker);
        } else {
            // Actively train a new one.
            commands.entity(*city).with_children(|city| {
                city.spawn((Cost::new(&Label::Pilgrim, &5), ActiveCostMarker));
            });
            println!("Training pilgrim!");
        }
    } else {
        println!("Pilgrim already actively training.");
    }
}
