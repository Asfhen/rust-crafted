use crate::{
    custom_meshing::CHUNK_SIZE_F,
    prelude::{BlockWorld, BlockWorldConfig, Chunk},
};
use bevy::{ecs::system::SystemParam, prelude::*};
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct BlockWorldDebugDrawPlugin<C: BlockWorldConfig> {
    _marker: std::marker::PhantomData<C>,
}

impl<C: BlockWorldConfig> Plugin for BlockWorldDebugDrawPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<ChunkGizmos>()
            .add_systems(Startup, setup::<C>)
            .add_systems(Update, (draw_block_gizmos::<C>, draw_ray_gizmos::<C>));
    }
}

pub struct BlockGizmo {
    pub color: Srgba,
    pub pos: IVec3,
}

#[derive(Resource)]
struct BlockGizmos<C: BlockWorldConfig> {
    gizmos: Arc<RwLock<Vec<BlockGizmo>>>,
    _marker: std::marker::PhantomData<C>,
}

pub struct RayGizmo {
    pub ray: Ray3d,
    pub color: Srgba,
}

#[derive(Resource)]
struct RayGizmos<C: BlockWorldConfig> {
    gizmos: Arc<RwLock<Vec<RayGizmo>>>,
    _marker: std::marker::PhantomData<C>,
}

#[derive(SystemParam)]
pub struct BlockWorldDebugDraw<'w, C: BlockWorldConfig> {
    block_gizmos: Res<'w, BlockGizmos<C>>,
    ray_gizmos: Res<'w, RayGizmos<C>>,
}

impl<C: BlockWorldConfig> BlockWorldDebugDraw<'_, C> {
    pub fn set_block_gizmo(&self, gizmo: BlockGizmo) {
        self.set_block_gizmo_fn()(gizmo);
    }

    pub fn set_block_gizmo_fn(&self) -> Arc<dyn Fn(BlockGizmo) + Send + Sync> {
        let gizmos = self.block_gizmos.gizmos.clone();
        Arc::new(move |gizmo| {
            gizmos.write().unwrap().push(gizmo);
        })
    }

    pub fn clear_block_gizmo(&self, pos: IVec3) {
        self.clear_block_gizmo_fn()(pos);
    }

    pub fn clear_block_gizmo_fn(&self) -> Arc<dyn Fn(IVec3) + Send + Sync> {
        let gizmos = self.block_gizmos.gizmos.clone();
        Arc::new(move |pos: IVec3| {
            gizmos.write().unwrap().retain(|gizmo| gizmo.pos != pos);
        })
    }

    pub fn clear_all_block_gizmos(&self) {
        self.clear_all_block_gizmos_fn()();
    }

    pub fn clear_all_block_gizmos_fn(&self) -> Arc<dyn Fn() + Send + Sync> {
        let gizmos = self.block_gizmos.gizmos.clone();
        Arc::new(move || {
            gizmos.write().unwrap().clear();
        })
    }

    pub fn set_ray_gizmo(&self, gizmo: RayGizmo) {
        self.set_ray_gizmo_fn()(gizmo);
    }

    pub fn set_ray_gizmo_fn(&self) -> Arc<dyn Fn(RayGizmo) + Send + Sync> {
        let gizmos = self.ray_gizmos.gizmos.clone();
        Arc::new(move |gizmo| {
            gizmos.write().unwrap().push(gizmo);
        })
    }

    pub fn clear_ray_gizmo(&self, ray: Ray3d) {
        self.clear_ray_gizmo_fn()(ray);
    }

    pub fn clear_ray_gizmo_fn(&self) -> Arc<dyn Fn(Ray3d) + Send + Sync> {
        let gizmos = self.ray_gizmos.gizmos.clone();
        Arc::new(move |ray| {
            gizmos.write().unwrap().retain(|gizmo| gizmo.ray != ray);
        })
    }

    pub fn clear_all_ray_gizmos(&self) {
        self.clear_all_ray_gizmos_fn()();
    }

    pub fn clear_all_ray_gizmos_fn(&self) -> Arc<dyn Fn() + Send + Sync> {
        let gizmos = self.ray_gizmos.gizmos.clone();
        Arc::new(move || {
            gizmos.write().unwrap().clear();
        })
    }
}

fn setup<C: BlockWorldConfig>(mut commands: Commands) {
    commands.insert_resource(BlockGizmos {
        gizmos: Arc::new(RwLock::new(Vec::new())),
        _marker: std::marker::PhantomData::<C>,
    });
    commands.insert_resource(RayGizmos {
        gizmos: Arc::new(RwLock::new(Vec::new())),
        _marker: std::marker::PhantomData::<C>,
    });
}

fn draw_block_gizmos<C: BlockWorldConfig>(mut gizmos: Gizmos, block_gizmos: Res<BlockGizmos<C>>) {
    for gizmo in block_gizmos.gizmos.read().unwrap().iter() {
        let pos = gizmo.pos.as_vec3();
        let radius = 0.45;
        let color = gizmo.color;

        Vec3::AXES.iter().for_each(|&axis| {
            gizmos.circle(
                Isometry3d::new(
                    pos - Vec3::ONE * 0.5,
                    Quat::from_rotation_arc(Vec3::Z, axis),
                ),
                radius,
                color,
            );
            gizmos.circle(
                Isometry3d::new(
                    pos + Vec3::ONE * 0.5,
                    Quat::from_rotation_arc(Vec3::Z, -axis),
                ),
                radius,
                color,
            );
        });
    }
}

fn draw_ray_gizmos<C: BlockWorldConfig>(mut gizmos: Gizmos, ray_gizmos: Res<RayGizmos<C>>) {
    for gizmo in ray_gizmos.gizmos.read().unwrap().iter() {
        gizmos.line(gizmo.ray.origin, gizmo.ray.get_point(10.0), gizmo.color);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct ChunkGizmos;

pub fn debug_draw_chunks<C: BlockWorldConfig>(
    mut gizmos: Gizmos<ChunkGizmos>,
    chunks: Query<(&Chunk<C>, &GlobalTransform)>,
    block_world: BlockWorld<C>,
) {
    for (chunk, transform) in chunks.iter() {
        let size = Vec3::ONE * CHUNK_SIZE_F;
        let color = Srgba::new(0.0, 1.0, 0.0, 1.0);

        let Some(chunk_data) = block_world.get_chunk_data(chunk.position) else {
            continue;
        };

        if chunk_data.is_empty() {
            continue;
        }

        gizmos.cuboid(
            Transform::from(*transform)
                .with_scale(size)
                .with_translation(transform.translation() + (CHUNK_SIZE_F / 2.0) + 1.0),
            color,
        );
    }
}
