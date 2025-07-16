/// Representação compacta de posição de bloco em 64 bits
/// Layout: 21 bits para X, 22 bits para Y, 21 bits para Z (total 64 bits)
/// Suporta coordenadas negativas usando sign extension
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Position(i64);

impl Position {
    // Deslocamentos de bits
    const X_SHIFT: i64 = 43;
    const Z_SHIFT: i64 = 22;
    const Y_SHIFT: i64 = 0;
    
    // Máscaras de bits
    const X_MASK: i64 = 0x1FFFFF; // 21 bits
    const Y_MASK: i64 = 0x3FFFFF; // 22 bits
    const Z_MASK: i64 = 0x1FFFFF; // 21 bits

    /// Cria uma nova posição a partir de coordenadas i32
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        // Aplica máscara e desloca para a posição correta
        let x_val = ((x as i64) & Self::X_MASK) << Self::X_SHIFT;
        let y_val = ((y as i64) & Self::Y_MASK) << Self::Y_SHIFT;
        let z_val = ((z as i64) & Self::Z_MASK) << Self::Z_SHIFT;
        
        Self(x_val | y_val | z_val)
    }
    
    /// Extrai coordenada X com extensão de sinal
    pub fn x(&self) -> i32 {
        Self::sign_extend((self.0 >> Self::X_SHIFT) & Self::X_MASK, 21)
    }
    
    /// Extrai coordenada Y com extensão de sinal
    pub fn y(&self) -> i32 {
        Self::sign_extend((self.0 >> Self::Y_SHIFT) & Self::Y_MASK, 22)
    }
    
    /// Extrai coordenada Z com extensão de sinal
    pub fn z(&self) -> i32 {
        Self::sign_extend((self.0 >> Self::Z_SHIFT) & Self::Z_MASK, 21)
    }
    
    /// Função auxiliar para extensão de sinal
    fn sign_extend(value: i64, bits: i64) -> i32 {
        let sign_bit = 1 << (bits - 1);
        if value & sign_bit != 0 {
            (value | !((1 << bits) - 1)) as i32
        } else {
            value as i32
        }
    }
    
    /// Converte para coordenadas de chunk
    pub fn to_chunk_coords(&self, chunk_size: u16) -> (i32, i32, i32) {
        let cs = chunk_size as i32;
        (self.x().div_euclid(cs), self.y().div_euclid(cs), self.z().div_euclid(cs))
    }
}

// Implementações de conversão para facilitar o uso
impl From<(i32, i32, i32)> for Position {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self::new(x, y, z)
    }
}