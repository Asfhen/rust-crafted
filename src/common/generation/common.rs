use bevy::math::{IVec3, Vec3};
use ilattice::{extent::Extent, glam::{UVec2, UVec3}};
use crate::{sdf, Bedrock, Block, BlockBuffer, BlockMaterial, ChunkShape, HeightMap, Stone, Water, CHUNK_SIZE};

pub fn terrain_generate_world_bottom_border(
    buffer: &mut BlockBuffer<Block, ChunkShape>
) {
    buffer.fill_extent(
        Extent::from_min_and_shape(UVec3::ZERO, UVec3::new(CHUNK_SIZE as u32, 2, CHUNK_SIZE as u32)),
        Bedrock::into_block(),
    );
}

pub fn carve_terrain_heightmap(
    buffer: &mut BlockBuffer<Block, ChunkShape>,
    key: IVec3,
    heightmap: &HeightMap<CHUNK_SIZE, CHUNK_SIZE>
) {
    if key.y <= 0 {
        buffer.fill_extent(
            Extent::from_min_and_shape(UVec3::ZERO, UVec3::splat(CHUNK_SIZE as u32)),
            Water::into_block(),
        );
    }

    Extent::from_min_and_shape(
        UVec2::ZERO,
        UVec2::new(CHUNK_SIZE as u32, CHUNK_SIZE as u32))
        .iter2()
        .for_each(|pos| {
            let local_height = heightmap
                .get(pos.into())
                .checked_sub(key.y as u32)
                .unwrap_or_default()
                .min(CHUNK_SIZE as u32);

            for h in 0..local_height {
                *buffer.block_at_mut([pos.x, h, pos.y].into()) = Stone::into_block();
            }
        });
}

pub fn make_pine_tree<T: BlockMaterial, L: BlockMaterial>(
    buffer: &mut BlockBuffer<Block, ChunkShape>,
    origin: UVec3,
) {
    let origin = Vec3::from(origin.as_vec3().to_array());
    Extent::from_min_and_shape(UVec3::ZERO, UVec3::splat(CHUNK_SIZE as u32)) //may want to calculate an extent encompassing the tree instead of iterating over the complete 32^3 volume
        .iter3()
        .map(|x| Vec3::from_array(x.as_vec3().to_array()))
        .map(|position| {
            let trunk_distance =
                sdf::sdf_capped_cylinder(position - (origin + 2.0 * Vec3::Y), 1.5, 8.0) < 0.;
            let leaves_distance =
                sdf::sdf_v_cone(position - (origin + 6.0 * Vec3::Y), 7.0, 17.0) < 0.;
            (trunk_distance, leaves_distance, position)
        })
        .map(|(trunk_distance, leaves_distance, position)| {
            (
                trunk_distance,
                leaves_distance,
                UVec3::from(position.as_uvec3().to_array()),
            )
        })
        .for_each(|(trunk_distance, leaves_distance, position)| {
            if trunk_distance {
                *buffer.block_at_mut(position) = T::into_block();
            }

            if leaves_distance {
                *buffer.block_at_mut(position) = L::into_block();
            }
        });
}

/// Make a tree using SDF functions
pub fn make_tree<T: BlockMaterial, L: BlockMaterial>(
    buffer: &mut BlockBuffer<Block, ChunkShape>,
    origin: UVec3,
) {
    let origin = Vec3::from(origin.as_vec3().to_array());
    Extent::from_min_and_shape(UVec3::ZERO, UVec3::splat(CHUNK_SIZE as u32)) //may want to calculate an extent encompassing the tree instead of iterating over the complete 32^3 volume
        .iter3()
        .map(|x| Vec3::from_array(x.as_vec3().to_array()))
        .map(|position| {
            let trunk_distance =
                sdf::sdf_capped_cylinder(position - (origin + 2.0 * Vec3::Y), 1.5, 8.0) < 0.;
            let leaves_distance = sdf::sdf_sphere(position - (origin + 14.0 * Vec3::Y), 6.0) < 0.;
            (trunk_distance, leaves_distance, position)
        })
        .map(|(trunk_distance, leaves_distance, position)| {
            (
                trunk_distance,
                leaves_distance,
                UVec3::from(position.as_uvec3().to_array()),
            )
        })
        .for_each(|(trunk_distance, leaves_distance, position)| {
            if trunk_distance {
                *buffer.block_at_mut(position) = T::into_block();
            }

            if leaves_distance {
                *buffer.block_at_mut(position) = L::into_block();
            }
        });
}

pub fn make_rock<V: BlockMaterial>(
    buffer: &mut BlockBuffer<Block, ChunkShape>,
    origin: UVec3,
    size: f32,
) {
    let origin = Vec3::from(origin.as_vec3().to_array());
    Extent::from_min_and_shape(UVec3::ZERO, UVec3::splat(CHUNK_SIZE as u32)) //may want to calculate an extent encompassing the tree instead of iterating over the complete 32^3 volume
        .iter3()
        .map(|x| Vec3::from_array(x.as_vec3().to_array()))
        .map(|position| {
            let trunk_distance = sdf::sdf_sphere(position - origin, size) < 0.;
            (trunk_distance, position)
        })
        .map(|(rock_distance, position)| {
            (rock_distance, UVec3::from(position.as_uvec3().to_array()))
        })
        .for_each(|(rock, position)| {
            if rock {
                *buffer.block_at_mut(position) = V::into_block();
            }
        });
}
