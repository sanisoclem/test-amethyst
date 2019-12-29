use amethyst::{
    core::math::*,
    renderer::rendy::mesh::Indices,
    renderer::{
        rendy::mesh::{Color, MeshBuilder, Normal, Position, Tangent, TexCoord},
        types::{Mesh, MeshData},
        Material, Texture,
    },
};
pub fn create_cube_mesh(size: f32) -> MeshData {
    let vertices = vec![
        Position([0.0, 0.0, 0.0]),
        Position([0.0, size, 0.0]),
        Position([size, 0.0, 0.0]),
        Position([size, size, 0.0]),
        Position([0.0, 0.0, size]),
        Position([0.0, size, size]),
        Position([size, 0.0, size]),
        Position([size, size, size]),
    ];
    let indices = vec![
        0, 1, 2, 1, 3, 2, // front
        1, 5, 3, 5, 7, 3, // top
        0, 4, 1, 4, 5, 1, // right
        0, 2, 4, 2, 6, 4, // bottom
        3, 7, 2, 7, 6, 2, // left
        6, 5, 4, 5, 6, 7 as u16, // back
    ];
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

    MeshData(
        MeshBuilder::new()
            .with_vertices(vertices)
            .with_vertices(
                normals
                    .into_iter()
                    .map(|n| Normal(n.normalize().into()))
                    .collect::<Vec<_>>(),
            )
            .with_vertices(vec![
                TexCoord([0.0, 0.0]),
                TexCoord([0.0, 1.0]),
                TexCoord([1.0, 1.0]),
                TexCoord([1.0, 1.0]),
                TexCoord([0.0, 0.0]),
                TexCoord([0.0, 1.0]),
                TexCoord([1.0, 1.0]),
                TexCoord([1.0, 1.0]),
            ])
            .with_indices(Indices::U16(indices.into())),
    )
}

// use amethyst::{
//   core::math::{zero, Vector3},
//   error::Error,
//   renderer::rendy::mesh::{Indices, MeshBuilder, Normal, Position, TexCoord},
// };
// use std::{iter::repeat, ops::Range};

// fn calculate_normals(positions: &[Position], indices: &Indices) -> Vec<Normal> {
//   let mut normals = vec![zero::<Vector3<f32>>(); positions.len()];
//   let num_faces = indices.len().unwrap_or_else(|| positions.len()) / 3;
//   for face in 0..num_faces {
//     let i0 = indices.map(face, 0);
//     let i1 = indices.map(face, 1);
//     let i2 = indices.map(face, 2);
//     let a = Vector3::from(positions[i0].0);
//     let b = Vector3::from(positions[i1].0);
//     let c = Vector3::from(positions[i2].0);
//     let n = (b - a).cross(&(c - a));
//     normals[i0] += n;
//     normals[i1] += n;
//     normals[i2] += n;
//   }
//   normals
//     .into_iter()
//     .map(|n| Normal(n.normalize().into()))
//     .collect::<Vec<_>>()
// }

// fn calculate_tangents(
//   positions: &[Position],
//   normals: &[Normal],
//   tex_coords: &[TexCoord],
//   indices: &Indices,
// ) -> Vec<Tangent> {
//   let mut tangents = vec![Tangent([0.0, 0.0, 0.0, 0.0]); positions.len()];
//   let num_faces = indices.len().unwrap_or_else(|| positions.len()) / 3;
//   mikktspace::generate_tangents(
//     &|| 3,
//     &|| num_faces,
//     &|face, vert| &positions[indices.map(face, vert)].0,
//     &|face, vert| &normals[indices.map(face, vert)].0,
//     &|face, vert| &tex_coords[indices.map(face, vert)].0,
//     &mut |face, vert, tangent| {
//       let [x, y, z, w] = tangent;
//       tangents[indices.map(face, vert)] = Tangent([x, y, z, -w]);
//     },
//   );
//   tangents
// }
