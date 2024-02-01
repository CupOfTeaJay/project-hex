use::bevy::input::mouse::*;
use::bevy::prelude::*;

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
    buttons: Res<Input<MouseButton>>
) {
    for motion in motion_evr.read() {
        // Get the camera's position at the time of the mouse motion event.
        let mut camera_pos = query.single_mut();
        // Translate the camera.
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
