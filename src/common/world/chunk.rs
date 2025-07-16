use std::array;

use crate::{
    common::{
        components::player::Player,
        world::{block::{BlockType, WorldBlock}, position::Position},
    },
    CHUNK_HEIGHT, CHUNK_SIZE,
};
use bevy::{platform::collections::HashMap, prelude::*};
use libnoise::prelude::*;

const USIZE_CHUNK_SIZE: usize = CHUNK_SIZE as usize;
const USIZE_CHUNK_HEIGHT: usize = CHUNK_HEIGHT as usize;

type ChunkBlocksArray = [[[WorldBlock; USIZE_CHUNK_SIZE]; USIZE_CHUNK_HEIGHT]; USIZE_CHUNK_SIZE];
type ChunkCoords = (i32, i32);

#[derive(Component)]
pub struct Chunk {
    pub position: ChunkCoords, // (x, z) coordinates of the chunk for plane generation
    pub blocks: ChunkBlocksArray, // Array of 16x16x16 blocks
}

#[derive(Component)]
pub struct ChunkNeedsMeshing;

#[derive(Resource)]
pub struct ChunkManager {
    pub loaded_chunks: HashMap<ChunkCoords, Entity>,
    pub render_distance: i8,
}

pub fn chunk_generation_system(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.single() {
        let player_position = player_transform.translation;
        let (chunk_x, _, chunk_z) = world_to_chunk_coords(player_position, CHUNK_SIZE as i32);

        // Load chunks around the player
        for x_offset in -chunk_manager.render_distance..=chunk_manager.render_distance {
            for z_offset in -chunk_manager.render_distance..=chunk_manager.render_distance {
                let chunk_coords = (chunk_x + x_offset as i32, chunk_z + z_offset as i32);
                
                if !chunk_manager.loaded_chunks.contains_key(&chunk_coords) {
                    let entity = spawn_chunk(&mut commands, chunk_coords);
                    chunk_manager.loaded_chunks.insert(chunk_coords, entity);
                }
            }
        }
    }
}

fn spawn_chunk(commands: &mut Commands, position: ChunkCoords) -> Entity {
    commands
        .spawn((
            Chunk {
                position,
                blocks: generate_terrain(position),
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
            Visibility::default(),
        ))
        .id()
}

fn generate_terrain(position: ChunkCoords) -> ChunkBlocksArray {
    let perlin = Source::<2>::simplex(42);

    array::from_fn::<_, USIZE_CHUNK_SIZE, _>(|x| {
        array::from_fn::<_, USIZE_CHUNK_HEIGHT, _>(|y| {
            array::from_fn::<WorldBlock, USIZE_CHUNK_SIZE, _>(|z| {
                let global_x = (position.0 * CHUNK_SIZE as i32) + x as i32;
                let global_z = (position.1 * CHUNK_SIZE as i32) + z as i32;

                let height = (perlin.sample([global_x as f64 * 0.01, global_z as f64 * 0.01])
                    * 64.0
                    + 64.0) as usize;

                let block = if y == 0 {
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
                };

                WorldBlock {
                    block_type: block,
                    position: Position::new(
                        position.0,
                        height as i32,
                        position.1,
                    ),
                    light_level: if y > height { 15 } else { 0 },
                }
            })
        })
    })
}

fn world_to_chunk_coords(position: Vec3, chunk_size: i32) -> (i32, i32, i32) {
    (
        (position.x / chunk_size as f32).floor() as i32,
        (position.y / chunk_size as f32).floor() as i32,
        (position.z / chunk_size as f32).floor() as i32,
    )
}
