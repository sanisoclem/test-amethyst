use crate::components::terrain::{Chunk, VoxelData};
use amethyst::{
    core::math::*,
    renderer::rendy::mesh::Indices,
    renderer::{
        rendy::mesh::{MeshBuilder, Normal, Position, TexCoord},
        types::MeshData,
    },
};

pub fn create_voxel_mesh2(
    voxels: &VoxelData,
    chunk_size: i32,
    voxel_size: f32,
    offset: f32,
    chunk: &Chunk,
) -> MeshData {
    let chunk_size = chunk_size as u16;
    let side_length = chunk_size as f32 * voxel_size;
    let half_size = voxel_size / 2.;

    let vertices = voxels
        .voxels
        .iter()
        .flat_map(|v| {
            let x_origin = (v.x as f32 * voxel_size) - offset;
            let y_origin = (v.y as f32 * voxel_size) - offset;

            vec![
                Position([x_origin + half_size, v.heights[0], y_origin + half_size]),
                Position([
                    x_origin,
                    (v.heights[0] + v.heights[2] + v.heights[4] + v.heights[6]) / 4.,
                    y_origin,
                ]),
            ]
        })
        .chain((0..chunk_size).map(|i| {
            let index = i as usize * chunk_size as usize + (chunk_size as usize - 1);
            let heights = &voxels.voxels[index].heights;
            Position([
                (chunk_size as f32 * voxel_size) - offset,
                (heights[0] + heights[1] + heights[4] + heights[7]) / 4.,
                i as f32 * voxel_size - offset,
            ])
        }))
        .chain((0..chunk_size).map(|i| {
            let index = ((chunk_size * (chunk_size - 1)) + i) as usize;
            let heights = &voxels.voxels[index].heights;
            Position([
                i as f32 * voxel_size - offset,
                (heights[0] + heights[2] + heights[3] + heights[8]) / 4.,
                (chunk_size as f32 * voxel_size) - offset,
            ])
        }))
        .chain((0..1).map(|_| {
            let heights = &voxels.voxels[(chunk_size * chunk_size) as usize - 1].heights;
            Position([
                (chunk_size as f32 * voxel_size) - offset,
                (heights[0] + heights[1] + heights[3] + heights[5]) / 4.,
                (chunk_size as f32 * voxel_size) - offset,
            ])
        }))
        .collect::<Vec<_>>();

    let indices = voxels
        .voxels
        .iter()
        .enumerate()
        .flat_map(|(index, _)| {
            let index = index as u16;

            let i = index * 2;
            let i_right = if ((index + 1) % chunk_size) == 0 {
                (chunk_size * chunk_size * 2) + (index / chunk_size)
            } else {
                (index + 1) * 2 + 1
            };
            let i_down = if (index + chunk_size) / chunk_size >= chunk_size {
                (chunk_size * chunk_size * 2) + (chunk_size) + (index % chunk_size)
            } else {
                (index + chunk_size) * 2 + 1
            };

            let i_down_1 = if (index + chunk_size) / chunk_size >= chunk_size {
                (chunk_size * chunk_size * 2) + (chunk_size) + (index % chunk_size) + 1
            } else if ((index + 1) % chunk_size) == 0 {
                (chunk_size * chunk_size * 2) + (index / chunk_size) + 1
            } else {
                (index + chunk_size + 1) * 2 + 1
            };

            vec![
                // -- how can I replace this with a slice??? I only want to create 1 vector after the collect
                i + 0,
                i + 1,
                i_down,
                i + 0,
                i_down,
                i_down_1,
                i + 0,
                i_down_1,
                i_right,
                i + 0,
                i_right,
                i + 1,
            ]
        })
        .collect::<Vec<_>>();

    let tex_coords = voxels
        .voxels
        .iter()
        .flat_map(|v| {
            let x_origin = (v.x as f32 * voxel_size) - offset;
            let y_origin = (v.y as f32 * voxel_size) - offset;

            vec![
                TexCoord([
                    (x_origin + half_size) / side_length,
                    (y_origin + half_size) / side_length,
                ]),
                TexCoord([x_origin / side_length, y_origin / side_length]),
            ]
        })
        .chain((0..chunk_size).map(|i| TexCoord([1.0, i as f32 / chunk_size as f32])))
        .chain((0..chunk_size + 1).map(|i| TexCoord([i as f32 / chunk_size as f32, 1.0])))
        .collect::<Vec<_>>();

    let normals = calculate_normals(&vertices, &indices);

    MeshData(
        MeshBuilder::new()
            .with_vertices(vertices)
            .with_vertices(normals)
            .with_vertices(tex_coords)
            .with_indices(Indices::U16(indices.into())),
    )
}

// pub fn create_voxel_mesh(
//     voxels: &VoxelData,
//     chunk_size: i32,
//     voxel_size: f32,
//     offset: f32,
// ) -> MeshData {
//     let chunk_size = chunk_size as u16;
//     let vertices = voxels
//         .voxels
//         .iter()
//         .flat_map(|v| {
//             let x_origin = (v.x as f32 * voxel_size) - offset;
//             let y_origin = (v.y as f32 * voxel_size) - offset;

//             vec![
//                 // -- how can I replace this with a slice??? I only want to create 1 vector after the collect
//                 Position([x_origin, v.height, y_origin]),
//                 Position([x_origin + voxel_size, v.height, y_origin]),
//                 Position([x_origin, v.height, y_origin + voxel_size]),
//                 Position([x_origin + voxel_size, v.height, y_origin + voxel_size]),
//             ]
//         })
//         .collect::<Vec<_>>();
//     let indices = voxels
//         .voxels
//         .iter()
//         .enumerate()
//         .flat_map(|(index, _)| {
//             let index = index as u16;

//             let i = index * 4;
//             let i_right = if ((index + 1) % chunk_size) == 0 {
//                 None
//             } else {
//                 Some((index + 1) * 4)
//             };
//             let i_down = if (index + chunk_size) / chunk_size >= chunk_size {
//                 None
//             } else {
//                 Some((index + chunk_size) * 4)
//             };

//             let mut t = vec![
//                 // -- how can I replace this with a slice??? I only want to create 1 vector after the collect
//                 i + 0,
//                 i + 2,
//                 i + 1,
//                 i + 1,
//                 i + 2,
//                 i + 3,
//             ];

//             if let Some(right) = i_right {
//                 t.extend(&[i + 1, i + 3, right, right, i + 3, right + 2]);
//             }

//             if let Some(down) = i_down {
//                 t.extend(&[i + 3, i + 2, down, down, down + 1, i + 3]);
//             }

//             t
//         })
//         .collect::<Vec<_>>();

//     let tex_coords = voxels
//         .voxels
//         .iter()
//         .flat_map(|_| {
//             vec![
//                 // -- how can I replace this with a slice??? I only want to create 1 vector after the collect
//                 TexCoord([0.0, 0.0]),
//                 TexCoord([1.0, 0.0]),
//                 TexCoord([0.0, 1.0]),
//                 TexCoord([1.0, 1.0]),
//             ]
//         })
//         .collect::<Vec<_>>();
//     let normals = calculate_normals(&vertices, &indices);

//     MeshData(
//         MeshBuilder::new()
//             .with_vertices(vertices)
//             .with_vertices(normals)
//             .with_vertices(tex_coords)
//             .with_indices(Indices::U16(indices.into())),
//     )
// }

pub fn create_biome_mesh(size: f32) -> MeshData {
    let vertices = vec![
        Position([size, 0.0, size]),
        Position([-size, 0.0, size]),
        Position([-size, 0.0, -size]),
        Position([size, 0.0, -size]),
    ];
    let indices = vec![0, 2, 1, 0, 3, 2 as u16];
    let normals = calculate_normals(&vertices, &indices);

    MeshData(
        MeshBuilder::new()
            .with_vertices(vertices)
            .with_vertices(normals)
            .with_vertices(vec![
                TexCoord([1.0, 0.0]),
                TexCoord([0.0, 0.0]),
                TexCoord([0.0, 1.0]),
                TexCoord([1.0, 1.0]),
            ])
            .with_indices(Indices::U16(indices.into())),
    )
}
pub fn calculate_normals(vertices: &[Position], indices: &[u16]) -> Vec<Normal> {
    let mut normals = vec![zero::<Vector3<f32>>(); vertices.len()];
    let num_faces = indices.len() / 3;
    {
        for face in 0..num_faces {
            let i0 = face * 3;
            let i1 = i0 + 1;
            let i2 = i0 + 2;
            let a = Vector3::from(vertices[indices[i0] as usize].0);
            let b = Vector3::from(vertices[indices[i1] as usize].0);
            let c = Vector3::from(vertices[indices[i2] as usize].0);
            let n = (b - a).cross(&(c - a));
            normals[indices[i0] as usize] += n;
            normals[indices[i1] as usize] += n;
            normals[indices[i2] as usize] += n;
        }
    }
    normals
        .into_iter()
        .map(|n| Normal(n.normalize().into()))
        .collect::<Vec<_>>()
}
