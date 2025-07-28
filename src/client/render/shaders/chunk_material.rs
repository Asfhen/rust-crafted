use bevy::{
    app::{Plugin, Update},
    asset::{Asset, Assets, Handle},
    color::{Color, LinearRgba},
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        resource::Resource,
        schedule::{common_conditions::resource_changed, IntoScheduleConfigs, SystemSet},
        system::{Commands, Query, Res, ResMut},
        world::FromWorld,
    },
    pbr::{Material, MaterialPlugin},
    prelude::{Deref, DerefMut},
    reflect::TypePath,
    render::{
        extract_component::ExtractComponent,
        mesh::{Mesh, MeshVertexAttribute, VertexFormat},
        render_resource::{AsBindGroup, ShaderType},
    },
    utils::default,
};
use voxel_engine::{material, BlockMaterialRegistry};

#[derive(Component, Clone, Default, ExtractComponent)]
pub struct BlockTerrainMesh;

impl BlockTerrainMesh {
    pub const ATTRIBUTE_DATA: MeshVertexAttribute =
        MeshVertexAttribute::new("vertex_data", 0x696969, VertexFormat::Uint32);
}

#[derive(ShaderType, Clone, Copy, Debug, Default)]
pub struct GpuBlockMaterial {
    base_color: LinearRgba,
    flags: u32,
    emissive: LinearRgba,
    perceptual_roughness: f32,
    metallic: f32,
    reflectance: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GpuTerrainUniforms {
    #[uniform(0)]
    pub render_distance: u32,
    #[uniform(1)]
    pub materials: [GpuBlockMaterial; 256],
}

impl Default for GpuTerrainUniforms {
    fn default() -> Self {
        Self {
            render_distance: 16,
            materials: [default(); 256],
        }
    }
}

impl Material for GpuTerrainUniforms {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/terrain_pipeline.wgsl".into()
    }

    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/terrain_pipeline.wgsl".into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> bevy::ecs::error::Result<(), bevy::render::render_resource::SpecializedMeshPipelineError>
    {
        let vertex_layout = layout
            .0
            .get_layout(&[
                Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
                BlockTerrainMesh::ATTRIBUTE_DATA.at_shader_location(1),
            ])
            .unwrap_or_default();
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct ChunkMaterialSingleton(Handle<GpuTerrainUniforms>);

impl FromWorld for ChunkMaterialSingleton {
    fn from_world(world: &mut bevy::ecs::world::World) -> Self {
        let mut materials = world.resource_mut::<Assets<GpuTerrainUniforms>>();
        Self(materials.add(GpuTerrainUniforms::default()))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, SystemSet)]
pub struct ChunkMaterialSet;

pub struct ChunkMaterialPlugin;

impl Plugin for ChunkMaterialPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(MaterialPlugin::<GpuTerrainUniforms>::default())
            .init_resource::<ChunkMaterialSingleton>()
            .add_systems(
                Update,
                update_chunk_material_singleton
                    .run_if(resource_changed::<BlockMaterialRegistry>)
                    .in_set(ChunkMaterialSet)
            );
    }
}

fn update_chunk_material_singleton(
    mut commands: Commands,
    mut materials: ResMut<Assets<GpuTerrainUniforms>>,
    chunk_material: Option<Res<ChunkMaterialSingleton>>,
    block_materials: Res<BlockMaterialRegistry>,
) {
    let mut needs_new_asset = false;
    let handle = chunk_material.as_ref().map(|cm| cm.0.clone());

    if let Some(ref h) = handle {
        if !materials.contains(h) {
            needs_new_asset = true;
        }
    } else {
        needs_new_asset = true;
    }

    if needs_new_asset {
        let mut gpu_mats = GpuTerrainUniforms {
            render_distance: 32,
            materials: [GpuBlockMaterial {
                base_color: Color::WHITE.into(),
                flags: 0,
                ..Default::default()
            }; 256],
        };

        for (index, material) in block_materials.iter_materials().into_iter().enumerate() {
            gpu_mats.materials[index].base_color = material.base_color.into();
            gpu_mats.materials[index].flags = material.flags.bits();
            gpu_mats.materials[index].emissive = material.emissive.into();
            gpu_mats.materials[index].perceptual_roughness = material.perceptual_roughness;
            gpu_mats.materials[index].metallic = material.metallic;
            gpu_mats.materials[index].reflectance = material.reflectance;
        }

        let new_handle = materials.add(gpu_mats);
        commands.insert_resource(ChunkMaterialSingleton(new_handle.clone()));
    } else if let Some(ref h) = handle {
        if let Some(gpu_mats) = materials.get_mut(h) {
            *gpu_mats = GpuTerrainUniforms {
                render_distance: 32,
                materials: [GpuBlockMaterial {
                    base_color: Color::WHITE.into(),
                    flags: 0,
                    ..Default::default()
                }; 256],
            };

            for (index, material) in block_materials.iter_materials().into_iter().enumerate() {
                gpu_mats.materials[index].base_color = material.base_color.into();
                gpu_mats.materials[index].flags = material.flags.bits();
                gpu_mats.materials[index].emissive = material.emissive.into();
                gpu_mats.materials[index].perceptual_roughness = material.perceptual_roughness;
                gpu_mats.materials[index].metallic = material.metallic;
                gpu_mats.materials[index].reflectance = material.reflectance;
            }
        }
    }
}
