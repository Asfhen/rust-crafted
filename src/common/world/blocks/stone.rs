use bevy::color::{palettes::css, Color};

use crate::{BlockMaterial, BlockMaterialFlags};

pub struct Stone;

impl BlockMaterial for Stone {
    const ID: u64 = 3;

    fn block_name() -> &'static str { "stone" }
    fn base_color() -> bevy::color::Color { css::GRAY.into() }
    fn flags() -> crate::BlockMaterialFlags { BlockMaterialFlags::SOLID }
    fn emissive() -> bevy::color::Color { Color::BLACK }
    fn perceptual_roughness() -> f32 { 0.85 }
    fn metallic() -> f32 { 0.6 }
}
