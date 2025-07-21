// use bevy::{
//     asset::RenderAssetUsages,
//     prelude::*,
//     render::mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues},
// };
// use voxel_engine::{
//     common::world::{
//         block::BlockRegistry,
//         chunk::{Chunk, ChunkNeedsMeshing},
//     },
//     CHUNK_HEIGHT, CHUNK_SIZE,
// };

// pub fn chunk_mesh_system(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut query: Query<(Entity, &Chunk), With<ChunkNeedsMeshing>>,
//     block_registry: Res<BlockRegistry>,
// ) {
//     for (entity, chunk) in query.iter_mut() {
//         let mesh = generate_chunk_mesh(chunk, &block_registry);

//         commands
//             .entity(entity)
//             .insert(Mesh3d {
//                 0: meshes.add(mesh),
//             })
//             .insert(MeshMaterial3d {
//                 0: materials.add(StandardMaterial {
//                     base_color: Color::linear_rgb(0.156862, 0.380392, 0.074509),
//                     ..Default::default()
//                 }),
//             })
//             .remove::<ChunkNeedsMeshing>();
//     }
// }

// pub fn generate_chunk_mesh(chunk: &Chunk, _block_registry: &BlockRegistry) -> Mesh {
//     let mut mesh = Mesh::new(
//         PrimitiveTopology::TriangleList,
//         RenderAssetUsages::default(),
//     );
//     let mut positions = Vec::new();
//     let mut normals = Vec::new();
//     let mut uvs = Vec::new();
//     let mut indices = Vec::new();
//     let mut index_offset = 0;

//     for x in 0..CHUNK_SIZE as usize {
//         for z in 0..CHUNK_SIZE as usize {
//             for y in 0..CHUNK_HEIGHT as usize {
//                 let block_idx = (x, y, z);
//                 let block = &chunk.blocks.get(&block_idx).unwrap();

//                 if block.block_type.is_none() {
//                     continue; // Skip empty blocks
//                 }

//                 let neighbors = [
//                     (x + 1, y, z),             // Right
//                     (x.wrapping_sub(1), y, z), // Left
//                     (x, y + 1, z),             // Up
//                     (x, y.wrapping_sub(1), z), // Down
//                     (x, y, z + 1),             // Forward
//                     (x, y, z.wrapping_sub(1)), // Backward
//                 ];

//                 for (face_idx, &(nx, ny, nz)) in neighbors.iter().enumerate() {
//                     let is_visible = if nx < CHUNK_SIZE && ny < CHUNK_HEIGHT && nz < CHUNK_SIZE {
//                         let neighbor_idx = (nx, ny, nz);
//                         chunk.blocks.get(&neighbor_idx).unwrap().block_type.is_none()
//                     } else {
//                         true
//                     };

//                     if is_visible {
//                         add_block_face(
//                             x as f32,
//                             y as f32,
//                             z as f32,
//                             face_idx,
//                             &mut positions,
//                             &mut normals,
//                             &mut uvs,
//                             &mut indices,
//                             &mut index_offset,
//                         );
//                     }
//                 }
//             }
//         }
//     }

//     mesh.insert_attribute(
//         Mesh::ATTRIBUTE_POSITION,
//         VertexAttributeValues::Float32x3(positions),
//     );
//     mesh.insert_attribute(
//         Mesh::ATTRIBUTE_NORMAL,
//         VertexAttributeValues::Float32x3(normals),
//     );
//     mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(uvs));
//     mesh.insert_indices(Indices::U32(indices));

//     mesh
// }

// fn _is_visible(chunk: &Chunk, x: usize, y: usize, z: usize, face: usize) -> bool {
//     let (nx, ny, nz) = match face {
//         0 => (x + 1, y, z),             // Direita
//         1 => (x.wrapping_sub(1), y, z), // Esquerda
//         2 => (x, y + 1, z),             // Topo
//         3 => (x, y.wrapping_sub(1), z), // Base
//         4 => (x, y, z + 1),             // Frente
//         5 => (x, y, z.wrapping_sub(1)), // Trás
//         _ => return true,
//     };

//     // Verificação de limites rigorosa
//     if nx >= CHUNK_SIZE || ny >= CHUNK_HEIGHT || nz >= CHUNK_SIZE {
//         return true;
//     }

//     let idx = (nx, ny, nz);
//     chunk
//         .blocks
//         .get(&idx)
//         .map(|b| b.block_type.is_none())
//         .unwrap_or(true)
// }

// fn add_block_face(
//     x: f32,
//     y: f32,
//     z: f32,
//     face_idx: usize,
//     positions: &mut Vec<[f32; 3]>,
//     normals: &mut Vec<[f32; 3]>,
//     uvs: &mut Vec<[f32; 2]>,
//     indices: &mut Vec<u32>,
//     index_offset: &mut u32,
// ) {
//     let vertices = match face_idx {
//         0 => [
//             // Right (X+)
//             [x + 1.0, y, z],
//             [x + 1.0, y, z + 1.0],
//             [x + 1.0, y + 1.0, z + 1.0],
//             [x + 1.0, y + 1.0, z],
//         ],
//         1 => [
//             // Left (X-)
//             [x, y, z + 1.0],
//             [x, y, z],
//             [x, y + 1.0, z],
//             [x, y + 1.0, z + 1.0],
//         ],
//         2 => [
//             // Top (Y+)
//             [x, y + 1.0, z + 1.0],
//             [x, y + 1.0, z],
//             [x + 1.0, y + 1.0, z],
//             [x + 1.0, y + 1.0, z + 1.0],
//         ],
//         3 => [
//             // Bottom (Y-)
//             [x, y, z],
//             [x, y, z + 1.0],
//             [x + 1.0, y, z + 1.0],
//             [x + 1.0, y, z],
//         ],
//         4 => [
//             // Forward (Z+)
//             [x + 1.0, y, z + 1.0],
//             [x, y, z + 1.0],
//             [x, y + 1.0, z + 1.0],
//             [x + 1.0, y + 1.0, z + 1.0],
//         ],
//         5 => [
//             // Backward (Z-)
//             [x, y, z],
//             [x + 1.0, y, z],
//             [x + 1.0, y + 1.0, z],
//             [x, y + 1.0, z],
//         ],
//         _ => unreachable!(),
//     };

//     // Normals per face
//     let normal = match face_idx {
//         0 => [1.0, 0.0, 0.0],
//         1 => [-1.0, 0.0, 0.0],
//         2 => [0.0, 1.0, 0.0],
//         3 => [0.0, -1.0, 0.0],
//         4 => [0.0, 0.0, 1.0],
//         5 => [0.0, 0.0, -1.0],
//         _ => unreachable!(),
//     };

//     positions.extend_from_slice(&vertices);
//     normals.extend(std::iter::repeat(normal).take(4));

//     uvs.extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);

//     indices.extend(&[
//         *index_offset,
//         *index_offset + 1,
//         *index_offset + 2,
//         *index_offset + 2,
//         *index_offset + 3,
//         *index_offset,
//     ]);

//     *index_offset += 4;
// }
