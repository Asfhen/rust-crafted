use bevy::prelude::*;
use crate::common::world::position::Position;

#[derive(Component)]
pub struct Player {
    pub username: String,
    pub health: i64,
    pub world_position: Position,
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player {
            health: 20,
            username: "player".to_string(),
            world_position: Position::new(0, 0, 0)
        },
        Transform::default(),
        GlobalTransform::default(),
    ));
}
