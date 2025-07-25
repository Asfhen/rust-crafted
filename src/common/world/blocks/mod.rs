pub mod bedrock;
pub use bedrock::*;

pub mod dirt;
pub use dirt::*;

pub mod grass;
pub use grass::*;

pub mod leaves;
pub use leaves::*;

pub mod stone;
pub use stone::*;

pub mod water;
pub use water::*;

pub mod wood;
pub use wood::*;

use bevy::prelude::Plugin;
use crate::common::world::material::BlockMaterialRegistry;

pub struct BlockBaseMaterialsPlugin;
impl Plugin for BlockBaseMaterialsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut registry = app
            .world_mut()
            .get_resource_mut::<BlockMaterialRegistry>()
            .unwrap();

        registry.register::<Bedrock>();
        registry.register::<Dirt>();
        registry.register::<Grass>();
        registry.register::<Leaves>();
        registry.register::<Stone>();
        registry.register::<Water>();
        registry.register::<Wood>();
    }
}

