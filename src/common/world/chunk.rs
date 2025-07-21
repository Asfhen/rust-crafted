use bevy::ecs::component::Component;
use ilattice::glam::I64Vec3 as IVec3;
use ndshape::ConstShape3u32;
use crate::{CHUNK_SIZE, CHUNK_HEIGHT};

#[derive(Component)]
pub struct Chunk(pub IVec3);


const CHUNK_SIZE_U32: u32 = CHUNK_SIZE as u32;
const CHUNK_HEIGHT_U32: u32 = CHUNK_HEIGHT as u32;
pub type ChunkShape = ConstShape3u32<CHUNK_SIZE_U32, CHUNK_HEIGHT_U32, CHUNK_SIZE_U32>;
