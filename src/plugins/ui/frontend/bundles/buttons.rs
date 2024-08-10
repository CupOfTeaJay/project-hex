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
use bevy_mod_picking::prelude::*;

use crate::common::components::labels::Label;
use crate::common::events::train_unit_event::TrainUnitEvent;
use crate::common::resources::placement_focus::PlacementFocus;
use crate::common::resources::selection_focus::SelectionFocus;
use crate::common::states::placement_state::PlacementState;
use crate::plugins::ui::backend::systems::button_callbacks::end_turn;
use crate::plugins::ui::backend::systems::button_callbacks::send_settle_event;
use crate::plugins::ui::frontend::components::markers::HudEndTurnButtonMarker;
use crate::plugins::ui::frontend::components::markers::SettleButtonMarker;

#[derive(Bundle)]
pub struct BuildMartialZoneButton {
    button: ButtonBundle,
    callback: On<Pointer<Click>>,
}

impl BuildMartialZoneButton {
    pub fn new() -> Self {
        BuildMartialZoneButton {
            button: ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                border_color: Color::srgb(0.0, 1.0, 0.0).into(),
                ..default()
            },
            callback: On::<Pointer<Click>>::run(
                |mut next_placement_state: ResMut<NextState<PlacementState>>,
                 mut placement_focus: ResMut<PlacementFocus>,
                 selection_focus: Res<SelectionFocus>| {
                    placement_focus
                        .set_focus(&Label::MartialZone, &selection_focus.subject.unwrap());
                    next_placement_state.set(PlacementState::Active);
                },
            ),
        }
    }
}

#[derive(Bundle)]
pub struct CityNameButton {
    button: ButtonBundle,
    callback: On<Pointer<Click>>,
}

impl CityNameButton {
    pub fn new() -> Self {
        CityNameButton {
            button: ButtonBundle {
                style: Style {
                    width: Val::Percent(50.0),
                    height: Val::Percent(50.0),
                    border: UiRect::all(Val::Px(5.0)),
                    align_self: AlignSelf::End,
                    ..default()
                },
                border_color: Color::srgb(0.0, 1.0, 0.0).into(),
                ..default()
            },
            callback: On::<Pointer<Click>>::run(|| println!("'CityNameButton' clicked.")),
        }
    }
}

#[derive(Bundle)]
pub struct EndTurnButton {
    button: ButtonBundle,
    callback: On<Pointer<Click>>,
    marker: HudEndTurnButtonMarker,
}

impl EndTurnButton {
    pub fn new() -> Self {
        EndTurnButton {
            button: ButtonBundle {
                style: Style {
                    width: Val::Percent(36.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(5.0)),
                    align_self: AlignSelf::End,
                    ..default()
                },
                border_color: Color::srgb(0.0, 1.0, 0.0).into(),
                ..default()
            },
            callback: On::<Pointer<Click>>::run(end_turn),
            marker: HudEndTurnButtonMarker,
        }
    }
}

#[derive(Bundle)]
pub struct SettleButton {
    button: ButtonBundle,
    callback: On<Pointer<Click>>,
    marker: SettleButtonMarker,
}

impl SettleButton {
    pub fn new() -> Self {
        SettleButton {
            button: ButtonBundle {
                style: Style {
                    width: Val::Percent(30.0),
                    height: Val::Percent(40.0),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                border_color: Color::srgb(0.0, 1.0, 0.0).into(),
                ..default()
            },
            callback: On::<Pointer<Click>>::run(send_settle_event),
            marker: SettleButtonMarker,
        }
    }
}

#[derive(Bundle)]
pub struct TrainPilgrimButton {
    button: ButtonBundle,
    callback: On<Pointer<Click>>,
}

impl TrainPilgrimButton {
    pub fn new() -> Self {
        TrainPilgrimButton {
            button: ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                border_color: Color::srgb(0.0, 1.0, 0.0).into(),
                ..default()
            },
            callback: On::<Pointer<Click>>::run(
                |selection_focus: Res<SelectionFocus>,
                 mut train_unit_event: EventWriter<TrainUnitEvent>| {
                    if let Some(city) = selection_focus.subject {
                        match selection_focus.label {
                            Label::City => {
                                train_unit_event.send(TrainUnitEvent::new(&city, &Label::Pilgrim));
                            }
                            _ => {
                                panic!("Error: bad 'label' for 'TrainPilgrimButton'.");
                            }
                        }
                    } else {
                        panic!("Error: bad 'subject' for 'TrainPilgrimButton'.");
                    }
                },
            ),
        }
    }
}
