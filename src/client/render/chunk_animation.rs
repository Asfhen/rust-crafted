use bevy::{
    app::{Plugin, PostUpdate, Update},
    ecs::{
        component::Component,
        entity::Entity,
        removal_detection::RemovedComponents,
        schedule::{IntoScheduleConfigs, SystemSet},
        system::{Commands, Query, Res},
    },
    render::view::Visibility,
    time::Time,
    transform::components::Transform,
};
use voxel_engine::Chunk;

use crate::render::{ChunkMeshingSet, ChunkMeshingTask};

const ANIMATION_DURATION: f32 = 0.8;
const ANIMATION_HEIGHT: f32 = 128.;

#[derive(Component)]
pub struct ChunkSpawnAnimation {
    start_time: f32,
}

pub struct ChunkSpawnAnimatorPlugin;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, SystemSet)]
pub struct ChunkSpawnAnimatorSet;

impl Plugin for ChunkSpawnAnimatorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.configure_sets(
            PostUpdate,
            ChunkSpawnAnimatorSet.after(ChunkMeshingSet)
        )
        .add_systems(
            Update,
            (step_chunk_animation, attach_chunk_animation)
                .in_set(ChunkSpawnAnimatorSet)
        );
    }
}

fn attach_chunk_animation(
    mut ready_chunks: Query<(&mut Transform, &mut Visibility, &Chunk)>,
    mut removed_chunk_meshes: RemovedComponents<ChunkMeshingTask>,
    time: Res<Time>,
    mut commands: Commands,
) {
    removed_chunk_meshes.read().for_each(|entity| {
        if ready_chunks.contains(entity) {
            commands.entity(entity).insert(ChunkSpawnAnimation {
                start_time: time.elapsed_secs(),
            });
            if let Ok((mut transform, mut visibility, chunk)) = ready_chunks.get_mut(entity) {
                *visibility = Visibility::Visible;
                transform.translation.y = chunk.0.y as f32 - ANIMATION_HEIGHT;
            }
        }
    });
}

fn step_chunk_animation(
    mut chunks: Query<(Entity, &mut Transform, &Chunk, &ChunkSpawnAnimation)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    chunks
        .iter_mut()
        .for_each(|(entity, mut transform, chunk, animation)| {
            let delta = (time.elapsed_secs() - animation.start_time).min(ANIMATION_DURATION);

            let y_transform = (1. - (1. - (delta / ANIMATION_DURATION)).powi(5))
                .mul_add(ANIMATION_HEIGHT, chunk.0.y as f32 - ANIMATION_HEIGHT);

            transform.translation.y = y_transform;

            if delta == ANIMATION_DURATION {
                commands.entity(entity).remove::<ChunkSpawnAnimation>();
            }
        });
}
