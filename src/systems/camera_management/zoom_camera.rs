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

use bevy::input::mouse::*;
use bevy::prelude::*;

const CAMERA_ZOOM_SPEED_FACTOR: f32 = 0.92;

pub fn zoom_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    for scroll in scroll_evr.read() {
        let transform = query.single_mut().into_inner();

        // Zoom in or out by adjusting the field of view.
        match scroll.unit {
            MouseScrollUnit::Line => {
                if scroll.y > 0.0 {
                    transform.translation.y *= CAMERA_ZOOM_SPEED_FACTOR;
                    transform.translation.z *= CAMERA_ZOOM_SPEED_FACTOR.sqrt();
                } else {
                    transform.translation.y /= CAMERA_ZOOM_SPEED_FACTOR;
                    transform.translation.z /= CAMERA_ZOOM_SPEED_FACTOR.sqrt();
                }
            }
            MouseScrollUnit::Pixel => {
                if scroll.y > 0.0 {
                    transform.translation.y *= CAMERA_ZOOM_SPEED_FACTOR;
                    transform.translation.z *= CAMERA_ZOOM_SPEED_FACTOR.sqrt();
                } else {
                    transform.translation.y /= CAMERA_ZOOM_SPEED_FACTOR;
                    transform.translation.z /= CAMERA_ZOOM_SPEED_FACTOR.sqrt();
                }
            }
        }
    }
}
