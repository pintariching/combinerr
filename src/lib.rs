use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

mod camera;
mod field;
mod plant;
mod scene;

use camera::spawn_camera;
use field::{spawn_corn_field, Tile, TileAssets};
use plant::{grow_plants, Growth, Plant, PlantAssets, PlantType};
use scene::spawn_scene;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    AssetLoading,
    Next,
}

pub trait BuildApp {
    fn build(&mut self) -> &mut Self;
}

impl BuildApp for App {
    fn build(&mut self) -> &mut App {
        self.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
            .insert_resource(WindowDescriptor {
                height: WINDOW_HEIGHT,
                width: WINDOW_WIDTH,
                title: "combinerr".to_string(),
                resizable: false,
                ..default()
            })
            .add_plugins(DefaultPlugins)
            // Load assets
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Next)
                    .with_collection::<TileAssets>()
                    .with_collection::<PlantAssets>(),
            )
            .add_state(GameState::AssetLoading)
            // Inspector setup
            .add_plugin(WorldInspectorPlugin::new())
            .register_type::<Tile>()
            .register_type::<Plant>()
            .register_inspectable::<PlantType>()
            .register_inspectable::<Growth>()
            // Custom systems
            .add_startup_system(spawn_scene)
            .add_system_set(SystemSet::on_enter(GameState::Next).with_system(spawn_corn_field))
            //.add_system_set(SystemSet::on_enter(GameState::Next).with_system(grow_plants))
            // Camera
            //.add_startup_system(spawn_camera)
            .add_plugin(PlayerPlugin)
    }
}
