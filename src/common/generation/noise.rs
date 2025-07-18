use bevy::ecs::resource::Resource;
use libnoise::Perlin;

#[derive(Resource)]
pub struct WorldNoise {
    pub terrain: Perlin<2>,
    pub continental: Perlin<2>,
}

impl Default for WorldNoise {
    fn default() -> Self {
        Self {
            terrain: Perlin::<2>::new(42),
            continental: Perlin::<2>::new(42),
        }
    }
}