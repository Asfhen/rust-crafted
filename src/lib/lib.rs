mod configuration;
mod data;
mod debug_draw;
mod error;
mod logging;
mod materials;
mod networking;
mod rendering;
mod world;
mod world_plugin;

pub mod prelude {
    pub use crate::configuration::*;
    pub use crate::data::block::{BlockFace, WorldBlock, BLOCK_SIZE};
    pub use crate::data::chunk::{Chunk, NeedsDespawn};
    pub use crate::world::world::{
        ChunkWillDespawn, ChunkWillRemesh, ChunkWillSpawn, ChunkWillUpdate,
    };
    pub use crate::world::world::{
        get_chunk_block_position, BlockRaycastResult, BlockWorld, BlockWorldCamera,
    };
    pub use crate::world_plugin::BlockWorldPlugin;
}

pub mod custom_meshing {
    pub use crate::data::chunk::PaddedChunkShape;
    pub use crate::data::chunk::CHUNK_SIZE_F;
    pub use crate::data::chunk::CHUNK_SIZE_I;
    pub use crate::data::chunk::CHUNK_SIZE_U;
    pub use crate::rendering::meshing::generate_chunk_mesh;
    pub use crate::rendering::meshing::mesh_from_quads;
    pub use crate::rendering::meshing::BlockArray;
}

pub mod debug {
    pub use crate::debug_draw::*;
    pub use crate::error::*;
    pub use crate::logging::setup_file_logging;
}

pub mod traversal_alg {
    pub use crate::world::traversal::*;
}
