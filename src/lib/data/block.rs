use bevy::math::Vec3;
use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

pub const BLOCK_SIZE: f32 = 1.;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WorldBlock<I = u8> {
    #[default]
    Unset,
    Air,
    Solid(I),
    Transparent(I),
}

impl<I: PartialEq> WorldBlock<I> {
    pub fn is_unset(&self) -> bool {
        *self == Self::Unset
    }

    pub fn is_air(&self) -> bool {
        *self == Self::Air
    }

    pub fn is_solid(&self) -> bool {
        matches!(self, Self::Solid(_))
    }
}

impl<I: PartialEq> Voxel for WorldBlock<I> {
    fn get_visibility(&self) -> VoxelVisibility {
        match self {
            WorldBlock::Unset => VoxelVisibility::Empty,
            WorldBlock::Air => VoxelVisibility::Empty,
            WorldBlock::Solid(_) => VoxelVisibility::Opaque,
            WorldBlock::Transparent(_) => VoxelVisibility::Translucent,
        }
    }
}

impl<I: PartialEq + Eq + Default + Copy> MergeVoxel for WorldBlock<I>  {
    type MergeValue = I;

    fn merge_value(&self) -> Self::MergeValue {
        match self {
            WorldBlock::Solid(v) => *v,
            WorldBlock::Transparent(v) => *v,
            _ => I::default(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BlockFace {
    None,
    Bottom,
    Top,
    Left,
    Right,
    Back,
    Forward,
}

impl TryFrom<BlockFace> for Vec3 {
    type Error = ();

    fn try_from(value: BlockFace) -> Result<Self, Self::Error> {
        match value {
            BlockFace::None => Err(()),
            BlockFace::Bottom => Ok(-Vec3::Y),
            BlockFace::Top => Ok(Vec3::Y),
            BlockFace::Left => Ok(-Vec3::X),
            BlockFace::Right => Ok(Vec3::X),
            BlockFace::Back => Ok(-Vec3::Z),
            BlockFace::Forward => Ok(Vec3::Z),
        }
    }
}
