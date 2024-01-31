use::bevy::input::mouse::*;
use::bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, (zoom_perspective, translate_camera));
    }
}

/*
 * A PlayerCamera component to be associated with the player's camera.
 */
#[derive(Component)]
struct PlayerCamera;

/*
 * Initializes a Camera3dBundle object.
 */
fn init_camera_bundle() -> Camera3dBundle {
    return Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: PerspectiveProjection {
            fov: 45.0_f32.to_radians(),
            ..default()
        }.into(),
        ..default()
    };
}

/*
 * Spawns the player's camera.
 */
fn spawn_camera(mut commands: Commands) {
    commands.spawn((PlayerCamera, init_camera_bundle()));
}

/*
 * Adjusts a camera's position along its current (x,z) plane via clicking
 * and dragging the mouse.
 */
fn translate_camera(mut query: Query<&mut Transform, With<PlayerCamera>>,
                    mut motion_evr: EventReader<MouseMotion>,
                    buttons: Res<Input<MouseButton>>)
{
    for motion in motion_evr.read() 
    {
        // Get the camera's current position.
        let mut camera_pos = query.single_mut();

        // TODO: maybe avoid use of magic numbers here.
        if buttons.pressed(MouseButton::Left) {
            camera_pos.translation.x -= motion.delta.x*0.015;
            camera_pos.translation.z -= motion.delta.y*0.015;
        }
    }
}

/*
 * Adjusts a camera's field of view based on mouse scroll input.
 */
fn zoom_perspective(mut query: Query<&mut Camera3dBundle>,
                    mut scroll_evr: EventReader<MouseWheel>) 
{
    for scroll in scroll_evr.read() 
    {
        // Get the current perspective projection.
        // TODO: Can this be done without pattern matching? We're not using the orthographic projection.
        let Projection::Perspective(persp) = query.single_mut().into_inner() else {
        return;
        };

        // Zoom in or out depending on the scroll direction.
        match scroll.unit {
            MouseScrollUnit::Line => {
                if scroll.y > 0.0 {
                    persp.fov -= 2.0_f32.to_radians();
                }
                else {
                    persp.fov += 2.0_f32.to_radians();
                }
            }
            MouseScrollUnit::Pixel => {
                if scroll.y > 0.0 {
                    persp.fov -= 2.0_f32.to_radians();
                }
                else {
                    persp.fov += 2.0_f32.to_radians();
                }
            }
        }
    }
}