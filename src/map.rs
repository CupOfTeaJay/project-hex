use bevy::{asset::LoadedFolder, prelude::*};

pub struct MapPlugin;

impl Plugin for MapPlugin {
        fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_light);
    }
}

fn load_tile_assets(asset_server: Res<AssetServer>) -> Handle<LoadedFolder> {
    return asset_server.load_folder("assets/tiles");
}

fn generateMap(mut commands: Commands,
               asset_server: Res<AssetServer>) 
{
    let tile_assets = load_tile_assets(asset_server);

    let test = tile_assets.into();
}

// fn spawn_tile(mut commands: Commands,
//               ass: Res<AssetServer>) 
// {
//     let tile0 = SceneBundle {
//         scene: ass.load("blueTile.glb#Scene0"),
//         transform: Transform::from_xyz(0.0, 0.0, 0.0),
//         ..Default::default()
//     };

//     let offset = 3.0_f32.sqrt();

//     let tile1 = SceneBundle {
//         scene: ass.load("yellowTile.glb#Scene0"),
//         transform: Transform::from_xyz(1.5, 0.0, -offset/2.0_f32),
//         ..Default::default()
//     };

//     let tile2 = SceneBundle {
//         scene: ass.load("greenTile.glb#Scene0"),
//         transform: Transform::from_xyz(0.0, 0.0, -offset),
//         ..Default::default()
//     };

//     // to position our 3d model, simply use the Transform
//     // in the SceneBundle
//     commands.spawn(tile0);
//     commands.spawn(tile1);
//     commands.spawn(tile2);
// }