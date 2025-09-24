use bevy::app::App;
use bevy::MinimalPlugins;
use voxel_engine::debug::setup_file_logging;


fn main() {
    let _guard = setup_file_logging();
    App::new()
        .add_plugins(MinimalPlugins)
        .run();
}