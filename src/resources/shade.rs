#[derive(Debug, Clone, Copy)]
pub enum Shade {
    Light,
    Dark,
}

impl Shade {
    pub fn from_coordinates(row: i64, col: i64) -> Self {
        match (row + col) % 2 {
            0 => Self::Light,
            1 => Self::Dark,
            _ => unreachable!(),
        }
    }
}
