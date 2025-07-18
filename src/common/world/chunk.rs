use crate::{
    common::{
        components::player::Player, generation::noise::WorldNoise, world::{block::{BlockType, WorldBlock}, position::Position}
    },
    CHUNK_HEIGHT, CHUNK_SIZE,
};
use bevy::{platform::collections::HashMap, prelude::*};
use libnoise::prelude::*;

const TOTAL_BLOCKS: usize = CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE;

type ChunkCoords = (i32, i32);

#[derive(Component)]
pub struct Chunk {
    pub position: ChunkCoords,
    pub blocks: Vec<WorldBlock>,
}

#[derive(Component)]
pub struct ChunkNeedsMeshing;

#[derive(Resource)]
pub struct ChunkManager {
    pub loaded_chunks: HashMap<ChunkCoords, Entity>,
    pub render_distance: i8,
}

#[derive(Resource)]
pub struct ChunkGenerationState {
    pub pending_chunks: Vec<(i32, i32)>,
}

impl Chunk {
    /// Get block safely and return None if out of bounds
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&WorldBlock> {
        if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
            Some(&self.blocks[block_index(x, y, z)])
        } else {
            None
        }
    }

    /// Get mutable block reference
    pub fn get_block_mut(&mut self, x: usize, y: usize, z: usize) -> Option<&mut WorldBlock> {
        if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
            Some(&mut self.blocks[block_index(x, y, z)])
        } else {
            None
        }
    }
}

#[inline]
pub fn block_index(x: usize, y: usize, z: usize) -> usize {
    (x * CHUNK_HEIGHT + y) * CHUNK_SIZE + z
}

pub fn chunk_generation_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut state: ResMut<ChunkGenerationState>,
    noise: Res<WorldNoise>
) {
    if state.pending_chunks.is_empty() {
        if let Ok(player_transform) = player_query.single() {
            let player_position = player_transform.translation;
            let (chunk_x, _, chunk_z) = world_to_chunk_coords(player_position, CHUNK_SIZE as i32);
    
            // Load chunks around the player
            for x_offset in -chunk_manager.render_distance..=chunk_manager.render_distance {
                for z_offset in -chunk_manager.render_distance..=chunk_manager.render_distance {
                    let chunk_coords = (chunk_x + x_offset as i32, chunk_z + z_offset as i32);
                    
                    if !chunk_manager.loaded_chunks.contains_key(&chunk_coords) {
                        state.pending_chunks.push(chunk_coords);
                    }
                }
            }
        }
    }

    if let Some(chunk_coords) = state.pending_chunks.pop() {
        let entity = spawn_chunk(&mut commands, chunk_coords, &noise);
        chunk_manager.loaded_chunks.insert(chunk_coords, entity);
    }
}

fn spawn_chunk(commands: &mut Commands, position: ChunkCoords, noise: &WorldNoise) -> Entity {
    commands
        .spawn((
            Chunk {
                position,
                blocks: generate_terrain(position, noise),
            },
            ChunkNeedsMeshing,
            Name::new(format!(
                "chunk_{}_{}",
                position.0, position.1,
            )),
            Transform::from_translation(Vec3::new(
                position.0 as f32 * CHUNK_SIZE as f32,
                0.0,
                position.1 as f32 * CHUNK_SIZE as f32,
            )),
            GlobalTransform::default(),
            Visibility::default(),
        ))
        .id()
}

fn generate_terrain(position: ChunkCoords, noise: &WorldNoise) -> Vec<WorldBlock> {
    let mut blocks = Vec::<WorldBlock>::with_capacity(TOTAL_BLOCKS);

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let global_x = (position.0 * CHUNK_SIZE as i32) + x as i32;
            let global_z = (position.1 * CHUNK_SIZE as i32) + z as i32;

            let height = (noise.terrain.sample([global_x as f64 * 0.01, global_z as f64 * 0.01])
                * 64.0
                + 64.0) as usize;

            for y in 0..CHUNK_HEIGHT {
                let idx = block_index(x, y, z);

                blocks[idx] = WorldBlock {
                    block_type: if y == 0 {
                        Some(BlockType::new("rust_crafted", "bedrock", None))
                    } else if y < height.saturating_sub(3) {
                        Some(BlockType::new("rust_crafted", "stone", None))
                    } else if y < height {
                        Some(BlockType::new("rust_crafted", "dirt", None))
                    } else if y == height {
                        Some(BlockType::new("rust_crafted", "grass", None))
                    } else if y < 64 {
                        Some(BlockType::new("rust_crafted", "water", None))
                    } else {
                        None
                    },
                    position: Position::new(x as i32, y as i32, z as i32),
                    light_level: if y > height { 15 } else { 0 },
                };
            }
        }
    }

    blocks
}

fn world_to_chunk_coords(position: Vec3, chunk_size: i32) -> (i32, i32, i32) {
    (
        (position.x / chunk_size as f32).floor() as i32,
        (position.y / chunk_size as f32).floor() as i32,
        (position.z / chunk_size as f32).floor() as i32,
    )
}
