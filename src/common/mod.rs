pub mod generation;
pub mod world;

pub mod error;
pub mod logging;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U32Position(u32);

impl U32Position {
    const X_BITS: u32 = 12;
    const Z_BITS: u32 = 12;
    const Y_BITS: u32 = 8;

    const X_MASK: u32 = (1 << Self::X_BITS) - 1;
    const Z_MASK: u32 = (1 << Self::Z_BITS) - 1;
    const Y_MASK: u32 = (1 << Self::Y_BITS) - 1;

    const Z_SHIFT: u32 = Self::Y_BITS;
    const X_SHIFT: u32 = Self::Y_BITS + Self::Z_BITS;

    pub fn new(x: u16, y: u8, z: u16) -> Self {
        let x_val = (x as u32) & Self::X_MASK;
        let z_val = (z as u32) & Self::Z_MASK;
        let y_val = (y as u32) & Self::Y_MASK;

        let encoded = (x_val << Self::X_SHIFT) | (z_val << Self::Z_SHIFT) | y_val;

        Self(encoded)
    }

    pub fn from_xyz(x: u16, y: u8, z: u16) -> Self {
        Self::new(x, y, z)
    }

    pub fn x(&self) -> u16 {
        ((self.0 & Self::X_MASK) >> Self::X_SHIFT) as u16
    }

    pub fn y(&self) -> u8 {
        (self.0 & Self::Y_MASK) as u8
    }

    pub fn z(&self) -> u16 {
        ((self.0 & Self::Z_MASK) >> Self::Z_SHIFT) as u16
    }

    pub fn set_x(&mut self, x: u16) {
        self.0 = (self.0 & !Self::X_MASK) | ((x as u32) << Self::X_SHIFT);
    }

    pub fn set_y(&mut self, y: u8) {
        self.0 = (self.0 & !Self::Y_MASK) | (y as u32);
    }

    pub fn set_z(&mut self, z: u16) {
        self.0 = (self.0 & !Self::Z_MASK) | ((z as u32) << Self::Z_SHIFT);
    }

    pub fn set_xyz(&mut self, x: u16, y: u8, z: u16) {
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    // Métodos para manipulação direta do chunk
    pub fn chunk_coords(&self, chunk_size: u16) -> (u16, u8, u16) {
        (
            self.x() / chunk_size,
            self.y() / (chunk_size as u8),
            (self.z() as u16) / chunk_size,
        )
    }

    pub fn local_coords(&self, chunk_size: u16) -> (u16, u8, u16) {
        (
            self.x() % chunk_size,
            self.y() % chunk_size as u8,
            self.z() % chunk_size,
        )
    }
}

// Implementação para conversão de/para tupla
impl From<(u16, u8, u16)> for U32Position {
    fn from((x, y, z): (u16, u8, u16)) -> Self {
        Self::from_xyz(x, y, z)
    }
}

impl From<U32Position> for (u16, u8, u16) {
    fn from(pos: U32Position) -> Self {
        (pos.x(), pos.y(), pos.z())
    }
}

// Implementação de serialização/deserialização para rede
impl U32Position {
    pub fn to_bits(&self) -> u32 {
        self.0
    }

    pub fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U64Position(u64);

impl U64Position {
    const X_BITS: u64 = 21;
    const Z_BITS: u64 = 21;
    const Y_BITS: u64 = 22;

    const X_MASK: u64 = (1 << Self::X_BITS) - 1;
    const Z_MASK: u64 = (1 << Self::Z_BITS) - 1;
    const Y_MASK: u64 = (1 << Self::Y_BITS) - 1;

    const X_SHIFT: u64 = Self::Y_BITS + Self::Z_BITS;
    const Z_SHIFT: u64 = Self::Y_BITS;

    pub fn new(x: u32, y: u32, z: u32) -> Self {
        let x_val = (x as u64) & Self::X_MASK;
        let z_val = (z as u64) & Self::Z_MASK;
        let y_val = (y as u64) & Self::Y_MASK;

        let encoded = (x_val << Self::X_SHIFT) | (z_val << Self::Z_SHIFT) | y_val;

        Self(encoded)
    }

    pub fn from_xyz(x: u32, y: u32, z: u32) -> Self {
        Self::new(x, y, z)
    }

    pub fn from_u32(x: u32, y: u32, z: u32) -> Self {
        Self::new(x, y, z)
    }

    pub fn x(&self) -> u32 {
        ((self.0 & Self::X_MASK) >> Self::X_SHIFT) as u32
    }

    pub fn y(&self) -> u32 {
        (self.0 & Self::Y_MASK) as u32
    }

    pub fn z(&self) -> u32 {
        ((self.0 & Self::Z_MASK) >> Self::Z_SHIFT) as u32
    }

    pub fn set_x(&mut self, x: u32) {
        self.0 = (self.0 & !(Self::X_MASK << Self::X_SHIFT)) | ((x as u64) << Self::X_SHIFT);
    }

    pub fn set_y(&mut self, y: u32) {
        self.0 = (self.0 & !Self::Y_MASK) | (y as u64);
    }

    pub fn set_z(&mut self, z: u32) {
        self.0 = (self.0 & !(Self::Z_MASK << Self::Z_SHIFT)) | ((z as u64) << Self::Z_SHIFT);
    }

    pub fn set_xyz(&mut self, x: u32, y: u32, z: u32) {
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    pub fn chunk_coords(&self, chunk_size: u32) -> (u32, u32, u32) {
        (
            self.x() / chunk_size,
            self.y() / chunk_size,
            self.z() / chunk_size,
        )
    }

    pub fn local_coords(&self, chunk_size: u32) -> (u32, u32, u32) {
        (
            self.x() % chunk_size,
            self.y() % chunk_size,
            self.z() % chunk_size,
        )
    }
}

impl From<(u32, u32, u32)> for U64Position {
    fn from((x, y, z): (u32, u32, u32)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<U64Position> for (u32, u32, u32) {
    fn from(pos: U64Position) -> Self {
        (pos.x(), pos.y(), pos.z())
    }
}

impl U64Position {
    pub fn to_bits(&self) -> u64 {
        self.0
    }

    pub fn from_bits(bits: u64) -> Self {
        Self(bits)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I32Position(i32);

impl I32Position {
    const X_BITS: i32 = 12;
    const Y_BITS: i32 = 8;
    const Z_BITS: i32 = 12;

    const X_MASK: i32 = (1 << Self::X_BITS) - 1;
    const Y_MASK: i32 = (1 << Self::Y_BITS) - 1;
    const Z_MASK: i32 = (1 << Self::Z_BITS) - 1;

    const X_SHIFT: i32 = Self::Y_BITS + Self::Z_BITS;
    const Z_SHIFT: i32 = Self::Y_BITS;
    const Y_SHIFT: i32 = 0; // stupid zero is here just to prove a point that it does not need a shift to be applied, because my brain is not working today

    const X_MASK_SHIFTED: i32 = Self::X_MASK << Self::X_SHIFT;
    const Y_MASK_SHIFTED: i32 = Self::Y_MASK << Self::Y_SHIFT;
    const Z_MASK_SHIFTED: i32 = Self::Z_MASK << Self::Z_SHIFT;

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let mut value: i32 = 0;

        let mut apply = |val: i32, mask: i32, shift: i32| {
            value |= (val & mask) << shift;
        };

        apply(x, Self::X_MASK, Self::X_SHIFT);
        apply(y, Self::Y_MASK, 0);
        apply(z, Self::Z_MASK, Self::Z_SHIFT);

        Self(value as i32)
    }

    pub fn from_xyz(x: i32, y: i32, z: i32) -> Self {
        Self::new(x, y, z)
    }

    pub fn x(&self) -> i32 {
        self.extract_coord(Self::X_SHIFT, Self::X_MASK)
    }

    pub fn y(&self) -> i32 {
        self.extract_coord(Self::Y_SHIFT, Self::Y_MASK)
    }

    pub fn z(&self) -> i32 {
        self.extract_coord(Self::Z_SHIFT, Self::Z_MASK)
    }

    pub fn set_x(&mut self, x: i32) {
        self.set_coord(x, Self::X_SHIFT, Self::X_MASK, Self::X_MASK_SHIFTED)
    }

    pub fn set_y(&mut self, y: i32) {
        self.set_coord(y, Self::Y_SHIFT, Self::Y_MASK, Self::Y_MASK_SHIFTED)
    }

    pub fn set_z(&mut self, z: i32) {
        self.set_coord(z, Self::Z_SHIFT, Self::Z_MASK, Self::Z_MASK_SHIFTED)
    }

    // Métodos internos
    fn extract_coord(&self, shift: i32, mask: i32) -> i32 {
        let raw = (self.0 >> shift) & mask;
        Self::sign_extend(raw, mask.count_ones() as i32)
    }

    fn set_coord(&mut self, value: i32, shift: i32, mask: i32, shifted_mask: i32) {
        // Limpa a área atual
        self.0 &= !shifted_mask;

        // Aplica o novo valor
        let val = (value & mask) << shift;
        self.0 |= val;
    }

    // Extensão de sinal para valores negativos
    fn sign_extend(value: i32, bits: i32) -> i32 {
        let sign_bit = 1 << (bits - 1);
        let mask = !((1 << bits) - 1);

        if (value & sign_bit) != 0 {
            (value | mask) as i32
        } else {
            value as i32
        }
    }

    pub fn chunk_coords(&self, chunk_size: u16) -> (i32, i32, i32) {
        let cs = chunk_size as i32;
        (
            self.x().div_euclid(cs),
            self.y().div_euclid(cs),
            self.z().div_euclid(cs),
        )
    }

    pub fn local_coords(&self, chunk_size: u16) -> (i32, i32, i32) {
        let cs = chunk_size as i32;
        (
            self.x().rem_euclid(cs),
            self.y().rem_euclid(cs),
            self.z().rem_euclid(cs),
        )
    }

    // Operações com bits
    pub fn to_bits(&self) -> i32 {
        self.0
    }

    pub fn from_bits(bits: i32) -> Self {
        Self(bits)
    }
}

impl From<(i32, i32, i32)> for I32Position {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<I32Position> for (i32, i32, i32) {
    fn from(pos: I32Position) -> Self {
        (pos.x(), pos.y(), pos.z())
    }
}

impl From<[i32; 3]> for I32Position {
    fn from([x, y, z]: [i32; 3]) -> Self {
        Self::new(x, y, z)
    }
}

impl From<I32Position> for [i32; 3] {
    fn from(pos: I32Position) -> Self {
        [pos.x(), pos.y(), pos.z()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct I64Position(i64);

impl I64Position {
    // Deslocamentos de bits
    const X_SHIFT: i64 = 43;
    const Z_SHIFT: i64 = 22;
    const Y_SHIFT: i64 = 0;

    // Máscaras de bits
    const X_MASK: i64 = 0x1FFFFF; // 21 bits
    const Z_MASK: i64 = 0x1FFFFF; // 21 bits
    const Y_MASK: i64 = 0x3FFFFF; // 22 bits

    // Máscaras com deslocamento
    const X_MASK_SHIFTED: i64 = Self::X_MASK << Self::X_SHIFT;
    const Z_MASK_SHIFTED: i64 = Self::Z_MASK << Self::Z_SHIFT;
    const Y_MASK_SHIFTED: i64 = Self::Y_MASK << Self::Y_SHIFT;

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let mut value: i64 = 0;

        // Função para aplicar máscara e deslocamento
        let apply = |val: i32, mask: i64, shift: i64| ((val as i64) & mask) << shift;

        value |= apply(x, Self::X_MASK, Self::X_SHIFT);
        value |= apply(z, Self::Z_MASK, Self::Z_SHIFT);
        value |= apply(y, Self::Y_MASK, Self::Y_SHIFT);

        Self(value)
    }

    pub fn from_xyz(x: i32, y: i32, z: i32) -> Self {
        Self::new(x, y, z)
    }

    pub fn x(&self) -> i32 {
        self.extract_coord(Self::X_SHIFT, Self::X_MASK)
    }

    pub fn y(&self) -> i32 {
        self.extract_coord(Self::Y_SHIFT, Self::Y_MASK)
    }

    pub fn z(&self) -> i32 {
        self.extract_coord(Self::Z_SHIFT, Self::Z_MASK)
    }

    pub fn set_x(&mut self, x: i32) {
        self.set_coord(x, Self::X_SHIFT, Self::X_MASK, Self::X_MASK_SHIFTED)
    }

    pub fn set_y(&mut self, y: i32) {
        self.set_coord(y, Self::Y_SHIFT, Self::Y_MASK, Self::Y_MASK_SHIFTED)
    }

    pub fn set_z(&mut self, z: i32) {
        self.set_coord(z, Self::Z_SHIFT, Self::Z_MASK, Self::Z_MASK_SHIFTED)
    }

    // Métodos internos
    fn extract_coord(&self, shift: i64, mask: i64) -> i32 {
        let raw = (self.0 >> shift) & mask;
        Self::sign_extend(raw, mask.count_ones() as i64)
    }

    fn set_coord(&mut self, value: i32, shift: i64, mask: i64, shifted_mask: i64) {
        // Limpa a área atual
        self.0 &= !shifted_mask;

        // Aplica o novo valor
        let val = ((value as i64) & mask) << shift;
        self.0 |= val;
    }

    // Extensão de sinal para valores negativos
    fn sign_extend(value: i64, bits: i64) -> i32 {
        let sign_bit = 1 << (bits - 1);
        let mask = !((1 << bits) - 1);

        if (value & sign_bit) != 0 {
            (value | mask) as i32
        } else {
            value as i32
        }
    }

    // Conversão para/from chunks
    pub fn chunk_coords(&self, chunk_size: u16) -> (i32, i32, i32) {
        let cs = chunk_size as i32;
        (
            self.x().div_euclid(cs),
            self.y().div_euclid(cs),
            self.z().div_euclid(cs),
        )
    }

    pub fn local_coords(&self, chunk_size: u16) -> (i32, i32, i32) {
        let cs = chunk_size as i32;
        (
            self.x().rem_euclid(cs),
            self.y().rem_euclid(cs),
            self.z().rem_euclid(cs),
        )
    }

    // Operações com bits
    pub fn to_bits(&self) -> i64 {
        self.0
    }

    pub fn from_bits(bits: i64) -> Self {
        Self(bits)
    }
}

// Implementações de conversão
impl From<(i32, i32, i32)> for I64Position {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<I64Position> for (i32, i32, i32) {
    fn from(pos: I64Position) -> Self {
        (pos.x(), pos.y(), pos.z())
    }
}

impl From<[i32; 3]> for I64Position {
    fn from([x, y, z]: [i32; 3]) -> Self {
        Self::new(x, y, z)
    }
}

impl From<I64Position> for [i32; 3] {
    fn from(pos: I64Position) -> Self {
        [pos.x(), pos.y(), pos.z()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_roundtrip() {
        let pos = I64Position::new(123456, -7890, -987654);
        assert_eq!(pos.x(), 123456);
        assert_eq!(pos.y(), -7890);
        assert_eq!(pos.z(), -987654);

        let bits = pos.to_bits();
        let new_pos = I64Position::from_bits(bits);
        assert_eq!(pos, new_pos);
    }

    #[test]
    fn test_negative_values() {
        let mut pos = I64Position::new(-500000, -1000000, -500000);
        assert_eq!(pos.x(), -500000);
        assert_eq!(pos.y(), -1000000);
        assert_eq!(pos.z(), -500000);

        pos.set_x(999999);
        pos.set_y(-111111);
        pos.set_z(0);

        assert_eq!(pos.x(), 999999);
        assert_eq!(pos.y(), -111111);
        assert_eq!(pos.z(), 0);
    }

    #[test]
    fn test_chunk_conversion() {
        let pos = I64Position::new(35, -5, 70);
        let chunk = pos.chunk_coords(16);
        assert_eq!(chunk, (2, -1, 4));

        let local = pos.local_coords(16);
        assert_eq!(local, (3, 11, 6));
    }

    #[test]
    fn test_edge_cases() {
        // Testa valores nos limites
        let max_x = (1 << 20) - 1; // 1.048.575
        let min_x = -(1 << 20); // -1.048.576
        let max_y = (1 << 21) - 1; // 2.097.151
        let min_y = -(1 << 21); // -2.097.152

        let pos1 = I64Position::new(max_x, max_y, 0);
        assert_eq!(pos1.x(), max_x);
        assert_eq!(pos1.y(), max_y);

        let pos2 = I64Position::new(min_x, min_y, 0);
        assert_eq!(pos2.x(), min_x);
        assert_eq!(pos2.y(), min_y);
    }

    #[test]
    fn test_conversion_traits() {
        let pos = I64Position::from((100, -200, 300));
        let tuple: (i32, i32, i32) = pos.into();
        assert_eq!(tuple, (100, -200, 300));

        let pos = I64Position::from([-100, 200, -300]);
        let array: [i32; 3] = pos.into();
        assert_eq!(array, [-100, 200, -300]);
    }
}
