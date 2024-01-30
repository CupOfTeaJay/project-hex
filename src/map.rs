use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
        fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_map);
    }
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct HexIndex {
    file: u32,
    rank: u32
}

fn init_blue_tile(asset_server: &Res<AssetServer>, 
                   x_pos: f32,
                   z_pos: f32) -> SceneBundle{
    return SceneBundle {
        scene: asset_server.load("tiles/blueTile.glb#Scene0"),
        transform: Transform::from_xyz(x_pos, 0.0, z_pos),
        ..Default::default()
    };
}

fn init_green_tile(asset_server: &Res<AssetServer>, 
                   x_pos: f32,
                   z_pos: f32) -> SceneBundle{
    return SceneBundle {
        scene: asset_server.load("tiles/greenTile.glb#Scene0"),
        transform: Transform::from_xyz(x_pos, 0.0, z_pos),
        ..Default::default()
    };
}

fn init_white_tile(asset_server: &Res<AssetServer>, 
                   x_pos: f32,
                   z_pos: f32) -> SceneBundle{
    return SceneBundle {
        scene: asset_server.load("tiles/whiteTile.glb#Scene0"),
        transform: Transform::from_xyz(x_pos, 0.0, z_pos),
        ..Default::default()
    };
}

fn init_yellow_tile(asset_server: &Res<AssetServer>, 
                   x_pos: f32,
                   z_pos: f32) -> SceneBundle{
    return SceneBundle {
        scene: asset_server.load("tiles/yellowTile.glb#Scene0"),
        transform: Transform::from_xyz(x_pos, 0.0, z_pos),
        ..Default::default()
    };
}

fn generate_map(mut commands: Commands,
                asset_server: Res<AssetServer>) 
{

    // Special triangle offset.
    let offset = 3.0_f32.sqrt()/2.0_f32;

    let map_dims = 20.0;
    let range = map_dims - 1.0;

    // Starting tile position.
    let mut x = -0.75*map_dims + 0.75;
    let mut z = 0.0;

    for file in 0 ..= range as u32 {

        if file % 2 == 0 {
            z = -map_dims*offset;
        }
        else {
            z = -(map_dims - 1.0)*offset;
        }

        for rank in 0 ..= range as u32 {
            if rank % 2 == 0 {
                commands.spawn(
                    (Tile, init_white_tile(&asset_server, x, z), HexIndex {file: file, rank: rank})
                );
            }
            else if rank == file {
                commands.spawn(
                    (Tile, init_green_tile(&asset_server, x, z), HexIndex {file: file, rank: rank})
                );
            }
            else {
                commands.spawn(
                    (Tile, init_yellow_tile(&asset_server, x, z), HexIndex {file: file, rank: rank})
                );
            }
            z += 2.0*offset;
        }

        x += 1.5;
    }
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