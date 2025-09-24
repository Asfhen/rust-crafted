use crate::{
    custom_meshing::{CHUNK_SIZE_F, CHUNK_SIZE_I},
    data::{chunk::ChunkData, chunk_map::ChunkMap},
    prelude::{BlockWorldConfig, WorldBlock},
    traversal_alg::block_line_traversal,
    world::internals::{BlockWriteBuffer, ModifiedBlocks},
};
use bevy::{ecs::system::SystemParam, math::bounding::RayCast3d, prelude::*};
use std::{marker::PhantomData, sync::Arc};

#[derive(Component)]
pub struct BlockWorldCamera<C> {
    _marker: PhantomData<C>,
}

impl<C> Default for BlockWorldCamera<C> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

pub trait ChunkEventType {}

#[derive(Event)]
pub struct ChunkEvent<C, E: ChunkEventType> {
    pub chunk_key: IVec3,
    pub entity: Entity,
    _marker: (PhantomData<C>, PhantomData<E>),
}

impl<C, E: ChunkEventType> ChunkEvent<C, E> {
    pub fn new(chunk_key: IVec3, entity: Entity) -> Self {
        Self {
            chunk_key,
            entity,
            _marker: (PhantomData, PhantomData),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            chunk_key: self.chunk_key,
            entity: self.entity,
            _marker: (PhantomData, PhantomData),
        }
    }
}

/// Fired when a chunk is about to be despawned.
pub type ChunkWillDespawn<C> = ChunkEvent<C, WillDespawn>;
pub struct WillDespawn;
impl ChunkEventType for WillDespawn {}

/// Fired when a chunk is about to be spawned.
pub type ChunkWillSpawn<C> = ChunkEvent<C, WillSpawn>;
pub struct WillSpawn;
impl ChunkEventType for WillSpawn {}

/// Fired when a chunk is about to be remeshed.
pub type ChunkWillRemesh<C> = ChunkEvent<C, WillRemesh>;
pub struct WillRemesh;
impl ChunkEventType for WillRemesh {}

/// Fired when a chunk is about to be updated, typically when `set_voxel` was called on a voxel
/// within the chunk.
pub type ChunkWillUpdate<C> = ChunkEvent<C, WillUpdate>;
pub struct WillUpdate;
impl ChunkEventType for WillUpdate {}

pub trait FilterFn<I> {
    fn call(&self, input: (Vec3, WorldBlock<I>)) -> bool;
}

impl<F: Fn((Vec3, WorldBlock<I>)) -> bool, I> FilterFn<I> for F {
    fn call(&self, input: (Vec3, WorldBlock<I>)) -> bool {
        self(input)
    }
}

pub type RaycastFn<I> =
    dyn Fn(Ray3d, &dyn FilterFn<I>) -> Option<BlockRaycastResult<I>> + Send + Sync;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct BlockRaycastResult<I = u8> {
    pub position: Vec3,
    pub normal: Option<Vec3>,
    pub block: WorldBlock<I>,
}

impl<I> BlockRaycastResult<I> {
    pub fn block_pos(&self) -> IVec3 {
        self.position.floor().as_ivec3()
    }

    pub fn block_normal(&self) -> Option<IVec3> {
        self.normal.map(|n| n.floor().as_ivec3())
    }
}

#[derive(SystemParam)]
pub struct BlockWorld<'w, C: BlockWorldConfig> {
    chunk_map: Res<'w, ChunkMap<C, <C as BlockWorldConfig>::MaterialIndex>>,
    modified_blocks: Res<'w, ModifiedBlocks<C, <C as BlockWorldConfig>::MaterialIndex>>,
    block_write_buffer: ResMut<'w, BlockWriteBuffer<C, <C as BlockWorldConfig>::MaterialIndex>>,
    #[allow(unused)]
    configuration: Res<'w, C>,
}

impl<C: BlockWorldConfig> BlockWorld<'_, C> {
    pub fn get_block(&self, position: IVec3) -> WorldBlock<C::MaterialIndex> {
        self.get_block_fn()(position)
    }

    pub fn set_block(&mut self, position: IVec3, block: WorldBlock<C::MaterialIndex>) {
        self.block_write_buffer.push((position, block));
    }

    pub fn get_block_fn(&self) -> Arc<dyn Fn(IVec3) -> WorldBlock<C::MaterialIndex> + Send + Sync> {
        let chunk_map = self.chunk_map.get_map();
        let write_buffer = self.block_write_buffer.clone();
        let modified_blocks = self.modified_blocks.clone();

        Arc::new(move |position| {
            let (chunk_pos, local_pos) = get_chunk_block_position(position);

            if let Some(block) = write_buffer
                .iter()
                .find(|(pos, _)| *pos == position)
                .map(|(_, block)| *block)
            {
                return block;
            }

            {
                if let Some(block) = modified_blocks.get_block(&position) {
                    return block;
                }
            }

            let chunk_opt = {
                let chunk_map_read = chunk_map.read().unwrap();
                chunk_map_read.get(&chunk_pos).cloned()
            };

            if let Some(chunk_data) = chunk_opt {
                chunk_data.get_block(local_pos)
            } else {
                WorldBlock::Unset
            }
        })
    }

    pub fn get_chunk_data(&self, chunk_pos: IVec3) -> Option<ChunkData<C::MaterialIndex>> {
        self.chunk_map
            .get_map()
            .read()
            .unwrap()
            .get(&chunk_pos)
            .cloned()
    }

    pub fn get_chunk_data_fn(
        &self,
    ) -> Arc<dyn Fn(IVec3) -> Option<ChunkData<C::MaterialIndex>> + Send + Sync> {
        let chunk_map = self.chunk_map.get_map();
        Arc::new(move |chunk_pos| chunk_map.read().unwrap().get(&chunk_pos).cloned())
    }

    pub fn raycast(
        &self,
        ray: Ray3d,
        filter: &impl Fn((Vec3, WorldBlock<C::MaterialIndex>)) -> bool,
    ) -> Option<BlockRaycastResult<C::MaterialIndex>> {
        let raycast_fn = self.raycast_fn();
        raycast_fn(ray, filter)
    }

    pub fn raycast_fn(&self) -> Arc<RaycastFn<C::MaterialIndex>> {
        let chunk_map = self.chunk_map.get_map();
        let get_block = self.get_block_fn();

        Arc::new(move |ray, filter| {
            let p = ray.origin;
            let d = ray.direction;

            let loaded_aabb =
                ChunkMap::<C, C::MaterialIndex>::get_world_bounds(&chunk_map.read().unwrap());
            let trace_start = if p.cmplt(loaded_aabb.min.into()).any() {
                if let Some(trace_start_t) =
                    RayCast3d::from_ray(ray, f32::MAX).aabb_intersection_at(&loaded_aabb)
                {
                    ray.get_point(trace_start_t)
                } else {
                    return None;
                }
            } else {
                p
            };

            let trace_end_origin =
                trace_start + d * loaded_aabb.min.distance_squared(loaded_aabb.max);
            let trace_end_t = RayCast3d::new(trace_end_origin, -ray.direction, f32::MAX)
                .aabb_intersection_at(&loaded_aabb)
                .unwrap();
            let trace_end = Ray3d::new(trace_end_origin, -d).get_point(trace_end_t);

            let mut raycast_result = None;
            block_line_traversal(trace_start, trace_end, |block_coords, _time, face| {
                let block = get_block(block_coords);

                if !block.is_unset() && filter.call((block_coords.as_vec3(), block)) {
                    if block.is_solid() {
                        raycast_result = Some(BlockRaycastResult {
                            position: block_coords.as_vec3(),
                            normal: face.try_into().ok(),
                            block,
                        });

                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            });

            raycast_result
        })
    }
}

#[inline]
pub fn get_chunk_block_position(position: IVec3) -> (IVec3, UVec3) {
    let chunk_position = IVec3 {
        x: (position.x as f32 / CHUNK_SIZE_F).floor() as i32,
        y: (position.y as f32 / CHUNK_SIZE_F).floor() as i32,
        z: (position.z as f32 / CHUNK_SIZE_F).floor() as i32,
    };

    let block_position = (position - chunk_position * CHUNK_SIZE_I).as_uvec3() + 1;

    (chunk_position, block_position)
}
