use bevy::{
    log::LogPlugin, pbr::CascadeShadowConfigBuilder, platform::collections::HashMap, prelude::*,
};
use noise::{HybridMulti, NoiseFn, Perlin};
use std::sync::Arc;
use voxel_engine::{
    debug::setup_file_logging,
    prelude::{BlockLookupDelegate, BlockWorldCamera, BlockWorldConfig, BlockWorldPlugin, WorldBlock},
};

#[derive(Resource, Clone, Default)]
struct MyWorld;

impl BlockWorldConfig for MyWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    fn spawning_distance(&self) -> u32 {
        25
    }

    fn min_despawn_distance(&self) -> u32 {
        1
    }

    fn block_lookup_delegate(
        &self,
    ) -> BlockLookupDelegate<Self::MaterialIndex> {
        Box::new(move |_chunk_pos| get_block_fn())
    }

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(|mat| match mat {
            0 => [0, 0, 0],
            1 => [1, 1, 1],
            2 => [2, 2, 2],
            3 => [3, 3, 3],
            _ => [0, 0, 0],
        })
    }
}

fn main() {
    let _guard = setup_file_logging();
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins(BlockWorldPlugin::with_config(MyWorld))
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-200.0, 180.0, -200.0).looking_at(Vec3::ZERO, Vec3::Y),
        BlockWorldCamera::<MyWorld>::default(),
    ));

    let cascade_shadow_config = CascadeShadowConfigBuilder { ..default() }.build();
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.15, -0.1, 0.15), Vec3::Y),
        cascade_shadow_config,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.98, 0.95, 0.82),
        brightness: 100.0,
        affects_lightmapped_meshes: true,
    });
}

fn get_block_fn() -> Box<dyn FnMut(IVec3) -> WorldBlock + Send + Sync> {
    let mut noise = HybridMulti::<Perlin>::new(14);
    noise.octaves = 5;
    noise.frequency = 1.1;
    noise.lacunarity = 2.8;
    noise.persistence = 0.4;

    let mut cache = HashMap::<(i32, i32), f64>::new();

    Box::new(move |pos: IVec3| {
        if pos.y < 1 {
            return WorldBlock::Solid(3);
        }

        let [x, y, z] = pos.as_dvec3().to_array();

        let is_ground = y < match cache.get(&(pos.x, pos.z)) {
            Some(sample) => *sample,
            None => {
                let sample = noise.get([x / 1000.0, z / 1000.0]) * 50.0;
                cache.insert((pos.x, pos.z), sample);
                sample
            }
        };

        if is_ground {
            WorldBlock::Solid(0)
        } else {
            WorldBlock::Air
        }
    })
}

fn move_camera(
    time: Res<Time>,
    mut cam_transform: Query<&mut Transform, With<BlockWorldCamera<MyWorld>>>,
) {
    let Ok(mut transform) = cam_transform.single_mut() else {
        return;
    };
    transform.translation.x += time.delta_secs() * 30.0;
    transform.translation.z += time.delta_secs() * 60.0;
}
