use bevy::math::{ivec2, IVec2, IVec3};

pub const CHUNK_SIZE: i32 = 32;
pub const CHUNK_HEIGHT: i32 = 256;
pub const CHUNK_SIZE_P: i32 = CHUNK_SIZE + 2;
pub const CHUNK_SIZE_P2: i32 = CHUNK_SIZE_P * CHUNK_SIZE_P;
pub const CHUNK_SIZE_P3: i32 = CHUNK_SIZE_P * CHUNK_SIZE_P * CHUNK_SIZE_P;
pub const CHUNK_SIZE2: i32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_SIZE3: i32 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const ADJACENT_CHUNK_DIRECTIONS: [IVec3; 27] = [
    IVec3 { x: 0, y: 0, z: 0 },
    // moore neighbours in the negative direction
    IVec3 { x:  0, y: -1, z: -1 },
    IVec3 { x: -1, y:  0, z: -1 },
    IVec3 { x: -1, y:  0, z:  1 },
    IVec3 { x: -1, y: -1, z:  0 },
    IVec3 { x: -1, y: -1, z: -1 },
    IVec3 { x: -1, y:  1, z: -1 },
    IVec3 { x: -1, y: -1, z:  1 },
    IVec3 { x: -1, y:  1, z:  1 },
    IVec3 { x:  1, y:  0, z: -1 },
    IVec3 { x:  1, y: -1, z: -1 },
    IVec3 { x:  0, y:  1, z: -1 },
    IVec3 { x:  1, y:  1, z:  1 },
    IVec3 { x:  1, y: -1, z:  1 },
    IVec3 { x:  1, y:  1, z: -1 },
    IVec3 { x:  1, y:  1, z:  0 },
    IVec3 { x:  0, y:  1, z:  1 },
    IVec3 { x:  1, y: -1, z:  0 },
    IVec3 { x:  0, y: -1, z:  1 },
    IVec3 { x:  1, y:  0, z:  1 },
    IVec3 { x: -1, y:  1, z:  0 },
    // von neumann neighbour
    IVec3 { x: -1, y:  0, z:  0 },
    IVec3 { x:  1, y:  0, z:  0 },
    IVec3 { x:  0, y: -1, z:  0 },
    IVec3 { x:  0, y:  1, z:  0 },
    IVec3 { x:  0, y:  0, z: -1 },
    IVec3 { x:  0, y:  0, z:  1 },
];

pub const ADJACENT_AO_DIRS: [IVec2; 9] = [
    ivec2(-1, -1),
    ivec2(-1,  0),
    ivec2(-1,  1),
    ivec2( 0, -1),
    ivec2( 0,  0),
    ivec2( 0,  1),
    ivec2( 1, -1),
    ivec2( 1,  0),
    ivec2( 1,  1),
];

pub mod common;
