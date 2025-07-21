use block_mesh::{MergeVoxel, Voxel};

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct Block(pub u64);

impl Block {
    pub const EMPTY_BLOCK: Self = Self(0);
    pub const TRANSPARENT_FLAG: u64 = 0x8000_0000_0000_0000;

    pub fn new_opaque(mat_id: u64) -> Self {
        assert!(mat_id != 0, "Material ID 0 is reserved for EMPTY_BLOCK");
        Self(mat_id)
    }

    pub fn new_transparent(mat_id: u64) -> Self {
        assert!(mat_id != 0, "Material ID 0 is reserved for EMPTY_BLOCK");
        Self(mat_id | Self::TRANSPARENT_FLAG)
    }

    /// Returns true if the block is empty.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns true if the block is transparent.
    pub fn is_transparent(&self) -> bool {
        (self.0 & Self::TRANSPARENT_FLAG) != 0 && !self.is_empty()
    }

    /// Returns true if the block is opaque.
    pub fn is_opaque(&self) -> bool {
        !self.is_empty() && !self.is_transparent()
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::EMPTY_BLOCK
    }
}

impl Voxel for Block {
    #[inline]
    fn get_visibility(&self) -> block_mesh::VoxelVisibility {
        if self.is_empty() {
            block_mesh::VoxelVisibility::Empty
        } else if self.is_transparent() {
            block_mesh::VoxelVisibility::Translucent
        } else {
            block_mesh::VoxelVisibility::Opaque
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

pub trait MaterialBlock: MergeVoxel + Voxel {
    fn as_mat_id(&self) -> u64;
}

impl MaterialBlock for Block {
    fn as_mat_id(&self) -> u64 {
        self.0 & !Block::TRANSPARENT_FLAG
    }
}
