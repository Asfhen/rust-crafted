use crate::{
    render::chunk_meshing::chunk_mesh_system,
    systems::player::{camera_follow_player, player_movement, spawn_player, update_ground_state},
};
use bevy::{log::LogPlugin, platform::collections::HashMap, prelude::*};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use voxel_engine::common::{
    generation::noise::WorldNoise,
    logging::setup_file_logging,
    world::{
        block::BlockRegistry,
        chunk::{chunk_generation_system, ChunkGenerationState, ChunkManager},
    },
};

mod render;
mod systems;

fn main() {
    let _guard = setup_file_logging();
    App::new()
        .insert_resource(ChunkManager {
            loaded_chunks: HashMap::new(),
            render_distance: 1,
        })
        .insert_resource(BlockRegistry::default())
        .insert_resource(ChunkGenerationState {
            pending_chunks: Vec::new(),
        })
        .insert_resource(WorldNoise::default())
        .add_plugins((
            DefaultPlugins.build().disable::<LogPlugin>(),
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .add_systems(
            Startup,
            (spawn_player, chunk_generation_system.after(spawn_player)),
        )
        .add_systems(
            Update,
            (
                chunk_generation_system,
                chunk_mesh_system.after(chunk_generation_system),
                player_movement,
                update_ground_state,
                camera_follow_player,
            ),
        )
        .run();
}
