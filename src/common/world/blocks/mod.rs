pub mod dirt;
pub use dirt::*;

use bevy::prelude::Plugin;
use crate::common::world::material::BlockMaterialRegistry;

pub struct BlockBaseMaterialsPlugin;
impl Plugin for BlockBaseMaterialsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut registry = app
            .world_mut()
            .get_resource_mut::<BlockMaterialRegistry>()
            .unwrap();

        registry.register::<Dirt>();
    }
}

