use crate::{
    materials::block::{
        prepare_texture, LoadingTexture, StandardBlockMaterial, TextureLayers,
        BLOCK_TEXTURE_SHADER_HANDLE,
    },
    prelude::{BlockWorldConfig, DefaultWorld},
    world::{
        internals::Internals,
        world::{ChunkWillDespawn, ChunkWillRemesh, ChunkWillSpawn, ChunkWillUpdate},
    },
};
use bevy::{
    asset::{load_internal_asset, RenderAssetUsages},
    image::{CompressedImageFormats, ImageSampler, ImageType},
    pbr::ExtendedMaterial,
    prelude::*,
};

#[derive(Resource)]
pub struct BlockWorldMaterialHandle<M: Material> {
    pub handle: Handle<M>,
}

pub struct BlockWorldPlugin<C, M = StandardMaterial>
where
    C: BlockWorldConfig,
    M: Material,
{
    spawn_meshes: bool,
    use_custom_material: bool,
    config: C,
    material: M,
}

impl<C> BlockWorldPlugin<C, StandardMaterial>
where
    C: BlockWorldConfig,
{
    pub fn with_config(config: C) -> Self {
        Self {
            config,
            spawn_meshes: true,
            use_custom_material: false,
            material: StandardMaterial::default(),
        }
    }

    pub fn minimal() -> Self {
        Self {
            spawn_meshes: false,
            use_custom_material: false,
            config: C::default(),
            material: StandardMaterial::default(),
        }
    }
}

impl<C, M> BlockWorldPlugin<C, M>
where
    C: BlockWorldConfig,
    M: Material,
{
    pub fn with_material<CustomMaterial: Material>(
        self,
        material: CustomMaterial,
    ) -> BlockWorldPlugin<C, CustomMaterial> {
        BlockWorldPlugin {
            spawn_meshes: self.spawn_meshes,
            use_custom_material: true,
            config: self.config,
            material,
        }
    }
}

impl Default for BlockWorldPlugin<DefaultWorld, StandardMaterial> {
    fn default() -> Self {
        Self {
            spawn_meshes: true,
            use_custom_material: false,
            config: DefaultWorld,
            material: StandardMaterial::default(),
        }
    }
}

impl<C, M> Plugin for BlockWorldPlugin<C, M>
where
    C: BlockWorldConfig,
    M: Material,
{
    fn build(&self, app: &mut App) {
        app.init_resource::<C>()
            .add_systems(PreStartup, Internals::<C>::setup)
            .add_systems(
                PreUpdate,
                (
                    (
                        (Internals::<C>::spawn_chunks, Internals::<C>::retire_chunks).chain(),
                        Internals::<C>::remesh_dirty_chunks,
                    )
                        .chain(),
                    (
                        Internals::<C>::flush_block_write_buffer,
                        Internals::<C>::despawn_retired_chunks,
                        (
                            Internals::<C>::flush_chunk_map_buffers,
                            Internals::<C>::flush_mesh_cache_buffers,
                        ),
                    )
                        .chain(),
                ),
            )
            .add_event::<ChunkWillSpawn<C>>()
            .add_event::<ChunkWillDespawn<C>>()
            .add_event::<ChunkWillRemesh<C>>()
            .add_event::<ChunkWillUpdate<C>>();

        if self.spawn_meshes {
            load_internal_asset!(
                app,
                BLOCK_TEXTURE_SHADER_HANDLE,
                "shaders/block_texture.wgsl",
                Shader::from_wgsl
            );

            app.add_systems(Update, Internals::<C>::spawn_meshes);
        }

        if !self.use_custom_material && self.spawn_meshes {
            let mat_plugins = app.get_added_plugins::<MaterialPlugin<
                ExtendedMaterial<StandardMaterial, StandardBlockMaterial>,
            >>();

            if mat_plugins.is_empty() {
                app.add_plugins(MaterialPlugin::<
                    ExtendedMaterial<StandardMaterial, StandardBlockMaterial>,
                >::default());
            }

            let mut preloaded_texture = true;
            let texture_conf = self.config.block_texture();
            let mut texture_layers = 0;

            let image_handle = if texture_conf.is_none() {
                let mut image = Image::from_buffer(
                    include_bytes!("shaders/default_texture.png"),
                    ImageType::MimeType("image/png"),
                    CompressedImageFormats::default(),
                    false,
                    ImageSampler::Default,
                    RenderAssetUsages::default(),
                )
                .unwrap();
                image.reinterpret_stacked_2d_as_array(4);
                let mut image_assets = app.world_mut().resource_mut::<Assets<Image>>();
                image_assets.add(image)
            } else {
                let (img_path, layers) = texture_conf.unwrap();
                texture_layers = layers;
                let asset_server = app.world().get_resource::<AssetServer>().unwrap();
                preloaded_texture = false;
                asset_server.load(img_path)
            };

            let mut material_assets = app
                .world_mut()
                .resource_mut::<Assets<ExtendedMaterial<StandardMaterial, StandardBlockMaterial>>>(
                );

            let mat_handle = material_assets.add(ExtendedMaterial {
                base: StandardMaterial {
                    reflectance: 0.05,
                    metallic: 0.05,
                    perceptual_roughness: 0.95,
                    ..default()
                },
                extension: StandardBlockMaterial {
                    blocks_texture: image_handle.clone(),
                },
            });

            app.insert_resource(LoadingTexture {
                is_loaded: preloaded_texture,
                handle: image_handle,
            });
            app.insert_resource(BlockWorldMaterialHandle { handle: mat_handle });
            app.insert_resource(TextureLayers(texture_layers));

            app.add_systems(Update, prepare_texture);

            app.add_systems(
                Update,
                Internals::<C>::assign_material::<
                    ExtendedMaterial<StandardMaterial, StandardBlockMaterial>,
                >,
            );
        }

        if self.use_custom_material {
            if self.config.init_custom_materials() {
                let mut custom_material_assets = app.world_mut().resource_mut::<Assets<M>>();
                let handle = custom_material_assets.add(self.material.clone());
                app.insert_resource(BlockWorldMaterialHandle { handle });
            }

            app.insert_resource(LoadingTexture {
                is_loaded: true,
                handle: Handle::default(),
            });

            app.add_systems(Update, Internals::<C>::assign_material::<M>);
        }

        app.insert_resource(self.config.clone());
    }
}
