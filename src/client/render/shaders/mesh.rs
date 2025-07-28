use bevy::render::mesh::{Indices, Mesh, VertexAttributeValues};
use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};
use ndcopy::copy3;
use ndshape::{RuntimeShape, Shape};
use std::marker::PhantomData;
use voxel_engine::{BlockBuffer, MaterialBlock};

use crate::render::BlockTerrainMesh;

pub struct MeshBuffer<T, S: Shape<3, Coord = u32>>
where
    T: Copy + Default + MaterialBlock,
{
    scratch_buffer: BlockBuffer<T, RuntimeShape<u32, 3>>,
    greedy_buffer: GreedyQuadsBuffer,
    _phantom: PhantomData<S>,
}

impl<T, S: Shape<3, Coord = u32>> MeshBuffer<T, S>
where
    T: Copy + Default + MaterialBlock,
{
    pub fn new(shape: S) -> Self {
        let padded_shape = RuntimeShape::<u32, 3>::new(shape.as_array().map(|x| x + 2));

        Self {
            greedy_buffer: GreedyQuadsBuffer::new(padded_shape.size() as usize),
            scratch_buffer: BlockBuffer::<T, RuntimeShape<u32, 3>>::new_empty(padded_shape),
            _phantom: Default::default(),
        }
    }
}

pub fn mesh_buffer<T, S>(
    buffer: &BlockBuffer<T, S>,
    mesh_buffers: &mut MeshBuffer<T, S>,
    render_mesh: &mut Mesh,
    scale: f32,
) where
    T: Copy + Default + MaterialBlock,
    S: Shape<3, Coord = u32>,
{
    mesh_buffers
        .greedy_buffer
        .reset(buffer.shape().size() as usize);

    let dst_shape = mesh_buffers.scratch_buffer.shape().clone();

    copy3(
        buffer.shape().as_array(),
        buffer.slice(),
        buffer.shape(),
        [0; 3],
        mesh_buffers.scratch_buffer.slice_mut(),
        &dst_shape,
        [1; 3],
    );

    greedy_quads(
        mesh_buffers.scratch_buffer.slice(),
        mesh_buffers.scratch_buffer.shape(),
        [0; 3],
        mesh_buffers
            .scratch_buffer
            .shape()
            .as_array()
            .map(|axis| axis - 1),
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut mesh_buffers.greedy_buffer,
    );

    let num_indices = mesh_buffers.greedy_buffer.quads.num_quads() * 6;
    let num_vertices = mesh_buffers.greedy_buffer.quads.num_quads() * 4;
    let mut indices: Vec<u32> = Vec::with_capacity(num_indices);
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut data: Vec<u32> = Vec::with_capacity(num_vertices);

    for (block_face_normal_index, (group, face)) in mesh_buffers
        .greedy_buffer
        .quads
        .groups
        .as_ref()
        .iter()
        .zip(RIGHT_HANDED_Y_UP_CONFIG.faces.iter())
        .enumerate()
    {
        for quad in group {
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(quad, scale));
            data.extend_from_slice(
                &[(block_face_normal_index as u32) << 8u32
                    | buffer
                        .block_at(quad.minimum.map(|x| x - 1).into())
                        .as_mat_id() as u32; 4],
            );
        }
    }

    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(positions),
    );

    render_mesh.insert_attribute(
        BlockTerrainMesh::ATTRIBUTE_DATA,
        VertexAttributeValues::Uint32(data),
    );

    render_mesh.insert_indices(Indices::U32(indices.clone()));
}
