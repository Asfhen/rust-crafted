use ilattice::{extent::Extent, glam::UVec3};
use ndshape::Shape;

#[derive(Clone)]
pub struct BlockBuffer<V, S:Shape<3, Coord = u32>>
where
    V: Copy + Clone + Default,
{
    data: Box<[V]>,
    shape: S,
}

impl<V, S: Shape<3, Coord = u32>> BlockBuffer<V, S>
where
    V: Copy + Clone + Default,
{
    #[inline]
    pub fn new(shape: S, initial_val: V) -> Self {
        Self {
            data: vec![initial_val; shape.size() as usize].into_boxed_slice(),
            shape,
        }
    }

    #[inline]
    pub fn new_empty(shape: S) -> Self {
        Self {
            data: vec![Default::default(); shape.size() as usize].into_boxed_slice(),
            shape,
        }
    }

    /// Returns the block at the queried position in local space.
    #[inline]
    pub fn block_at(&self, pos: UVec3) -> V {
        self.data[self.shape.linearize(pos.to_array()) as usize]
    }

    /// Returns a mutable reference to the block at the queried position in local space.
    #[inline]
    pub fn block_at_mut(&mut self, pos: UVec3) -> &mut V {
        &mut self.data[self.shape.linearize(pos.to_array()) as usize]
    }

    /// Fill an extent of this buffer with a specified value. 
    #[inline]
    pub fn fill_extent(&mut self, extent: Extent<UVec3>, val: V) {
        ndcopy::fill3(
            extent.shape.to_array(),
            val,
            &mut self.data,
            &self.shape,
            extent.minimum.to_array(),
        );
    }

    #[inline]
    pub const fn slice(&self) -> &[V] {
        &self.data
    }

    #[inline]
    pub const fn slice_mut(&mut self) -> &mut [V] {
        &mut self.data
    }

    #[inline]
    pub const fn shape(&self) -> &S {
        &self.shape
    }

    pub const fn shape_mut(&mut self) -> &mut S {
        &mut self.shape
    }
}

