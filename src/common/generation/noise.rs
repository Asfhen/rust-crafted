use bevy::math::{IVec3, Vec2, Vec2Swizzles, Vec3Swizzles};
use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    Fbm, MultiFractal, SuperSimplex,
};

pub fn rand2to1(p: Vec2, dot: Vec2) -> f32 {
    let sp: Vec2 = p.to_array().map(f32::sin).into();
    let random = sp.dot(dot);
    (random.sin() * 143_758.55).fract()
}

pub fn rand2to1i(vec: Vec2) -> f32 {
    let mut p3 = (vec.xyx() * 0.39).fract();
    p3 += p3.dot(p3.yzx());
    (p3.x + p3.y) * p3.z.fract()
}


#[inline(always)]
pub fn rand2to2(p: Vec2) -> Vec2 {
    Vec2::new(
        rand2to1(p, Vec2::new(12.989, 78.233)),
        rand2to1(p, Vec2::new(39.346, 11.135)),
    )
}

pub fn voronoi(p: Vec2) -> Vec2 {
    const NEIGHBOR_RANGE: i32 = 2;

    let base_cel = p.floor();
    let mut closest_point = base_cel;
    let mut min_distance = 1f32;

    for x in -NEIGHBOR_RANGE..=NEIGHBOR_RANGE {
        for y in -NEIGHBOR_RANGE..=NEIGHBOR_RANGE {
            let cell = base_cel + Vec2::new(x as f32, y as f32);
            let cell_pos = cell + rand2to2(cell);
            let distance = (cell_pos - p).length_squared();

            if distance < min_distance {
                min_distance = distance;
                closest_point = cell;
            }
        }
    }

    closest_point
}

pub fn generate_height_map(key: IVec3, chunk_len: usize) -> Vec<f32> {
    let noise = Fbm::<SuperSimplex>::new(0)
        .set_octaves(4)
        .set_frequency(0.005)
        .set_persistence(0.5)
        .set_lacunarity(2.0);

    PlaneMapBuilder::new(noise)
        .set_size(chunk_len, chunk_len)
        .set_x_bounds(key.x as f64, (key.x + chunk_len as i32) as f64)
        .set_y_bounds(key.y as f64, (key.y + chunk_len as i32) as f64)
        .build()
        .into_iter()
        .map(|x| x.mul_add(20f64, 132f64) as f32)
        .collect()
}

#[derive(Clone, Copy)]
pub struct HeightMap<'a, const W: usize, const H: usize> {
    slice: &'a [f32],
}

impl<'a, const W: usize, const H: usize> HeightMap<'a, W, H> {
    #[inline]
    pub fn get(&self, pos: [u32; 2]) -> u32 {
        self.slice[pos[1] as usize * W + pos[0] as usize].round() as u32
    }

    #[inline]
    pub const fn from_slice(slice: &'a [f32]) -> Self {
        Self { slice }
    }
}
