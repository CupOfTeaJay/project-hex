use::bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
struct PlayerCamera;

impl Plugin for CameraPlugin 
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, zoom_perspective);
    }
}

fn zoom_perspective(mut query: Query<&mut Projection, With<PlayerCamera>>) 
{
    // assume perspective. do nothing if orthographic.
    let Projection::Perspective(persp) = query.single_mut().into_inner() else {
        return;
    };
    // // zoom in
    // persp.fov /= 20.0;
    // zoom out
    persp.fov += persp.fov*0.001;
}

fn spawn_camera(mut commands: Commands)
{
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: PerspectiveProjection {
            fov: 45.0_f32.to_radians(),
            ..default()
        }.into(),
        ..default()
    };

    commands.spawn((PlayerCamera, camera));
}