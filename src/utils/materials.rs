// use amethyst::{
//     assets::{AssetLoaderSystemData, AssetStorage, Handle},
//     controls::FlyControlTag,
//     core::math::{Translation3, Vector3},
//     core::{SystemDesc, Time, Transform},
//     derive::SystemDesc,
//     ecs::prelude::{Entity, Read, ReadStorage, System, SystemData, World, Write, WriteStorage},
//     ecs::{Entities, Join},
//     renderer::rendy::mesh::{Indices, MeshBuilder, Normal, Position, TexCoord},
//     renderer::{
//         mtl::MaterialDefaults,
//         palette::LinSrgba,
//         rendy::texture::palette::load_from_linear_rgba,
//         types::{Mesh, MeshData},
//         Material, Texture,
//     },
// };

// fn create_default_material(world: &mut World) -> Material {
//     use crate::mtl::TextureOffset;

//     use amethyst_assets::Loader;

//     let loader = world.fetch::<Loader>();

//     let albedo = load_from_srgba(Srgba::new(0.5, 0.5, 0.5, 1.0));
//     let emission = load_from_srgba(Srgba::new(0.0, 0.0, 0.0, 0.0));
//     let normal = load_from_linear_rgba(LinSrgba::new(0.5, 0.5, 1.0, 1.0));
//     let metallic_roughness = load_from_linear_rgba(LinSrgba::new(0.0, 0.5, 0.0, 0.0));
//     let ambient_occlusion = load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));
//     let cavity = load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));

//     let tex_storage = world.fetch();

//     let albedo = loader.load_from_data(albedo.into(), (), &tex_storage);
//     let emission = loader.load_from_data(emission.into(), (), &tex_storage);
//     let normal = loader.load_from_data(normal.into(), (), &tex_storage);
//     let metallic_roughness = loader.load_from_data(metallic_roughness.into(), (), &tex_storage);
//     let ambient_occlusion = loader.load_from_data(ambient_occlusion.into(), (), &tex_storage);
//     let cavity = loader.load_from_data(cavity.into(), (), &tex_storage);

//     Material {
//         alpha_cutoff: 0.01,
//         albedo,
//         emission,
//         normal,
//         metallic_roughness,
//         ambient_occlusion,
//         cavity,
//         uv_offset: TextureOffset::default(),
//     }
// }
