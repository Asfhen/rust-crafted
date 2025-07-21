use std::f32::consts::PI;

use bevy::{app::{Plugin, Startup, Update}, color::Color, core_pipeline::core_3d::Camera3d, ecs::{schedule::IntoScheduleConfigs, system::Commands}, pbr::AmbientLight, render::camera::{PerspectiveProjection, Projection}, transform::components::{GlobalTransform, Transform}};

pub mod player;
pub use player::*;
use voxel_engine::common::components::player::{DisplayName, Health, Player};

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
        .add_systems(
            Startup,
            spawn_player,
        )
        .add_systems(
            Update,
            (handle_player_input, handle_player_mouse_move)
                .chain()
        );
    }
}

fn spawn_player(mut cmds: Commands) {
    cmds
    .spawn(Camera3d::default())
    .insert(
        Projection::from(
            PerspectiveProjection {
                fov: PI / 2.,
                far: 2048.,
                ..Default::default()
            }
        )
    )
    .insert(Player)
    .insert(DisplayName("Player".to_string()))
    .insert(Health::new(20))
    .insert(PlayerController::default())
    .insert(Transform::default())
    .insert(GlobalTransform::default());

    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 80.,
        affects_lightmapped_meshes: true,
    })
}
