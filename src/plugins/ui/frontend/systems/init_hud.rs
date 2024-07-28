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

use crate::plugins::ui::frontend::bundles::buttons::EndTurnButton;
use crate::plugins::ui::frontend::bundles::nodes::HudBottomLeftWidgetNode;
use crate::plugins::ui::frontend::bundles::nodes::HudBottomRightWidgetNode;
use crate::plugins::ui::frontend::bundles::nodes::HudLeftPaneNode;
use crate::plugins::ui::frontend::bundles::nodes::HudRightPaneNode;
use crate::plugins::ui::frontend::bundles::nodes::HudRootNode;
use crate::plugins::ui::frontend::bundles::nodes::HudTopLeftWidgetNode;
use crate::plugins::ui::frontend::bundles::nodes::HudTopRightWidgetNode;
use crate::plugins::ui::frontend::bundles::texts::EndTurnText;

/// Initializes the HUD (Heads Up Display) at game start.
pub fn init_hud(mut commands: Commands) {
    // Root node. Encapsulates the entire screen.
    commands.spawn(HudRootNode::new()).with_children(|root| {
        // Left "pane" (leftmost vertical split of root).
        root.spawn(HudLeftPaneNode::new())
            .with_children(|left_pane| {
                // Top-left "widget".
                left_pane.spawn(HudTopLeftWidgetNode::new());
                // Bottom-left "widget".
                left_pane.spawn(HudBottomLeftWidgetNode::new());
            });
        // Right "pane" (rightmost vertical split of root).
        root.spawn(HudRightPaneNode::new())
            .with_children(|right_pane| {
                // Top-right "widget".
                right_pane.spawn(HudTopRightWidgetNode::new());
                // Bottom-right "widget".
                right_pane
                    .spawn(HudBottomRightWidgetNode::new())
                    .with_children(|bottom_right_widget| {
                        // "End turn" button.
                        bottom_right_widget
                            .spawn(EndTurnButton::new())
                            .with_children(|end_turn_button| {
                                // ""End turn" text.
                                end_turn_button.spawn(EndTurnText::new());
                            });
                    });
            });
    });
}
