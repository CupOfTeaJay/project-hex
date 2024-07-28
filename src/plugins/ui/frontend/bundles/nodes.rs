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

use crate::plugins::ui::frontend::components::markers::HudBottomLeftWidgetMarker;
use crate::plugins::ui::frontend::components::markers::HudBottomRightWidgetContentMarker;
use crate::plugins::ui::frontend::components::markers::HudBottomRightWidgetMarker;
use crate::plugins::ui::frontend::components::markers::HudLeftPaneMarker;
use crate::plugins::ui::frontend::components::markers::HudRightBannerMarker;
use crate::plugins::ui::frontend::components::markers::HudRightPaneMarker;
use crate::plugins::ui::frontend::components::markers::HudRootMarker;
use crate::plugins::ui::frontend::components::markers::HudTopLeftWidgetMarker;
use crate::plugins::ui::frontend::components::markers::HudTopRightWidgetMarker;

#[derive(Bundle)]
pub struct HudBottomLeftWidget {
    marker: HudBottomLeftWidgetMarker,
    node: NodeBundle,
}

impl HudBottomLeftWidget {
    pub fn new() -> Self {
        HudBottomLeftWidget {
            marker: HudBottomLeftWidgetMarker,
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
pub struct HudRightBanner {
    marker: HudRightBannerMarker,
    node: NodeBundle,
    pickability: Pickable,
}

impl HudRightBanner {
    pub fn new() -> Self {
        HudRightBanner {
            marker: HudRightBannerMarker,
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    height: Val::Percent(75.0),
                    border: UiRect::all(Val::Px(5.0)),
                    align_self: AlignSelf::End,
                    ..default()
                },
                border_color: Color::srgb(0.2, 0.0, 0.8).into(),
                ..default()
            },
            pickability: Pickable::IGNORE,
        }
    }
}

#[derive(Bundle)]
pub struct HudBottomRightWidget {
    marker: HudBottomRightWidgetMarker,
    node: NodeBundle,
}

impl HudBottomRightWidget {
    pub fn new() -> Self {
        HudBottomRightWidget {
            marker: HudBottomRightWidgetMarker,
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(75.0),
                    height: Val::Percent(12.5),
                    border: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Row,
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
pub struct HudBottomRightWidgetContent {
    marker: HudBottomRightWidgetContentMarker,
    node: NodeBundle,
}

impl HudBottomRightWidgetContent {
    pub fn new() -> Self {
        HudBottomRightWidgetContent {
            marker: HudBottomRightWidgetContentMarker,
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(75.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                border_color: Color::srgb(1.0, 0.0, 0.0).into(),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct HudTopLeftWidget {
    marker: HudTopLeftWidgetMarker,
    node: NodeBundle,
}

impl HudTopLeftWidget {
    pub fn new() -> Self {
        HudTopLeftWidget {
            marker: HudTopLeftWidgetMarker,
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
pub struct HudTopRightWidget {
    marker: HudTopRightWidgetMarker,
    node: NodeBundle,
}

impl HudTopRightWidget {
    pub fn new() -> Self {
        HudTopRightWidget {
            marker: HudTopRightWidgetMarker,
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
pub struct HudLeftPane {
    marker: HudLeftPaneMarker,
    node: NodeBundle,
    pickability: Pickable,
}

impl HudLeftPane {
    pub fn new() -> Self {
        HudLeftPane {
            marker: HudLeftPaneMarker,
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
pub struct HudRightPane {
    marker: HudRightPaneMarker,
    node: NodeBundle,
    pickability: Pickable,
}

impl HudRightPane {
    pub fn new() -> Self {
        HudRightPane {
            marker: HudRightPaneMarker,
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(50.0),
                    // border: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Column,
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
pub struct HudRoot {
    marker: HudRootMarker,
    node: NodeBundle,
    pickability: Pickable,
}

impl HudRoot {
    pub fn new() -> Self {
        HudRoot {
            marker: HudRootMarker,
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
