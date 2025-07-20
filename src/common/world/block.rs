use block_mesh::{MergeVoxel, Voxel};

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct Block(pub u64);

impl Block {
    pub const EMPTY_BLOCK: Self = Self(0);
}

impl Default for Block {
    fn default() -> Self {
        Self::EMPTY_BLOCK
    }
}

impl Voxel for Block {
    #[inline]
    fn get_visibility(&self) -> block_mesh::VoxelVisibility {
        match *self {
            Self::EMPTY_BLOCK => block_mesh::VoxelVisibility::Empty,
            _ => block_mesh::VoxelVisibility::Opaque,
        }
    }
}

impl MergeVoxel for Block {
    type MergeValue = u64;

    #[inline]
    fn merge_value(&self) -> Self::MergeValue {
        self.0
    }
}

pub trait BlockMaterial: MergeVoxel + Voxel {
    fn as_mat_id(&self) -> u64;
}

impl BlockMaterial for Block {
    fn as_mat_id(&self) -> u64 {
        self.0
    }
}
