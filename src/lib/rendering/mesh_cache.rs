use crate::prelude::BlockWorldConfig;
use bevy::{platform::collections::HashMap, prelude::*};
use std::{
    marker::PhantomData,
    sync::{Arc, RwLock, Weak},
};
use weak_table::WeakValueHashMap;

#[derive(Component)]
pub(crate) struct MeshRef(pub Arc<Handle<Mesh>>);

type WeakMeshMap = WeakValueHashMap<u64, Weak<Handle<Mesh>>>;

type UserBundleMap<UB> = HashMap<u64, UB>;

#[derive(Resource, Clone)]
pub(crate) struct MeshCache<C: BlockWorldConfig> {
    mesh_handles: Arc<RwLock<WeakMeshMap>>,
    user_bundles: Arc<RwLock<UserBundleMap<C::ChunkUserBundle>>>,
    _marker: std::marker::PhantomData<C>,
}

impl<C: BlockWorldConfig> MeshCache<C> {
    pub fn apply_buffers(&self, insert_buffer: &mut MeshCacheInsertBuffer<C>) {
        if insert_buffer.is_empty() {
            return;
        }

        if let (Ok(mut mesh_handles), Ok(mut user_bundles)) =
            (self.mesh_handles.try_write(), self.user_bundles.try_write())
        {
            for (block, mesh, user_bundle) in insert_buffer.drain(..) {
                mesh_handles.insert(block, mesh);
                if let Some(user_bundle) = user_bundle {
                    user_bundles.insert(block, user_bundle);
                }
            }
            mesh_handles.remove_expired();
        }
    }

    pub fn get_mesh_handle(&self, block_hash: &u64) -> Option<Arc<Handle<Mesh>>> {
        self.mesh_handles.read().unwrap().get(block_hash)
    }

    pub fn get_mesh_map(&self) -> Arc<RwLock<WeakMeshMap>> {
        self.mesh_handles.clone()
    }

    pub fn get_user_bundle(&self, block_hash: &u64) -> Option<C::ChunkUserBundle> {
        self.user_bundles
            .read()
            .unwrap()
            .get(block_hash)
            .clone()
            .cloned()
    }
}

impl<C: BlockWorldConfig> Default for MeshCache<C> {
    fn default() -> Self {
        Self {
            mesh_handles: Arc::new(RwLock::new(WeakMeshMap::with_capacity(2000))),
            user_bundles: Arc::new(RwLock::new(UserBundleMap::with_capacity(2000))),
            _marker: std::marker::PhantomData,
        }
    }
}

type MeshHandleRef = Arc<Handle<Mesh>>;

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct MeshCacheInsertBuffer<C: BlockWorldConfig>(
    #[deref] Vec<(u64, MeshHandleRef, Option<C::ChunkUserBundle>)>,
    PhantomData<C>,
);

impl<C: BlockWorldConfig> Default for MeshCacheInsertBuffer<C> {
    fn default() -> Self {
        Self(Vec::with_capacity(1000), PhantomData)
    }
}
