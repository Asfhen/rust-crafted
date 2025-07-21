use bevy::{
    ecs::{
        query::{Changed, With},
        system::{Query, ResMut},
    },
    transform::components::GlobalTransform,
};
use voxel_engine::{Player, PlayerBundle};

pub fn update_player_pos(
    player: Query<&GlobalTransform, (With<Player>, Changed<GlobalTransform>)>,
    // mut chunk_pos: ResMut<>,
) {
    let transform = player.single().unwrap();
}
