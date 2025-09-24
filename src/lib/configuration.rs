use crate::{
    custom_meshing::generate_chunk_mesh, data::chunk::BlockArray, prelude::WorldBlock
};
use bevy::prelude::*;
use std::{hash::Hash, sync::Arc};

pub type BlockLookupFn<I = u8> = Box<dyn FnMut(IVec3) -> WorldBlock<I> + Send + Sync>;
pub type BlockLookupDelegate<I = u8> = Box<dyn Fn(IVec3) -> BlockLookupFn<I> + Send + Sync>;

pub type TextureIndexMapperFn<I = u8> = Arc<dyn Fn(I) -> [u32; 3] + Send + Sync>;

pub type ChunkMeshingFn<I, UB> =
    Box<dyn FnMut(Arc<BlockArray<I>>, TextureIndexMapperFn<I>) -> (Mesh, Option<UB>) + Send + Sync>;
pub type ChunkMeshingDelegate<I, UB> =
    Option<Box<dyn Fn(IVec3) -> ChunkMeshingFn<I, UB> + Send + Sync>>;

#[derive(Default, PartialEq, Eq)]
pub enum ChunkDespawnStrategy {
    /// Despawn chunks that are either further than `spawning_distance` away from the camera
    /// or outside of the viewport
    #[default]
    FarAwayOrOutOfView,
    /// Only despawn chunks that are further than `spawning_distance` away from the camera
    FarAway,
}

#[derive(Default, PartialEq, Eq)]
pub enum ChunkSpawnStrategy {
    /// Spawn chunks that are within `spawning_distance` of the camera
    /// and also within the viewport
    #[default]
    NearOrInView,
    /// Spawn chunks that are within `spawning_distance` of the camera, regardless of whether
    /// they are in the viewport or not. Will only have an effect if the despawn strategy is
    /// `FarAway`. If this strategy is used a flood fill will be used to find unspawned chunks
    /// and therefore it might make sense to lower the `spawning_rays` option.
    Near,
}

pub trait BlockWorldConfig: Resource + Default + Clone {
    type MaterialIndex: Copy + Hash + PartialEq + Eq + Default + Send + Sync;

    type ChunkUserBundle: Bundle + Clone;

    fn spawning_distance(&self) -> u32 {
        10
    }

    fn min_despawn_distance(&self) -> u32 {
        1
    }

    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        ChunkDespawnStrategy::default()
    }

    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        ChunkSpawnStrategy::default()
    }

    fn max_spawn_per_frame(&self) -> usize {
        10000
    }

    fn spawning_rays(&self) -> usize {
        100
    }

    fn spawning_ray_margin(&self) -> u32 {
        25
    }

    fn debug_draw_chunks(&self) -> bool {
        false
    }

    fn texture_index_mapper(&self) -> TextureIndexMapperFn<Self::MaterialIndex> {
        Arc::new(|_| [0, 0, 0])
    }

    fn block_lookup_delegate(&self) -> BlockLookupDelegate<Self::MaterialIndex> {
        Box::new(|_| Box::new(|_| WorldBlock::default()))
    }

    fn chunk_meshing_delegate(
        &self,
    ) -> ChunkMeshingDelegate<Self::MaterialIndex, Self::ChunkUserBundle> {
        None
    }

    fn block_texture(&self) -> Option<(String, u32)> {
        None
    }

    fn init_custom_materials(&self) -> bool {
        false
    }

    fn init_root(&self, mut _commands: Commands, _root: Entity) {}
}

pub fn default_chunk_meshing_delegate<I: PartialEq + Copy, UB: Bundle>(
    pos: IVec3,
) -> ChunkMeshingFn<I, UB> {
    Box::new(
        move |blocks: Arc<BlockArray<I>>, texture_index_mapper: TextureIndexMapperFn<I>| {
            let mesh = generate_chunk_mesh(blocks, pos, texture_index_mapper);
            (mesh, None)
        },
    )
}

#[derive(Resource, Clone, Default)]
pub struct DefaultWorld;

impl DefaultWorld {}

impl BlockWorldConfig for DefaultWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    fn texture_index_mapper(&self) -> TextureIndexMapperFn<Self::MaterialIndex> {
        Arc::new(|mat| match mat {
            0 => [0, 0, 0],
            1 => [1, 1, 1],
            2 => [2, 2, 2],
            3 => [3, 3, 3],
            _ => [0, 0, 0],
        })
    }
}
