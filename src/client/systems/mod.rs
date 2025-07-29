use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
};

pub mod player;
pub use player::*;

pub mod sky;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (handle_player_input, handle_player_mouse_move).chain(),
        );
    }
}
