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

use crate::plugins::ui::hud::bundles::buttons::EndTurnButton;
use crate::plugins::ui::hud::bundles::buttons::RndButton;
use crate::plugins::ui::hud::bundles::nodes::HudBottomLeftWidget;
use crate::plugins::ui::hud::bundles::nodes::HudBottomRightWidget;
use crate::plugins::ui::hud::bundles::nodes::HudBottomRightWidgetContent;
use crate::plugins::ui::hud::bundles::nodes::HudLeftPane;
use crate::plugins::ui::hud::bundles::nodes::HudRightBanner;
use crate::plugins::ui::hud::bundles::nodes::HudRightPane;
use crate::plugins::ui::hud::bundles::nodes::HudRoot;
use crate::plugins::ui::hud::bundles::nodes::HudTopLeftWidget;
use crate::plugins::ui::hud::bundles::nodes::HudTopRightWidget;
use crate::plugins::ui::hud::bundles::texts::EndTurnText;
use crate::plugins::ui::hud::bundles::texts::RndText;

/// Initializes the HUD (Heads Up Display) at game start.
pub fn construct_hud(mut commands: Commands) {
    // Root node. Encapsulates the entire screen.
    commands.spawn(HudRoot::new()).with_children(|root| {
        // Left "pane" (leftmost vertical split of root).
        root.spawn(HudLeftPane::new()).with_children(|left_pane| {
            // Top-left "widget".
            left_pane
                .spawn(HudTopLeftWidget::new())
                .with_children(|top_right_widget| {
                    // "R&D" button.
                    top_right_widget
                        .spawn(RndButton::new())
                        .with_children(|rnd_button| {
                            // "R&D" text.
                            rnd_button.spawn(RndText::new());
                        });
                });
            // Bottom-left "widget".
            left_pane.spawn(HudBottomLeftWidget::new());
        });
        // Right "pane" (rightmost vertical split of root).
        root.spawn(HudRightPane::new()).with_children(|right_pane| {
            // Top-right "widget".
            right_pane.spawn(HudTopRightWidget::new());
            // Right "banner".
            right_pane.spawn(HudRightBanner::new());
            // Bottom-right "widget".
            right_pane
                .spawn(HudBottomRightWidget::new())
                .with_children(|bottom_right_widget| {
                    // Content node for bottom-right "widget".
                    bottom_right_widget.spawn(HudBottomRightWidgetContent::new());
                    // "End turn" button.
                    bottom_right_widget
                        .spawn(EndTurnButton::new())
                        .with_children(|end_turn_button| {
                            // "End turn" text.
                            end_turn_button.spawn(EndTurnText::new());
                        });
                });
        });
    });
}
