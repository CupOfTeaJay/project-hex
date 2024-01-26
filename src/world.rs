use::bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin 
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_tile));
    }
}

fn spawn_light(mut commands: Commands)
{
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

fn spawn_tile(mut commands: Commands,
              ass: Res<AssetServer>) 
{
    // note that we have to include the `Scene0` label
    let tile_asset = ass.load("Tile.glb#Scene0");

    let tile = SceneBundle {
        scene: tile_asset,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(tile);
}