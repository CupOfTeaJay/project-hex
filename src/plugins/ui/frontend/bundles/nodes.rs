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

use crate::plugins::ui::frontend::components::markers::HudBottomLeftWidget;
use crate::plugins::ui::frontend::components::markers::HudBottomRightWidget;
use crate::plugins::ui::frontend::components::markers::HudLeftPane;
use crate::plugins::ui::frontend::components::markers::HudRightPane;
use crate::plugins::ui::frontend::components::markers::HudRoot;
use crate::plugins::ui::frontend::components::markers::HudTopLeftWidget;
use crate::plugins::ui::frontend::components::markers::HudTopRightWidget;

#[derive(Bundle)]
pub struct HudBottomLeftWidgetNode {
    marker: HudBottomLeftWidget,
    node: NodeBundle,
}

impl HudBottomLeftWidgetNode {
    pub fn new() -> Self {
        HudBottomLeftWidgetNode {
            marker: HudBottomLeftWidget,
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(75.0),
                    height: Val::Percent(12.5),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                border_color: Color::srgb(0.2, 0.0, 0.8).into(),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct HudBottomRightWidgetNode {
    marker: HudBottomRightWidget,
    node: NodeBundle,
}

impl HudBottomRightWidgetNode {
    pub fn new() -> Self {
        HudBottomRightWidgetNode {
            marker: HudBottomRightWidget,
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(75.0),
                    height: Val::Percent(12.5),
                    border: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::End,
                    ..default()
                },
                border_color: Color::srgb(0.2, 0.0, 0.8).into(),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct HudTopLeftWidgetNode {
    marker: HudTopLeftWidget,
    node: NodeBundle,
}

impl HudTopLeftWidgetNode {
    pub fn new() -> Self {
        HudTopLeftWidgetNode {
            marker: HudTopLeftWidget,
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(75.0),
                    height: Val::Percent(12.5),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                border_color: Color::srgb(0.2, 0.0, 0.8).into(),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct HudTopRightWidgetNode {
    marker: HudTopRightWidget,
    node: NodeBundle,
}

impl HudTopRightWidgetNode {
    pub fn new() -> Self {
        HudTopRightWidgetNode {
            marker: HudTopRightWidget,
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(75.0),
                    height: Val::Percent(12.5),
                    border: UiRect::all(Val::Px(5.0)),
                    align_self: AlignSelf::End,
                    ..default()
                },
                border_color: Color::srgb(0.2, 0.0, 0.8).into(),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct HudLeftPaneNode {
    marker: HudLeftPane,
    node: NodeBundle,
    pickability: Pickable,
}

impl HudLeftPaneNode {
    pub fn new() -> Self {
        HudLeftPaneNode {
            marker: HudLeftPane,
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(50.0),
                    // border: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                // border_color: Color::srgb(0.6, 0.0, 0.4).into(),
                ..default()
            },
            pickability: Pickable::IGNORE,
        }
    }
}

#[derive(Bundle)]
pub struct HudRightPaneNode {
    marker: HudRightPane,
    node: NodeBundle,
    pickability: Pickable,
}

impl HudRightPaneNode {
    pub fn new() -> Self {
        HudRightPaneNode {
            marker: HudRightPane,
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(50.0),
                    // border: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                // border_color: Color::srgb(0.6, 0.0, 0.4).into(),
                ..default()
            },
            pickability: Pickable::IGNORE,
        }
    }
}

#[derive(Bundle)]
pub struct HudRootNode {
    marker: HudRoot,
    node: NodeBundle,
    pickability: Pickable,
}

impl HudRootNode {
    pub fn new() -> Self {
        HudRootNode {
            marker: HudRoot,
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                border_color: BorderColor(Color::srgb(1.00, 0.00, 0.00)),
                ..default()
            },
            pickability: Pickable::IGNORE,
        }
    }
}
