use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::field::Tile;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Plant {
    pub plant_type: PlantType,
    pub growth: Growth,
    pub growth_timer: Timer,
}

#[derive(Reflect, Inspectable, Default, Clone, PartialEq)]
pub enum Growth {
    #[default]
    Sprout,
    Seedling,
    SmallPlant,
    MediumPlant,
    AdultPlant,
    RipePlant,
}

impl Growth {
    fn grow(&mut self) -> Self {
        match self {
            Growth::Sprout => Growth::Seedling,
            Growth::Seedling => Growth::SmallPlant,
            Growth::SmallPlant => Growth::MediumPlant,
            Growth::MediumPlant => Growth::AdultPlant,
            Growth::AdultPlant => Growth::RipePlant,
            Growth::RipePlant => Growth::RipePlant,
        }
    }
}

#[derive(Reflect, Inspectable, Default, Clone)]
pub enum PlantType {
    #[default]
    Corn,
    Wheat,
}

#[derive(AssetCollection)]
pub struct PlantAssets {
    #[asset(path = "corn_sprout.glb#Scene0")]
    pub corn_sprout: Handle<Scene>,
    #[asset(path = "corn_seedling.glb#Scene0")]
    pub corn_seedling: Handle<Scene>,
    #[asset(path = "corn_small_plant.glb#Scene0")]
    pub corn_small_plant: Handle<Scene>,
    #[asset(path = "corn_medium_plant.glb#Scene0")]
    pub corn_medium_plant: Handle<Scene>,
    #[asset(path = "corn_large_plant.glb#Scene0")]
    pub corn_large_plant: Handle<Scene>,
}

pub fn grow_plants(
    mut commands: Commands,
    mut tiles: Query<&mut Tile>,
    time: Res<Time>,
    plant_assets: Res<PlantAssets>,
) {
    for mut tile in &mut tiles {
        tile.plant.growth_timer.tick(time.delta());
        if tile.plant.growth_timer.just_finished() {
            tile.plant.growth = tile.plant.growth.grow();

            if tile.plant.growth == Growth::Sprout {
                continue;
            }

            // match tile.plant.growth {
            //     Growth::Sprout => (),
            //     Growth::Seedling => todo!(),
            //     Growth::SmallPlant => todo!(),
            //     Growth::MediumPlant => todo!(),
            //     Growth::AdultPlant => todo!(),
            //     Growth::RipePlant => todo!(),
            // }
        }
    }
}
