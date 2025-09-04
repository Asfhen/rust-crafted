use voxel_engine::*;
use bevy::{log::LogPlugin, prelude::*};

fn main() {
    let _guard = setup_file_logging();
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .run();
}
