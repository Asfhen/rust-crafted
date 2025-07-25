use std::ops::Div;

use bevy::math::{IVec3, UVec3};
use ilattice::{extent::Extent, glam::UVec2};

use crate::{BiomeTerrainGenerator, Block, BlockBuffer, BlockMaterial, ChunkShape, Dirt, Grass, HeightMap, CHUNK_SIZE};

pub trait LayeredBiomeTerrainGenerator: BiomeTerrainGenerator {
    fn fill_strata(&self, layer: u32) -> Block {
        match layer {
            0..1 => Grass::into_block(),
            _ => Dirt::into_block(),
        }
    }

    fn num_layers(&self) -> u32 {
        8
    }

    fn place_decoration(
        &self,
        _key: IVec3,
        _pos: UVec3,
        _buffer: &mut BlockBuffer<Block, ChunkShape>,
    ) {

    }
}

impl<T: LayeredBiomeTerrainGenerator> BiomeTerrainGenerator for T {
    fn carve_terrain(
        &self,
        chunk_key: IVec3,
        heightmap: HeightMap<CHUNK_SIZE, CHUNK_SIZE>,
        buffer: &mut BlockBuffer<Block, ChunkShape>,
    ) {
        Extent::from_min_and_shape(UVec2::ZERO, UVec2::splat(CHUNK_SIZE as u32))
            .iter2()
            .for_each(|pos| {
                let height = heightmap.get(pos.into());

                if height.div(CHUNK_SIZE as u32) == (chunk_key.y as u32).div(CHUNK_SIZE as u32) {
                    let local_height = height.rem_euclid(CHUNK_SIZE as u32);

                    for h in 0..=self.num_layers() {
                        let remaining_height = local_height.checked_sub(h);

                        if let Some(uh) = remaining_height {
                            *buffer.block_at_mut([pos.x, uh, pos.y].into()) = self.fill_strata(h);
                        }
                    }
                }
            });
    }

    fn decorate_terrain(
        &self,
        chunk_key: IVec3,
        heightmap: HeightMap<CHUNK_SIZE, CHUNK_SIZE>,
        buffer: &mut BlockBuffer<Block, ChunkShape>
    ) {
        if chunk_key.y <= 96 {
            return;
        }

        Extent::from_min_and_shape(UVec2::ZERO, UVec2::splat(CHUNK_SIZE as u32))
            .iter2()
            .for_each(|pos| {
                let height = heightmap.get(pos.into());

                if height.div(CHUNK_SIZE as u32) == (chunk_key.y as u32).div(CHUNK_SIZE as u32) {
                    let local_height = height.rem_euclid(CHUNK_SIZE as u32);
                    self.place_decoration(chunk_key, [pos.x, local_height, pos.y].into(), buffer);
                }
            });
    }
}
