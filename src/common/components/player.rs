use bevy::prelude::*;
use ilattice::glam::UVec3;

#[derive(Component)]
pub struct PlayerData {
    pub username: String,
    pub health: i64,
    pub world_position: UVec3,
    pub jump_strength: f32,
    pub movement_speed: f32,
    pub is_on_ground: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            health: 20,
            username: "player".to_string(),
            world_position: UVec3::new(0, 0, 0),
            jump_strength: 7.0,
            movement_speed: 5.0,
            is_on_ground: false,
        }
    }
}