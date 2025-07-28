use crate::render::{mesh_buffer, ChunkMaterialSingleton, MeshBuffer};
use bevy::{
    app::{Plugin, Update},
    asset::{Assets, RenderAssetUsages},
    ecs::{
        component::Component,
        entity::Entity,
        query::{Added, With},
        schedule::{IntoScheduleConfigs, SystemSet},
        system::{Commands, Query, Res, ResMut},
    },
    math::Vec3,
    pbr::{MeshMaterial3d, NotShadowCaster},
    render::{
        mesh::{Mesh, Mesh3d, PrimitiveTopology},
        primitives::Aabb,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    tasks::{AsyncComputeTaskPool, Task},
    transform::components::Transform,
};
use futures_lite::future;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use thread_local::ThreadLocal;
use voxel_engine::{
    Block, Chunk, ChunkEntities, ChunkLoadingSet, ChunkMap, ChunkShape, DirtyChunks, TerrainGenSet,
    CHUNK_SIZE,
};

#[derive(Component)]
pub struct ChunkMeshingTask(Task<Mesh>);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, SystemSet)]
pub struct ChunkMeshingSet;

pub struct WorldMeshingPlugin;

impl Plugin for WorldMeshingPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.configure_sets(
            Update,
            ChunkMeshingSet.after(TerrainGenSet).after(ChunkLoadingSet),
        )
        .add_systems(
            Update,
            (prepare_chunks, queue_mesh_tasks, process_mesh_tasks)
                .chain()
                .in_set(ChunkMeshingSet),
        );
    }
}

pub fn prepare_chunks(
    chunks: Query<(Entity, &Chunk), Added<Chunk>>,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<ChunkMaterialSingleton>,
    mut commands: Commands,
) {
    for (chunk, chunk_key) in chunks.iter() {
        let mut entity_commands = commands.entity(chunk);

        entity_commands.insert((
            Mesh3d(meshes.add(Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            ))),
            MeshMaterial3d((**material).clone()),
            Transform::from_translation(chunk_key.0.as_vec3()),
            Visibility::Hidden,
            InheritedVisibility::default(),
            ViewVisibility::default(),
            Aabb::from_min_max(Vec3::ZERO, Vec3::splat(CHUNK_SIZE as f32)),
        ));

        if chunk_key.0.y <= 64 {
            entity_commands.insert(NotShadowCaster);
        }
    }
}

static SHARED_MESH_BUFFERS: Lazy<ThreadLocal<RefCell<MeshBuffer<Block, ChunkShape>>>> =
    Lazy::new(ThreadLocal::default);

fn queue_mesh_tasks(
    mut commands: Commands,
    dirty_chunks: Res<DirtyChunks>,
    chunk_entities: Res<ChunkEntities>,
    chunks: Res<ChunkMap<Block, ChunkShape>>,
) {
    let task_pool = AsyncComputeTaskPool::get();

    dirty_chunks
        .iter_dirty()
        .filter_map(|key| chunk_entities.entity(*key).map(|entity| (key, entity)))
        .filter_map(|(key, entity)| {
            chunks
                .buffer_at(*key)
                .map(|buffer| (buffer.clone(), entity))
        })
        .map(|(buffer, entity)| {
            (
                entity,
                ChunkMeshingTask(task_pool.spawn(async move {
                    let mut mesh_buffers = SHARED_MESH_BUFFERS
                        .get_or(|| {
                            RefCell::new(MeshBuffer::<Block, ChunkShape>::new(ChunkShape {}))
                        })
                        .borrow_mut();

                    let mut mesh = Mesh::new(
                        PrimitiveTopology::TriangleList,
                        RenderAssetUsages::default(),
                    );
                    mesh_buffer(&buffer, &mut mesh_buffers, &mut mesh, 1.0);

                    mesh
                })),
            )
        })
        .for_each(|(entity, task)| {
            commands.entity(entity).insert(task);
        });
}

fn process_mesh_tasks(
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_query: Query<(Entity, &Mesh3d, &mut ChunkMeshingTask), With<Chunk>>,
    mut commands: Commands,
) {
    chunk_query
        .iter_mut()
        .for_each(|(entity, handle, mut mesh_task)| {
            if let Some(mesh) = future::block_on(future::poll_once(&mut mesh_task.0)) {
                *meshes.get_mut(handle.id()).unwrap() = mesh;
                commands.entity(entity).remove::<ChunkMeshingTask>();
            }
        });
}
