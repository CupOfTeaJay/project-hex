/* 
    This Life of Ours
    Copyright (C) 2024 Clevermeld LLC

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
const CAMERA_ZOOM_SPEED: f32 = 2.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, (zoom_camera, translate_camera));
    }
}

fn spawn_camera(
    mut commands: Commands
) {
    // Initialize a 3D camera.
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: PerspectiveProjection {
            fov: 45.0_f32.to_radians(),
            ..default()
        }.into(),
        ..default()
    };
    // Spawn the camera.
    commands.spawn(camera);
}

fn translate_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>
) {
    for motion in motion_evr.read() {
        // Get the camera's position at the time of the mouse motion event.
        let mut camera_pos = query.single_mut();
        // Translate the camera if the player is "dragging" the screen.
        if buttons.pressed(MouseButton::Left) {
            camera_pos.translation.x -= motion.delta.x*CAMERA_TRANSLATE_SPEED;
            camera_pos.translation.z -= motion.delta.y*CAMERA_TRANSLATE_SPEED;
        }
    }
}

fn zoom_camera(
    mut query: Query<&mut Projection, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>
) {
    for scroll in scroll_evr.read() {
        // Get the perspective projection at the time of the scroll event.
        // MAYBE: Can this be done without pattern matching? We're not using the orthographic projection.
        let Projection::Perspective(persp) = query.single_mut().into_inner() else {
        return;
        };
        // Zoom in or out by adjusting the field of view.
        match scroll.unit {
            MouseScrollUnit::Line => {
                if scroll.y > 0.0 {
                    persp.fov -= CAMERA_ZOOM_SPEED.to_radians();
                }
                else {
                    persp.fov += CAMERA_ZOOM_SPEED.to_radians();
                }
            }
            MouseScrollUnit::Pixel => {
                if scroll.y > 0.0 {
                    persp.fov -= CAMERA_ZOOM_SPEED.to_radians();
                }
                else {
                    persp.fov += CAMERA_ZOOM_SPEED.to_radians();
                }
            }
        }
    }
}

