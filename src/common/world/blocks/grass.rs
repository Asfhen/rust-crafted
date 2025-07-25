use bevy::color::palettes::css;

use crate::{BlockMaterial, BlockMaterialFlags};


pub struct Grass;

impl BlockMaterial for Grass {
    const ID: u64 = 5;

    fn block_name() -> &'static str {
        "grass"
    }

    fn base_color() -> bevy::color::Color {
        css::LIGHT_GREEN.into()
    }

    fn flags() -> crate::BlockMaterialFlags {
        BlockMaterialFlags::SOLID
    }
}
