use bevy::color::Color;

use crate::{BlockMaterial, BlockMaterialFlags};

pub struct Water;

impl BlockMaterial for Water {
    const ID: u64 = 4;

    fn perceptual_roughness() -> f32 { 0.2 }
    fn metallic() -> f32 { 0.47 }
    fn block_name() -> &'static str { "water" }
    fn base_color() -> bevy::color::Color { Color::srgba_u8(78, 167, 215, 102) }
    fn flags() -> crate::BlockMaterialFlags { BlockMaterialFlags::LIQUID }
}
