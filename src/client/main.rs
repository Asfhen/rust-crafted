use std::f32::consts::PI;
use voxel_engine::*;
use bevy::{core_pipeline::fxaa::Fxaa, log::LogPlugin, prelude::*};

use crate::systems::{sky, PlayerController};

mod render;
mod systems;

fn main() {
    let _guard = setup_file_logging();
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins(WorldPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: PI / 2.,
            far: 2048.0,
            ..Default::default()
        }),
        Transform::from_xyz(2.0, 160.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ))
    .insert(Player)
    .insert(DisplayName("player".to_string()))
    .insert(Health::new(20))
    .insert(PlayerController::default())
    .insert(Fxaa::default())
    .insert(bevy_atmosphere::plugin::AtmosphereCamera::default());

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 80.0,
        ..Default::default()
    });
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(ChunkMap::<Block, ChunkShape>::new(ChunkShape {}))
            .add_plugins(chunk::ChunkingPlugin)
            .add_plugins(render::chunk_meshing::WorldMeshingPlugin)
            // Ordering of plugins is important here;
            .add_plugins(generation::TerrainGeneratorPlugin)
            .add_plugins(terrain::WorldTerrainGenPlugin)
            .add_plugins(material::BlockMaterialPlugin)
            .add_plugins(render::shaders::ChunkMaterialPlugin)
            .add_plugins(world::blocks::BlockBaseMaterialsPlugin)
            .add_plugins(render::chunk_animation::ChunkSpawnAnimatorPlugin)
            .add_plugins(bevy_atmosphere::plugin::AtmospherePlugin)
            .add_plugins(systems::SystemsPlugin)
            .add_plugins(sky::InteractiveSkyboxPlugin);
    }
}
