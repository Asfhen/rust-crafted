use bevy::color::palettes::css;

use crate::{BlockMaterial, BlockMaterialFlags};

pub struct Bedrock;

impl BlockMaterial for Bedrock {
    const ID: u64 = 1;

    fn block_name() -> &'static str { "bedrock" }
    fn base_color() -> bevy::color::Color { css::DARK_GRAY.into() }

    fn flags() -> crate::BlockMaterialFlags { BlockMaterialFlags::UNBREAKABLE }

    fn perceptual_roughness() -> f32 { 0.9 }

    fn metallic() -> f32 { 1.0 }
}
