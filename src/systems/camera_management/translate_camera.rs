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

const CAMERA_TRANSLATE_SPEED: f32 = 0.015;

pub fn translate_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    for motion in motion_evr.read() {
        // Get the camera's position at the time of the mouse motion event.
        let mut camera_pos = query.single_mut();
        // Translate the camera if the player is "dragging" the screen.
        if buttons.pressed(MouseButton::Left) {
            camera_pos.translation.x -= motion.delta.x * CAMERA_TRANSLATE_SPEED;
            camera_pos.translation.z -= motion.delta.y * CAMERA_TRANSLATE_SPEED;
        }
    }
}
