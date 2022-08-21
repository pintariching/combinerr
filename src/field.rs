use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::plant::{Growth, Plant, PlantAssets, PlantType};

const FIELD_WIDTH: i32 = 6;
const FIELD_LENGTH: i32 = 10;

#[derive(Reflect, Component, Default, Bundle)]
#[reflect(Component)]
pub struct Tile {
    pub plant: Plant,
}

#[derive(AssetCollection)]
pub struct TileAssets {
    #[asset(path = "tile.glb#Scene0")]
    pub tile_scene: Handle<Scene>,
}

pub fn spawn_corn_field(
    mut commands: Commands,
    tile_assets: Res<TileAssets>,
    plant_assets: Res<PlantAssets>,
) {
    for w in 0..FIELD_WIDTH {
        for l in 0..FIELD_LENGTH {
            commands
                .spawn()
                .insert(Tile {
                    plant: Plant {
                        plant_type: PlantType::Corn,
                        growth: Growth::Sprout,
                        growth_timer: Timer::from_seconds(10.0, true),
                    },
                })
                .with_children(|parent| {
                    parent.spawn_bundle(SceneBundle {
                        scene: tile_assets.tile_scene.clone(),
                        transform: Transform::from_xyz(
                            (w - (FIELD_WIDTH / 2)) as f32,
                            0.,
                            (l - (FIELD_LENGTH / 2)) as f32,
                        )
                        .with_rotation(Quat::from_rotation_y(0.)),
                        ..default()
                    });
                    parent.spawn_bundle(SceneBundle {
                        scene: plant_assets.corn_sprout.clone(),
                        transform: Transform::from_xyz(
                            (w - (FIELD_WIDTH / 2)) as f32,
                            0.,
                            (l - (FIELD_LENGTH / 2)) as f32,
                        ),
                        ..default()
                    });
                });

            // commands
            //     .spawn()
            //     .insert(Tile {
            //         plant: Plant {
            //             plant_type: PlantType::Corn,
            //             growth: Growth::Sprout,
            //             growth_timer: Timer::from_seconds(10.0, true),
            //         },
            //     })
            //     .insert_bundle(SceneBundle {
            //         scene: tile_assets.tile_scene.clone(),
            //         transform: Transform::from_xyz(
            //             (w - (FIELD_WIDTH / 2)) as f32,
            //             0.,
            //             (l - (FIELD_LENGTH / 2)) as f32,
            //         )
            //         .with_rotation(Quat::from_rotation_y(0.)),
            //         ..default()
            //     })
            //     .insert_bundle(SceneBundle {
            //         scene: plant_assets.corn_sprout.clone(),
            //         transform: Transform::from_xyz(
            //             (w - (FIELD_WIDTH / 2)) as f32,
            //             0.,
            //             (l - (FIELD_LENGTH / 2)) as f32,
            //         ),
            //         ..default()
            //     });
        }
    }
}
