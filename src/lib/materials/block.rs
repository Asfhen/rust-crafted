use bevy::{
    asset::{weak_handle, Asset, AssetServer, Assets, Handle},
    ecs::{
        resource::Resource,
        system::{Res, ResMut},
    },
    image::Image,
    pbr::MaterialExtension,
    reflect::TypePath,
    render::{
        mesh::{Mesh, MeshVertexAttribute, VertexAttributeDescriptor, VertexFormat},
        render_resource::{AsBindGroup, Shader, ShaderDefVal},
    },
};

#[derive(Resource)]
pub(crate) struct LoadingTexture {
    pub is_loaded: bool,
    pub handle: Handle<Image>,
}

#[derive(Resource)]
pub(crate) struct TextureLayers(pub u32);

pub const BLOCK_TEXTURE_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("ec8b2b83-93ee-49c1-8600-7a0cb97b6fd9");

pub const ATTRIBUTE_TEX_INDEX: MeshVertexAttribute =
    MeshVertexAttribute::new("texture_index", 989640910, VertexFormat::Uint32x3);

pub fn vertex_layout() -> Vec<VertexAttributeDescriptor> {
    vec![
        Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
        Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
        Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
        Mesh::ATTRIBUTE_COLOR.at_shader_location(5),
        Mesh::ATTRIBUTE_COLOR.at_shader_location(7),
        ATTRIBUTE_TEX_INDEX.at_shader_location(8),
    ]
}

#[derive(Asset, AsBindGroup, Debug, Clone, TypePath)]
pub(crate) struct StandardBlockMaterial {
    #[texture(100, dimension = "2d_array")]
    #[sampler(101)]
    pub blocks_texture: Handle<Image>,
}

impl MaterialExtension for StandardBlockMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        BLOCK_TEXTURE_SHADER_HANDLE.into()
    }

    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        BLOCK_TEXTURE_SHADER_HANDLE.into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialExtensionPipeline,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialExtensionKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        if descriptor
            .vertex
            .shader_defs
            .contains(&ShaderDefVal::Bool("PREPASS_PIPELINE".into(), true))
        {
            return Ok(());
        }

        let vertex_layout = layout.0.get_layout(&vertex_layout())?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

pub(crate) fn prepare_texture(
    asset_server: Res<AssetServer>,
    texture_layers: Res<TextureLayers>,
    mut loading_texture: ResMut<LoadingTexture>,
    mut images: ResMut<Assets<Image>>,
) {
    if loading_texture.is_loaded
        || !matches!(
            asset_server.get_load_state(loading_texture.handle.clone().id()),
            Some(bevy::asset::LoadState::Loaded)
        )
    {
        return;
    }
    loading_texture.is_loaded = true;

    let image = images.get_mut(&loading_texture.handle).unwrap();
    image.reinterpret_stacked_2d_as_array(texture_layers.0);
}
