use crate::{Block, BlockBuffer, ChunkShape, HeightMap, CHUNK_SIZE};
use bevy::math::IVec3;

pub mod layered;
pub use layered::*;

pub mod plains;
pub use plains::*;

pub trait BiomeTerrainGenerator: 'static + Sync + Send {
    fn carve_terrain(
        &self,
        chunk_key: IVec3,
        heightmap: HeightMap<CHUNK_SIZE, CHUNK_SIZE>,
        buffer: &mut BlockBuffer<Block, ChunkShape>,
    );
    fn decorate_terrain(
        &self,
        chunk_key: IVec3,
        heightmap: HeightMap<CHUNK_SIZE, CHUNK_SIZE>,
        buffer: &mut BlockBuffer<Block, ChunkShape>
    );
}

pub trait IntoBoxedTerrainGenerator: BiomeTerrainGenerator + Sized {
    fn into_boxed_generator(self) -> Box<Self>;
}

impl<T: BiomeTerrainGenerator> IntoBoxedTerrainGenerator for T {
    fn into_boxed_generator(self) -> Box<Self> {
        Box::new(self)
    }
}
