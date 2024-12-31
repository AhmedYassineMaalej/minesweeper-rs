use bevy::{math::Vec2, prelude::{Component, Transform}};

use crate::{TILE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Component)]
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

    pub fn get_transform(&self, z: f32) -> Transform  {
        let x = -WINDOW_WIDTH / 2.0 + (self.col as f32) * TILE_SIZE + TILE_SIZE * 0.5;
        let y = WINDOW_HEIGHT / 2.0 - (self.row as f32) * TILE_SIZE - TILE_SIZE * 0.5;

        Transform::from_xyz(x, y, z)
    }

    pub fn from_screen_position(position: Vec2) -> Self {
        let col = (position.x / TILE_SIZE) as i64;
        let row = (position.y / TILE_SIZE) as i64;
        Self { col, row }
    }
}

impl From<Transform> for Coordinates {
    fn from(transform: Transform) -> Self {
        let x = transform.translation.x;
        let y = transform.translation.y;

        let col = (x - TILE_SIZE * 0.5 + WINDOW_WIDTH / 2.0) / TILE_SIZE;
        let row = (y + TILE_SIZE * 0.5 - WINDOW_HEIGHT / 2.0) / TILE_SIZE;

        Self {
            col: col as i64,
            row: row as i64,
        }
    }
}