use bevy::color::Color;

use crate::{BlockMaterial, BlockMaterialFlags};

pub struct Leaves;

impl BlockMaterial for Leaves {
    const ID: u64 = 6;

    fn block_name() -> &'static str { "leaves" }
    fn base_color() -> bevy::color::Color { Color::srgb_u8(109, 177, 56) }
    fn flags() -> crate::BlockMaterialFlags { BlockMaterialFlags::TRANSPARENT }
    fn perceptual_roughness() -> f32 { 0.73 }
    fn metallic() -> f32 { 1.0 }
}
