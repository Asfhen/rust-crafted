use crate::{Block, ChunkMap, Player, CHUNK_HEIGHT, CHUNK_SIZE};
use bevy::{
    ecs::{
        component::Component, entity::Entity, query::{Changed, With}, resource::Resource, schedule::SystemSet, system::{Commands, Query, Res, ResMut}
    },
    math::{FloatOrd, IVec3},
    platform::collections::{HashMap, HashSet}, transform::components::GlobalTransform,
};
use ndshape::ConstShape3u32;

#[derive(Component)]
pub struct Chunk(pub IVec3);

const CHUNK_SIZE_U32: u32 = CHUNK_SIZE as u32;
const CHUNK_HEIGHT_U32: u32 = CHUNK_HEIGHT as u32;
pub type ChunkShape = ConstShape3u32<CHUNK_SIZE_U32, CHUNK_HEIGHT_U32, CHUNK_SIZE_U32>;

#[derive(Default, Resource)]
pub struct ChunkEntities(HashMap<IVec3, Entity>);

impl ChunkEntities {
    /// Returns the entity at the given position.
    pub fn entity(&self, pos: IVec3) -> Option<Entity> {
        self.0.get(&pos).copied()
    }

    /// Attaches the specified entity to the chunk data.
    pub fn attach_entity(&mut self, pos: IVec3, entity: Entity) {
        self.0.insert(pos, entity);
    }

    /// Detaches the specified entity on the chunk data.
    pub fn detach_entity(&mut self, pos: IVec3) -> Option<Entity> {
        self.0.remove(&pos)
    }

    /// Returns an iterator iterating over the loaded chunk keys
    pub fn iter_keys(&self) -> impl Iterator<Item = &IVec3> {
        self.0.keys()
    }

    /// Returns the number of chunks loaded.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Default, Resource)]
pub struct DirtyChunks(HashSet<IVec3>);

impl DirtyChunks {
    pub fn mark_dirty(&mut self, chunk: IVec3) {
        self.0.insert(chunk);
    }

    pub fn iter_dirty(&self) -> impl Iterator<Item = &IVec3> {
        self.0.iter()
    }

    pub fn num_dirty(&self) -> usize {
        self.0.len()
    }
}

#[derive(Resource)]
pub struct CurrentLocalPlayer {
    pub chunk_min: IVec3,
    pub world_pos: IVec3,
}

#[derive(Resource)]
pub struct ChunkLoadRadius {
    pub horizontal: i32,
    pub vertical: i32,
}

#[derive(Default, Resource)]
pub struct ChunkCommandQueue {
    create: Vec<IVec3>,
    destroy: Vec<IVec3>,
}

impl ChunkCommandQueue {
    pub fn queue_unload<'a>(&mut self, region: impl Iterator<Item = &'a IVec3>) {
        self.destroy.extend(region)
    }
}

/// Label for the stage housing the chunk loading systems.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, SystemSet)]
pub struct ChunkLoadingSet;

fn update_view_chunks(
    player_pos: Res<CurrentLocalPlayer>,
    chunk_entities: Res<ChunkEntities>,
    view_radius: Res<ChunkLoadRadius>,
    mut chunk_command_queue: ResMut<ChunkCommandQueue>,
) {
    for x in -view_radius.horizontal..view_radius.horizontal {
        for z in -view_radius.horizontal..view_radius.horizontal {
            for y in -view_radius.vertical..view_radius.vertical {
                if x.pow(2) + z.pow(2) >= view_radius.horizontal.pow(2) {
                    continue;
                }

                let chunk_key = player_pos.chunk_min
                    + IVec3::new(
                        (x * CHUNK_SIZE as i32).into(),
                        (y * CHUNK_HEIGHT as i32).into(),
                        (z * CHUNK_SIZE as i32).into(),
                    );

                if chunk_entities.entity(chunk_key).is_none() {
                    chunk_command_queue.create.push(chunk_key);
                }
            }
        }
    }

    for loaded_chunk in chunk_entities.iter_keys() {
        let delta: IVec3 = *loaded_chunk - player_pos.chunk_min;

        if delta.x.pow(2) + delta.z.pow(2)
            > (view_radius.horizontal.pow(2) * (CHUNK_SIZE as i32).pow(2)).into()
            || delta.y.pow(2) > (view_radius.vertical.pow(2) * (CHUNK_HEIGHT as i32).pow(2)).into()
        {
            chunk_command_queue.destroy.push(*loaded_chunk);
        }
    }

    chunk_command_queue.create.sort_unstable_by_key(|key| {
        FloatOrd(key.as_vec3().distance(player_pos.chunk_min.as_vec3()))
    });
}

pub fn create_chunks(
    mut chunk_command_queue: ResMut<ChunkCommandQueue>,
    mut chunk_entities: ResMut<ChunkEntities>,
    mut cmds: Commands,
) {
    chunk_command_queue
        .create
        .drain(..)
        .for_each(|request| chunk_entities.attach_entity(request, cmds.spawn(Chunk(request)).id()));
}

pub fn destroy_chunks(
    mut chunk_command_queue: ResMut<ChunkCommandQueue>,
    mut chunk_entities: ResMut<ChunkEntities>,
    mut chunks: ResMut<ChunkMap<Block, ChunkShape>>,
    mut cmds: Commands,
) {
    chunk_command_queue.destroy.drain(..).for_each(|command| {
        cmds.entity(chunk_entities.detach_entity(command).unwrap())
            .despawn();
        chunks.remove(command);
    });
}

pub fn clear_dirty(mut dirty_chunks: ResMut<DirtyChunks>) {
    dirty_chunks.0.clear();
}

/// Updates the current chunk position for the current player.
pub fn update_player_pos(
    player: Query<&GlobalTransform, (With<Player>, Changed<GlobalTransform>)>,
    mut chunk_pos: ResMut<CurrentLocalPlayer>,
) {
    if let Ok(ply) = player.single() {
        let player_coords = ply.translation().as_ivec3();
        let nearest_chunk_origin = !IVec3::splat((CHUNK_SIZE - 1) as i32) & player_coords;

        chunk_pos.world_pos = player_coords;

        if chunk_pos.chunk_min != nearest_chunk_origin {
            chunk_pos.chunk_min = nearest_chunk_origin;
        }
    }
}
