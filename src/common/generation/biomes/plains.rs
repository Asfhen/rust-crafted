use bevy::math::{Vec2, Vec3Swizzles};
use ilattice::prelude::UVec3 as ILUVec3;
use crate::{make_rock, make_tree, noise, BlockMaterial, Grass, LayeredBiomeTerrainGenerator, Leaves, Stone, Wood};

pub struct PlainsBiomeTerrainGenerator;

impl LayeredBiomeTerrainGenerator for PlainsBiomeTerrainGenerator {
    fn place_decoration(
        &self,
        key: bevy::math::IVec3,
        pos: bevy::math::UVec3,
        buffer: &mut crate::BlockBuffer<crate::Block, crate::ChunkShape>,
    ) {
        let spawn_chance = noise::rand2to1(
            (pos.xz().as_vec2() + key.xz().as_vec2()) * 0.01,
            Vec2::new(12.989, 78.233),
        );

        let grass_blade_height = ((noise::rand2to1(
            (pos.xz().as_vec2() + key.xz().as_vec2()) * 0.1,
            Vec2::new(42.478_2, 8_472.243),
        ) * 100.) as u32)
            .rem_euclid(4);

        if grass_blade_height > 1 && pos.y <= 29 {
            for y in 0..grass_blade_height {
                let position = ILUVec3::from_array(pos.to_array()) + ILUVec3::new(0, y, 0);
                *buffer.block_at_mut(position) = Grass::into_block();
            }
        }

        // Let's put some rock boulders in the plains to populate a lil bit
        let rock_spawn_chance = noise::rand2to1(
            (pos.xz().as_vec2() + key.xz().as_vec2()) * 0.1,
            Vec2::new(72_845.48, 8_472.243),
        );

        if rock_spawn_chance > 0.995 {
            let rock_size = (1.0f32 - rock_spawn_chance) * 1000.0;
            make_rock::<Stone>(buffer, ILUVec3::from(pos.to_array()), rock_size);
        }

        if spawn_chance > 0.981 && pos.y <= 13 {
            // this is a stupid hack but a real fix would be to allow terrain decoration to work vertically
            make_tree::<Wood, Leaves>(buffer, ILUVec3::from(pos.to_array()));
        }
    }
}
