use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component)]
pub struct DisplayName(pub String);

#[derive(Component)]
pub struct Health {
    pub current: u8,
    pub max: u8,
}

impl Health {
    pub fn new(max_health: u8) -> Self {
        Self {
            current: max_health,
            max: max_health,
        }
    }

    pub fn damage(&mut self, d: u8) {
        self.current -= d;
    }

    pub fn heal(&mut self, h: u8) {
        self.current += h;
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: DisplayName,
    pub health: Health,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
