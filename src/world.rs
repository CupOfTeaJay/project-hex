use::bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_tile));
    }
}

fn spawn_light(
    mut commands: Commands
) {
    let light = PointLightBundle {
        point_light : PointLight {
            intensity : 2000.0,
            ..default()
        },
        transform : Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };
    commands.spawn(light);
}

fn spawn_tile(
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    let tile = SceneBundle {
        scene: asset_server.load("tiles/greenTile.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };
    commands.spawn(tile);
}
