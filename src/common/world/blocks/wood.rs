use bevy::color::Color;

use crate::{BlockMaterial, BlockMaterialFlags};

pub struct Wood;

impl BlockMaterial for Wood {
    const ID: u64 = 7;

    fn block_name() -> &'static str { "wood" }
    fn base_color() -> bevy::color::Color { Color::srgb_u8(188, 147, 97) }
    fn flags() -> crate::BlockMaterialFlags { BlockMaterialFlags::SOLID }
    fn perceptual_roughness() -> f32 { 0.7 }
    fn metallic() -> f32 { 0.46 }
}
