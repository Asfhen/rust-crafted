use crate::{Block, BlockBuffer, ChunkShape, CHUNK_SIZE};
use bevy::{
    app::Plugin,
    math::{FloatOrd, IVec3, Vec3Swizzles},
};
use once_cell::sync::Lazy;
use std::{collections::BTreeMap, sync::RwLock};

pub mod biomes;
pub use biomes::*;

pub mod common;
pub use common::*;

pub mod noise;
pub use noise::*;

pub static TERRAIN_GENERATOR: Lazy<RwLock<TerrainGenerator>> = Lazy::new(Default::default);

#[derive(Default)]
pub struct TerrainGenerator {
    biomes_map: BTreeMap<FloatOrd, Box<dyn BiomeTerrainGenerator>>,
}

impl TerrainGenerator {
    pub fn register_biome_generator(
        &mut self,
        chance: f32,
        biome: Box<dyn BiomeTerrainGenerator>,
    ) -> &mut Self {
        self.biomes_map.insert(FloatOrd(chance), biome);
        self
    }

    fn biome_at(&self, chunk_key: IVec3) -> &Box<dyn BiomeTerrainGenerator> {
        const BIOME_INVERSE_SCALE: f32 = 0.001;

        let coords = noise::voronoi(chunk_key.xzy().truncate().as_vec2() * BIOME_INVERSE_SCALE);
        let p = FloatOrd(noise::rand2to1i(coords));

        self.biomes_map
            .range(..=p)
            .last()
            .map_or(self.biomes_map.first_key_value().unwrap().1, |x| x.1)
    }

    pub fn generate(&self, chunk_key: IVec3, buffer: &mut BlockBuffer<Block, ChunkShape>) {
        let biome = self.biome_at(chunk_key);
        let noise = generate_height_map(chunk_key, CHUNK_SIZE);

        let noise_map = HeightMap::<CHUNK_SIZE, CHUNK_SIZE>::from_slice(&noise);

        common::carve_terrain_heightmap(buffer, chunk_key, &noise_map);

        biome.carve_terrain(chunk_key, noise_map, buffer);
        biome.decorate_terrain(chunk_key, noise_map, buffer);

        if chunk_key.y == 0 {
            terrain_generate_world_bottom_border(buffer);
        }
    }
}

pub struct TerrainGeneratorPlugin;
impl Plugin for TerrainGeneratorPlugin {
    fn build(&self, _app: &mut bevy::app::App) {
        TERRAIN_GENERATOR.write().unwrap().register_biome_generator(
            0.0f32,
            biomes::PlainsBiomeTerrainGenerator.into_boxed_generator(),
        );
    }
}
