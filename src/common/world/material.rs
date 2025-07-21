use bevy::{
    platform::collections::HashMap,
    prelude::{Color, Plugin, Resource},
};
use bitflags::bitflags;
use std::any::TypeId;
use tracing::info;

#[derive(Default)]
pub struct MaterialRegistryInfo {
    pub id: &'static str,
    pub namespace: &'static str,
    pub name: &'static str,
    pub variant: Option<&'static str>,
    pub base_color: Color,
    pub flags: BlockMaterialFlags,
    pub emissive: Color,
    pub perceptual_roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
}


pub trait BlockMaterial {
    /// Namespace for the owner of the block. default: rust_crafted
    fn namespace() -> &'static str {
        "rust_crafted"
    }
    fn block_name() -> &'static str;
    fn variant() -> Option<&'static str> {
        None
    }

    // --- Visual and Physical Properties ---
    fn base_color() -> Color;
    fn flags() -> BlockMaterialFlags;
    fn emissive() -> Color {
        Color::WHITE
    }
    fn perceptual_roughness() -> f32 {
        0.8
    }
    fn metallic() -> f32 {
        0.0
    }
    fn reflectance() -> f32 {
        0.5
    }

    fn id_string() -> String {
        match Self::variant() {
            Some(variant) => format!("{}::{}::{}", Self::namespace(), Self::block_name(), variant),
            None => format!("{}::{}", Self::namespace(), Self::block_name()),
        }
    }
}

/// Defines a new block material type with optional property overrides.
///
/// Any properties not specified will use the default from the `BlockMaterial` trait.
/// The compiler will enforce that all required properties (like `block_name`,
/// `base_color`, and `flags`) are provided.
///
/// # Example
///
/// ```rust,ignore
/// use crate::common::world::material::{block_material, BlockMaterialFlags};
/// use bevy::prelude::Color;
///
/// block_material!(Stone {
///     block_name: "stone",
///     base_color: Color::GRAY,
///     flags: BlockMaterialFlags::SOLID,
///     reflectance: 0.2, // Optional override
/// });
/// ```
#[macro_export]
macro_rules! block_material {
    ($type_name:ident { $($field:ident: $value:expr),* $(,)? }) => {
        pub struct $type_name;

        impl $crate::common::world::material::BlockMaterial for $type_name {
            $(
                $crate::__block_material_fn!($field: $value);
            )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __block_material_fn {
    (block_name: $value:expr) => {
        fn block_name() -> &'static str {
            $value
        }
    };
    (namespace: $value:expr) => {
        fn namespace() -> &'static str {
            $value
        }
    };
    (variant: $value:expr) => {
        fn variant() -> Option<&'static str> {
            $value
        }
    };
    (base_color: $value:expr) => {
        fn base_color() -> bevy::prelude::Color {
            $value
        }
    };
    (flags: $value:expr) => {
        fn flags() -> $crate::common::world::material::BlockMaterialFlags {
            $value
        }
    };
    (emissive: $value:expr) => {
        fn emissive() -> bevy::prelude::Color {
            $value
        }
    };
    (perceptual_roughness: $value:expr) => {
        fn perceptual_roughness() -> f32 {
            $value
        }
    };
    (metallic: $value:expr) => {
        fn metallic() -> f32 {
            $value
        }
    };
    (reflectance: $value:expr) => {
        fn reflectance() -> f32 {
            $value
        }
    };
}

bitflags! {
    pub struct BlockMaterialFlags : u32 {
        const SOLID = 0;
        const LIQUID = 1 << 1;
        const UNBREAKABLE = 1 << 2;
        const TRANSPARENT = 1 << 3;
    }
}

impl Default for BlockMaterialFlags {
    fn default() -> Self {
        Self::SOLID
    }
}

/// A registry for block material types.
/// This stores the block materials along their material id used to refer
/// them in block data
#[derive(Resource)]
pub struct BlockMaterialRegistry {
    materials: Vec<MaterialRegistryInfo>,
    mat_by_id: HashMap<String, usize>,
    mat_by_typeid: HashMap<TypeId, usize>,
}

impl BlockMaterialRegistry {
    #[inline]
    pub fn get_by_id(&self, id: u64) -> Option<&MaterialRegistryInfo> {
        self.materials.get(id as usize)
    }

    #[inline]
    pub fn get_by_id_mut(&mut self, id: u64) -> Option<&mut MaterialRegistryInfo> {
        self.materials.get_mut(id as usize)
    }

    pub fn get_by_type<M: 'static>(&self) -> Option<&MaterialRegistryInfo> {
        self.mat_by_typeid
            .get(&TypeId::of::<M>())
            .map(|id| self.materials.get(*id).unwrap())
    }

    pub fn get_id_for_type<M: 'static>(&self) -> Option<u64> {
        self.mat_by_typeid
            .get(&TypeId::of::<M>())
            .map(|x| *x as u64)
    }

    pub fn register<M: 'static + BlockMaterial>(&mut self) {
        let type_id = TypeId::of::<M>();
        if self.mat_by_typeid.contains_key(&type_id) {
            panic!(
                "Material type {} has already been registered.",
                std::any::type_name::<M>()
            );
        }

        let id_string = M::id_string();
        if self.mat_by_id.contains_key(&id_string) {
            panic!(
                "A material with ID '{}' has already been registered.",
                id_string
            );
        }

        let numeric_id = self.materials.len();
        let id: &'static str = Box::leak(id_string.clone().into_boxed_str());

        let info = MaterialRegistryInfo {
            id,
            namespace: M::namespace(),
            name: M::block_name(),
            variant: M::variant(),
            base_color: M::base_color(),
            flags: M::flags(),
            emissive: M::emissive(),
            perceptual_roughness: M::perceptual_roughness(),
            metallic: M::metallic(),
            reflectance: M::reflectance(),
        };

        self.materials.push(info);
        info!("Registered material {:?} (ID: {})", id, numeric_id);
        self.mat_by_id.insert(id_string, numeric_id);
        self.mat_by_typeid.insert(type_id, numeric_id);
    }

    pub fn iter_materials(&self) -> impl IntoIterator<Item = &MaterialRegistryInfo> {
        self.materials.iter()
    }
}

impl Default for BlockMaterialRegistry {
    fn default() -> Self {
        let mut registry = Self {
            materials: Vec::default(),
            mat_by_id: HashMap::default(),
            mat_by_typeid: HashMap::default(),
        };

        registry.register::<Air>();

        registry
    }
}

block_material!(Air {
    block_name: "air",
    base_color: Color::NONE,
    flags: BlockMaterialFlags::TRANSPARENT,
});

pub struct BlockMaterialPlugin;
impl Plugin for BlockMaterialPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<BlockMaterialRegistry>();
    }
}
