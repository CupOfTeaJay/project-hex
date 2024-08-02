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
use crate::common::resources::selection_focus::SelectionFocus;
use crate::plugins::city::components::markers::CityMarker;
use crate::plugins::ui::frontend::bundles::buttons::CityNameButton;
use crate::plugins::ui::frontend::bundles::buttons::SettleButton;
use crate::plugins::ui::frontend::bundles::texts::CityNameText;
use crate::plugins::ui::frontend::bundles::texts::SettleText;
use crate::plugins::ui::frontend::components::markers::HudBottomRightWidgetContentMarker;

pub fn update_hud(
    bottom_right_widget_content: Query<Entity, With<HudBottomRightWidgetContentMarker>>,
    city_names: Query<&Name, With<CityMarker>>,
    mut commands: Commands,
    selection_focus: Res<SelectionFocus>,
) {
    if selection_focus.is_changed() {
        match selection_focus.label {
            // View to show when nothing is the selection focus.
            Label::Void => {
                if let Ok(brw) = bottom_right_widget_content.get_single() {
                    commands.entity(brw).despawn_descendants();
                }
            }
            // View to show when a pilgrim becomes the selection focus.
            Label::Pilgrim => {
                commands
                    .entity(bottom_right_widget_content.get_single().unwrap())
                    .despawn_descendants()
                    .with_children(|bottom_right_widget_content| {
                        bottom_right_widget_content
                            .spawn(SettleButton::new())
                            .with_children(|settle_button| {
                                settle_button.spawn(SettleText::new());
                            });
                    });
            }
            // View to show when a city becomes the selection focus.
            Label::City => {
                commands
                    .entity(bottom_right_widget_content.get_single().unwrap())
                    .despawn_descendants()
                    .with_children(|bottom_right_widget_content| {
                        bottom_right_widget_content
                            .spawn(CityNameButton::new())
                            .with_children(|settle_button| {
                                settle_button.spawn(CityNameText::new(
                                    city_names
                                        .get(selection_focus.subject.unwrap())
                                        .unwrap()
                                        .to_string(),
                                ));
                            });
                    });
            }
        }
    }
}
