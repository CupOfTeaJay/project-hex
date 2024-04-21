/*
    Such is Life
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

use bevy::input::mouse::*;
use bevy::prelude::*;

const CAMERA_ZOOM_SPEED: f32 = 2.0;

pub fn zoom_camera(
    mut query: Query<&mut Projection, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    for scroll in scroll_evr.read() {
        // Get the perspective projection at the time of the scroll event.
        // TODO: Can this be done without pattern matching? We're not using the orthographic projection.
        let Projection::Perspective(persp) = query.single_mut().into_inner() else {
            return;
        };
        // Zoom in or out by adjusting the field of view.
        match scroll.unit {
            MouseScrollUnit::Line => {
                if scroll.y > 0.0 {
                    persp.fov -= CAMERA_ZOOM_SPEED.to_radians();
                } else {
                    persp.fov += CAMERA_ZOOM_SPEED.to_radians();
                }
            }
            MouseScrollUnit::Pixel => {
                if scroll.y > 0.0 {
                    persp.fov -= CAMERA_ZOOM_SPEED.to_radians();
                } else {
                    persp.fov += CAMERA_ZOOM_SPEED.to_radians();
                }
            }
        }
    }
}
