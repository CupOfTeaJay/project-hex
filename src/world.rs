use::bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light);
    }
}

fn spawn_light(
    mut commands: Commands
) {
    // Initialize a point light.
    let light = PointLightBundle {
        point_light : PointLight {
            intensity : 2000.0,
            ..default()
        },
        transform : Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    };
    // Spawn the point light.
    commands.spawn(light);
}

