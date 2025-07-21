use bevy::color::Color;
use crate::common::world::material::{BlockMaterial, BlockMaterialFlags};

pub struct Dirt;

impl BlockMaterial for Dirt {
    fn namespace() -> &'static str { "rust_crafted" }
    fn block_name() -> &'static str { "dirt" }
    fn variant() -> Option<&'static str> { None }
    fn flags() -> BlockMaterialFlags { BlockMaterialFlags::SOLID }
    fn emissive() -> Color { Color::BLACK }
    fn perceptual_roughness() -> f32 { 0.75 }
    fn reflectance() -> f32 { 0.45 }
    fn base_color() -> Color { Color::srgb_u8(112, 97, 92) }
}
