use crate::prelude::{BlockFace, BLOCK_SIZE};
use bevy::{
    math::{IVec3, Vec3},
    prelude::{FromReflect, Struct},
};

pub fn block_cartesian_traversal<F: FnMut(IVec3) -> bool + Sized>(
    start: IVec3,
    end: IVec3,
    mut visit_block: F,
) {
    let delta = end - start;

    // Make sure one and only one component of the direction we follow is non-zero
    debug_assert!(
        delta
            .signum()
            .abs()
            .iter_fields()
            .map(|f| { i32::from_reflect(f).unwrap() })
            .sum::<i32>()
            == 1
    );

    let distance = delta.abs().max_element();
    let direction = delta / distance;

    for d in 0..distance {
        let block_coords = start + direction * d;
        if !visit_block(block_coords) {
            break;
        }
    }
}

pub fn block_line_traversal<F: FnMut(IVec3, f32, BlockFace) -> bool + Sized>(
    start: Vec3,
    end: Vec3,
    mut visit_block: F,
) {
    let ray = end - start;
    let end_t = ray.length();
    let ray_dir = ray / end_t;
    let r_ray_dir = ray_dir.recip();
    let delta_t = (BLOCK_SIZE * r_ray_dir).abs();

    let step = ray_dir.signum().as_ivec3();

    let start_block = start.floor().as_ivec3();
    let end_block = end.floor().as_ivec3();

    let mut block = start_block;
    let mut max_t = Vec3::ZERO;

    max_t.x = if step.x == 0 {
        end_t
    } else {
        let o = if step.x > 0 { 1 } else { 0 };
        let plane = (start_block.x + o) as f32 * BLOCK_SIZE;
        (plane - start.x) * r_ray_dir.x
    };

    max_t.y = if step.y == 0 {
        end_t
    } else {
        let o = if step.y > 0 { 1 } else { 0 };
        let plane = (start_block.y + o) as f32 * BLOCK_SIZE;
        (plane - start.y) * r_ray_dir.y
    };

    max_t.z = if step.z == 0 {
        end_t
    } else {
        let o = if step.z > 0 { 1 } else { 0 };
        let plane = (start_block.z + o) as f32 * BLOCK_SIZE;
        (plane - start.z) * r_ray_dir.z
    };

    let r_end_t = 1. / end_t;
    let mut time = max_t.min_element() * r_end_t;
    let mut face = BlockFace::None;

    let out_of_bounds = end_block + step;
    let mut reached_end = block == end_block;
    let mut keep_going = visit_block(block, time, face);

    let x_face = if step.x > 0 {
        BlockFace::Left
    } else {
        BlockFace::Right
    };
    let y_face = if step.y > 0 {
        BlockFace::Bottom
    } else {
        BlockFace::Top
    };
    let z_face = if step.z > 0 {
        BlockFace::Back
    } else {
        BlockFace::Forward
    };

    while keep_going && !reached_end {
        if max_t.x < max_t.y && max_t.x < max_t.z {
            time = max_t.x * r_end_t;
            face = x_face;

            block.x += step.x;
            max_t.x += delta_t.x;

            reached_end = block.x == out_of_bounds.x;
        } else if max_t.y < max_t.z {
            time = max_t.y * r_end_t;
            face = y_face;

            block.y += step.y;
            max_t.y += delta_t.y;

            reached_end = block.y == out_of_bounds.y;
        } else {
            time = max_t.z * r_end_t;
            face = z_face;

            block.z += step.z;
            max_t.z += delta_t.z;

            reached_end = block.z == out_of_bounds.z;
        }

        if !reached_end {
            keep_going = visit_block(block, time, face);
        }
    }
}
