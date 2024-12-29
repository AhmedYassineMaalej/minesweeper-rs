use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coordinates {
    pub col: i64,
    pub row: i64,
}

impl Coordinates {
    pub fn new(col: i64, row: i64) -> Self {
        Self { col, row }
    }

    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    pub fn get_screen_position(&self) -> (f32, f32) {
        let x = -WINDOW_WIDTH / 2.0 + (self.col as f32) * 40.0 + 20.0;
        let y = WINDOW_HEIGHT / 2.0 - (self.row as f32) * 40.0 - 20.0;

        (x, y)
    }
}
