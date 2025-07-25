// use crate::{
//     render::chunk_meshing::chunk_mesh_system,
//     systems::player::{camera_follow_player, player_movement, spawn_player, update_ground_state},
// };
use bevy::{log::LogPlugin, prelude::*};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use voxel_engine::common::{
    // generation::noise::WorldNoise,
    logging::setup_file_logging,
    // world::{
    //     block::BlockRegistry,
    //     chunk::{chunk_generation_system, ChunkGenerationState, ChunkManager},
    // },
};

use crate::systems::SystemsPlugin;

mod render;
mod systems;

fn main() {
    let _guard = setup_file_logging();
    App::new()
        // .insert_resource(WorldNoise::default())
        .add_plugins((
            DefaultPlugins.build().disable::<LogPlugin>(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            SystemsPlugin,
        ))
        .run();
}
