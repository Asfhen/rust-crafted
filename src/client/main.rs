use bevy::{log::LogPlugin, platform::collections::HashMap, prelude::*};
use camera::CameraPlugin;
use voxel_engine::common::{
    error::{log_error, ErrorEvent}, generation::noise::WorldNoise, logging::setup_file_logging, world::{block::BlockRegistry, chunk::{chunk_generation_system, ChunkGenerationState, ChunkManager}}
};

use crate::render::chunk_meshing::chunk_mesh_system;

mod camera;

mod render;

fn main() {
    let _guard = setup_file_logging();
    App::new()
        .insert_resource(ChunkManager {
            loaded_chunks: HashMap::new(),
            render_distance: 8
        })
        .insert_resource(BlockRegistry::default())
        .insert_resource(ChunkGenerationState {
            pending_chunks: Vec::new()
        })
        .insert_resource(WorldNoise::default())
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins(CameraPlugin)
        .add_systems(Update, (
            chunk_generation_system,
            chunk_mesh_system.after(chunk_generation_system),
        ))
        .run();
}