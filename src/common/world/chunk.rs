use bevy::ecs::component::Component;

use crate::common::{world::block::WorldBlock, I32Position};

#[derive(Component)]
pub struct Chunk {
    pub position: I32Position,
    pub blocks: [[[WorldBlock; 16]; 16]; 16], // Array of 16x16x16 blocks
}
