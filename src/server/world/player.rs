use bevy::prelude::*;
use voxel_engine::common::components::player::PlayerBundle;

#[derive(Component)]
pub struct IsOnGround(pub bool);

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

#[derive(Bundle)]
pub struct ServerPlayerBundle {
    #[bundle()]
    pub shared: PlayerBundle,

    pub is_on_ground: IsOnGround,
    pub velocity: Velocity,
}
