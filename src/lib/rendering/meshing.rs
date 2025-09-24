use crate::{
    custom_meshing::CHUNK_SIZE_U,
    data::chunk::PaddedChunkShape,
    materials::block::ATTRIBUTE_TEX_INDEX,
    prelude::{TextureIndexMapperFn, WorldBlock},
};
use bevy::{
    asset::RenderAssetUsages,
    math::IVec3,
    render::mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues},
};
use block_mesh::{
    ndshape::ConstShape, visible_block_faces, OrientedBlockFace, UnitQuadBuffer, UnorientedQuad,
    Voxel, VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG,
};
use std::sync::Arc;

pub type BlockArray<I> = Arc<[WorldBlock<I>; PaddedChunkShape::SIZE as usize]>;

pub fn generate_chunk_mesh<I: PartialEq + Copy>(
    blocks: BlockArray<I>,
    _pos: IVec3,
    texture_index_mapper: TextureIndexMapperFn<I>,
) -> Mesh {
    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
    let mut buffer = UnitQuadBuffer::new();

    visible_block_faces(
        &*blocks,
        &PaddedChunkShape {},
        [0; 3],
        [CHUNK_SIZE_U + 1; 3],
        &faces,
        &mut buffer,
    );

    mesh_from_quads(buffer, faces, blocks, texture_index_mapper)
}

pub fn mesh_from_quads<I: PartialEq + Copy>(
    quads: UnitQuadBuffer,
    faces: [OrientedBlockFace; 6],
    blocks: BlockArray<I>,
    texture_index_mapper: TextureIndexMapperFn<I>,
) -> Mesh {
    let num_indices = quads.num_quads() * 6;
    let num_vertices = quads.num_quads() * 4;

    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut tex_coords = Vec::with_capacity(num_vertices);
    let mut material_types = Vec::with_capacity(num_vertices);
    let mut aos = Vec::with_capacity(num_vertices);

    for (group, face) in quads.groups.into_iter().zip(faces.into_iter()) {
        for quad in group.iter() {
            let normal = IVec3::from([
                face.signed_normal().x,
                face.signed_normal().y,
                face.signed_normal().z,
            ]);

            let ao = face_aos(&quad.minimum, &normal, &blocks);
            aos.extend_from_slice(&ao);

            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));

            positions.extend_from_slice(&face.quad_mesh_positions(
                &UnorientedQuad {
                    minimum: quad.minimum,
                    width: 1,
                    height: 1,
                },
                1.0,
            ));

            normals.extend_from_slice(&face.quad_mesh_normals());

            tex_coords.extend_from_slice(&face.tex_coords(
                RIGHT_HANDED_Y_UP_CONFIG.u_flip_face,
                true,
                &UnorientedQuad {
                    minimum: quad.minimum,
                    width: 1,
                    height: 1,
                },
            ));

            let block_index = PaddedChunkShape::linearize(quad.minimum) as usize;
            let material_type = match blocks[block_index] {
                WorldBlock::Solid(mt) => texture_index_mapper(mt),
                WorldBlock::Transparent(mt) => texture_index_mapper(mt),
                _ => [0, 0, 0],
            };
            material_types.extend_from_slice(&[material_type; 4]);
        }
    }

    let mut render_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(positions.clone()),
    );
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(normals),
    );
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::Float32x2(tex_coords),
    );
    render_mesh.insert_attribute(
        ATTRIBUTE_TEX_INDEX,
        VertexAttributeValues::Uint32x3(material_types),
    );

    {
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .enumerate()
            .map(|(i, _)| match aos[i] {
                0 => [0.1, 0.1, 0.1, 1.0],
                1 => [0.3, 0.3, 0.3, 1.0],
                2 => [0.5, 0.5, 0.5, 1.0],
                3 => [1.0, 1.0, 1.0, 1.0],
                _ => [1.0, 1.0, 1.0, 1.0],
            })
            .collect();
        render_mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            VertexAttributeValues::Float32x4(colors),
        );
    }

    render_mesh.insert_indices(Indices::U32(indices.clone()));

    render_mesh
}

fn ao_value(side1: bool, corner: bool, side2: bool) -> u32 {
    match (side1, corner, side2) {
        (true, _, true) => 0,
        (true, true, false) | (false, true, true) => 1,
        (false, false, false) => 3,
        _ => 2,
    }
}

fn side_aos<I: PartialEq>(neighbors: [WorldBlock<I>; 8]) -> [u32; 4] {
    let ns = [
        neighbors[0].get_visibility() == VoxelVisibility::Opaque,
        neighbors[1].get_visibility() == VoxelVisibility::Opaque,
        neighbors[2].get_visibility() == VoxelVisibility::Opaque,
        neighbors[3].get_visibility() == VoxelVisibility::Opaque,
        neighbors[4].get_visibility() == VoxelVisibility::Opaque,
        neighbors[5].get_visibility() == VoxelVisibility::Opaque,
        neighbors[6].get_visibility() == VoxelVisibility::Opaque,
        neighbors[7].get_visibility() == VoxelVisibility::Opaque,
    ];

    [
        ao_value(ns[0], ns[1], ns[2]),
        ao_value(ns[2], ns[3], ns[4]),
        ao_value(ns[6], ns[7], ns[0]),
        ao_value(ns[4], ns[5], ns[6]),
    ]
}

fn face_aos<I: PartialEq + Copy>(
    block_pos: &[u32; 3],
    face_normal: &IVec3,
    blocks: &BlockArray<I>,
) -> [u32; 4] {
    let [x, y, z] = *block_pos;

    match *face_normal {
        IVec3::NEG_X => side_aos([
            blocks[PaddedChunkShape::linearize([x - 1, y, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y - 1, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y - 1, z]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y - 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y + 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y + 1, z]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y + 1, z - 1]) as usize],
        ]),
        IVec3::X => side_aos([
            blocks[PaddedChunkShape::linearize([x + 1, y, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y - 1, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y - 1, z]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y - 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y + 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y + 1, z]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y + 1, z - 1]) as usize],
        ]),
        IVec3::NEG_Y => side_aos([
            blocks[PaddedChunkShape::linearize([x, y - 1, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y - 1, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y - 1, z]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y - 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x, y - 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y - 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y - 1, z]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y - 1, z - 1]) as usize],
        ]),
        IVec3::Y => side_aos([
            blocks[PaddedChunkShape::linearize([x, y + 1, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y + 1, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y + 1, z]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y + 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x, y + 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y + 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y + 1, z]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y + 1, z - 1]) as usize],
        ]),
        IVec3::NEG_Z => side_aos([
            blocks[PaddedChunkShape::linearize([x - 1, y, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y - 1, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x, y - 1, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y - 1, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y + 1, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x, y + 1, z - 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y + 1, z - 1]) as usize],
        ]),
        IVec3::Z => side_aos([
            blocks[PaddedChunkShape::linearize([x - 1, y, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y - 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x, y - 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y - 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x + 1, y + 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x, y + 1, z + 1]) as usize],
            blocks[PaddedChunkShape::linearize([x - 1, y + 1, z + 1]) as usize],
        ]),
        _ => unreachable!(),
    }
}
