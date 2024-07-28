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

// NOTE: Primary HUD flex box marker components.

/// Marker component for the root node of the heads up display (encapsulates
/// the entire screen).
#[derive(Component)]
pub struct HudRootMarker;

/// Marker component for the HUD's left pane (encapsulates the left side of the
/// screen).
#[derive(Component)]
pub struct HudLeftPaneMarker;

/// Marker component for the HUD's top-left widget (encapsulates top-left
/// corner of the screen).
#[derive(Component)]
pub struct HudTopLeftWidgetMarker;

/// Marker component for the HUD's bottom-left widget (encapsulates bottom-left
/// corner of the screen).
#[derive(Component)]
pub struct HudBottomLeftWidgetMarker;

/// Marker component for the HUD's right pane (encapsulates the left side of
/// the screen).
#[derive(Component)]
pub struct HudRightPaneMarker;

/// Marker component for the HUD's top-right widget (encapsulates top-right
/// corner of the screen).
#[derive(Component)]
pub struct HudTopRightWidgetMarker;

/// Marker component for the HUD's bottom-right widget (encapsulates bottom-
/// right corner of the screen).
#[derive(Component)]
pub struct HudBottomRightWidgetMarker;

/// FIXME:
#[derive(Component)]
pub struct HudBottomRightWidgetContentMarker;

/// FIXME:
#[derive(Component)]
pub struct SettleButtonMarker;

/// Marker component for the HUD's bottom-right banner (encapsulates far-right
/// side of the screen extending from the bottom-right widget).
#[derive(Component)]
pub struct HudRightBannerMarker;

// NOTE: Button marker components.

/// Marker component for the bottom-right widget's "End turn" button.
#[derive(Component)]
pub struct HudEndTurnButtonMarker;
