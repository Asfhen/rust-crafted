use crate::{
    prelude::{BlockWorldConfig, ChunkMeshingFn, TextureIndexMapperFn, WorldBlock},
    world::internals::ModifiedBlocks,
};
use bevy::{
    ecs::{component::Component, entity::Entity},
    math::{IVec3, UVec3, Vec3},
    platform::collections::HashSet,
    render::{mesh::Mesh, primitives::Aabb},
    tasks::Task,
};
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
    sync::Arc,
};

pub const CHUNK_SIZE_U: u32 = 32;
pub const CHUNK_SIZE_I: i32 = CHUNK_SIZE_U as i32;
pub const CHUNK_SIZE_F: f32 = CHUNK_SIZE_U as f32;

pub(crate) const PADDED_CHUNK_SIZE: u32 = CHUNK_SIZE_U + 2;
pub type PaddedChunkShape = ConstShape3u32<PADDED_CHUNK_SIZE, PADDED_CHUNK_SIZE, PADDED_CHUNK_SIZE>;

pub type BlockArray<I> = [WorldBlock<I>; PaddedChunkShape::SIZE as usize];

#[derive(Component)]
#[component(storage = "SparseSet")]
pub(crate) struct ChunkThread<C: BlockWorldConfig, I>(pub Task<ChunkTask<C, I>>, PhantomData<C>);

impl<C, I> ChunkThread<C, I>
where
    C: BlockWorldConfig,
{
    pub fn new(task: Task<ChunkTask<C, I>>, _pos: IVec3) -> Self {
        Self(task, PhantomData)
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct NeedsRemesh;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct NeedsDespawn;

#[derive(Clone, Debug)]
pub enum FillType<I> {
    Empty,
    Mixed,
    Uniform(WorldBlock<I>),
}

#[derive(Debug, Clone)]
pub struct ChunkData<I> {
    pub(crate) position: IVec3,
    pub(crate) blocks: Option<Arc<BlockArray<I>>>,
    pub(crate) blocks_hash: u64,
    pub(crate) is_full: bool,
    pub(crate) is_empty: bool,
    pub(crate) fill_type: FillType<I>,
    pub(crate) entity: Entity,
    pub(crate) has_generated: bool,
}

impl<I: Hash + Copy + PartialEq> ChunkData<I> {
    pub(crate) fn new() -> Self {
        Self {
            position: IVec3::ZERO,
            blocks: None,
            blocks_hash: 0,
            is_full: false,
            is_empty: true,
            fill_type: FillType::Empty,
            entity: Entity::PLACEHOLDER,
            has_generated: false,
        }
    }

    pub(crate) fn with_entity(entity: Entity) -> Self {
        let new = Self::new();
        Self { entity, ..new }
    }

    pub(crate) fn generate_hash(&mut self) {
        if let Some(blocks) = &self.blocks {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            blocks.hash(&mut hasher);
            self.blocks_hash = hasher.finish();
        }
    }

    pub fn get_block(&self, position: UVec3) -> WorldBlock<I> {
        if self.blocks.is_some() {
            self.blocks.as_ref().unwrap()[PaddedChunkShape::linearize(position.to_array()) as usize]
        } else {
            match self.fill_type {
                FillType::Empty => WorldBlock::Unset,
                FillType::Mixed => unreachable!(),
                FillType::Uniform(world_block) => world_block,
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    pub fn is_full(&self) -> bool {
        self.is_full
    }

    pub fn get_fill_type(&self) -> &FillType<I> {
        &self.fill_type
    }

    pub fn get_entity(&self) -> Entity {
        self.entity
    }

    pub fn world_position(&self) -> Vec3 {
        self.position.as_vec3() * CHUNK_SIZE_F
    }

    pub fn aabb(&self) -> Aabb {
        let min = Vec3::ZERO;
        let max = min + Vec3::splat(CHUNK_SIZE_F);
        Aabb::from_min_max(min, max)
    }

    pub fn encloses_point(&self, point: Vec3) -> bool {
        let local_point = point - self.world_position();
        let aabb = self.aabb();
        let min = aabb.min();
        let max = aabb.max();
        local_point.x >= min.x
            && local_point.y >= min.y
            && local_point.z >= min.z
            && local_point.x <= max.x
            && local_point.y <= max.y
            && local_point.z <= max.z
    }

    pub fn has_voxel(&self, block_pos: IVec3, block: WorldBlock<I>) -> bool {
        let chunk_pos = block_pos / CHUNK_SIZE_I;
        if self.position != chunk_pos {
            return false;
        }
        self.get_block(block_pos.as_uvec3() % CHUNK_SIZE_U) == block
    }

    pub fn has_generated(&self) -> bool {
        self.has_generated
    }
}

impl<I: Hash + Copy + PartialEq> Default for ChunkData<I> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Component)]
pub struct Chunk<C> {
    pub position: IVec3,
    pub entity: Entity,
    _marker: PhantomData<C>,
}

impl<C> Chunk<C> {
    pub fn new(position: IVec3, entity: Entity) -> Self {
        Self {
            position: position,
            entity: entity,
            _marker: PhantomData,
        }
    }

    pub fn aabb(&self) -> Aabb {
        let min = Vec3::ZERO;
        let max = min + Vec3::splat(CHUNK_SIZE_F);
        Aabb::from_min_max(min, max)
    }
}

#[derive(Component)]
pub(crate) struct ChunkTask<C, I>
where
    C: BlockWorldConfig,
{
    pub position: IVec3,
    pub chunk_data: ChunkData<I>,
    pub modified_blocks: ModifiedBlocks<C, I>,
    pub mesh: Option<Mesh>,
    pub user_bundle: Option<C::ChunkUserBundle>,
    _marker: PhantomData<C>,
}

impl<C: BlockWorldConfig + Send + Sync + 'static, I: Hash + Copy + Eq> ChunkTask<C, I> {
    pub fn new(entity: Entity, position: IVec3, modified_blocks: ModifiedBlocks<C, I>) -> Self {
        Self {
            position,
            chunk_data: ChunkData::with_entity(entity),
            modified_blocks,
            mesh: None,
            user_bundle: None,
            _marker: PhantomData,
        }
    }

    pub fn generate<F>(&mut self, mut block_data_fn: F)
    where
        F: FnMut(IVec3) -> WorldBlock<I> + Send + 'static,
    {
        let mut filled_count = 0;
        let modified_blocks = (*self.modified_blocks).read().unwrap();
        let mut blocks: [WorldBlock<I>; PaddedChunkShape::SIZE as usize] =
            [WorldBlock::Unset; PaddedChunkShape::SIZE as usize];
        let mut material_count = HashSet::new();

        self.chunk_data.has_generated = true;

        for i in 0..PaddedChunkShape::SIZE {
            let chunk_block = PaddedChunkShape::delinearize(i);

            let block_pos = IVec3 {
                x: chunk_block[0] as i32 + (self.position.x * CHUNK_SIZE_I) - 1,
                y: chunk_block[1] as i32 + (self.position.y * CHUNK_SIZE_I) - 1,
                z: chunk_block[2] as i32 + (self.position.z * CHUNK_SIZE_I) - 1,
            };

            if let Some(block) = modified_blocks.get(&block_pos) {
                blocks[i as usize] = *block;
                if !block.is_unset() && !block.is_air() {
                    filled_count += 1;
                }
                continue;
            }

            let block = block_data_fn(block_pos);

            blocks[i as usize] = block;

            if let WorldBlock::Solid(m) = block {
                filled_count += 1;
                material_count.insert(m);
            }
        }

        self.chunk_data.is_empty = filled_count == 0;
        self.chunk_data.is_full = filled_count == PaddedChunkShape::SIZE;

        if self.chunk_data.is_full && material_count.len() == 1 {
            self.chunk_data.fill_type = FillType::Uniform(blocks[0]);
            self.chunk_data.blocks = None;
        } else if filled_count > 0 {
            self.chunk_data.fill_type = FillType::Mixed;
            self.chunk_data.blocks = Some(Arc::new(blocks));
        } else {
            self.chunk_data.fill_type = FillType::Empty;
            self.chunk_data.blocks = None;
        };

        self.chunk_data.generate_hash();
    }

    pub fn mesh(
        &mut self,
        mut chunk_meshing_fn: ChunkMeshingFn<I, C::ChunkUserBundle>,
        texture_index_mapper: TextureIndexMapperFn<I>,
    ) {
        if self.mesh.is_none() && self.chunk_data.blocks.is_some() {
            let mesh_and_bundle = chunk_meshing_fn(
                self.chunk_data.blocks.as_ref().unwrap().clone(),
                texture_index_mapper,
            );
            self.mesh = Some(mesh_and_bundle.0);
            self.user_bundle = mesh_and_bundle.1;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.chunk_data.is_empty
    }

    pub fn is_full(&self) -> bool {
        self.chunk_data.is_full
    }

    pub fn blocks_hash(&self) -> u64 {
        self.chunk_data.blocks_hash
    }
}
