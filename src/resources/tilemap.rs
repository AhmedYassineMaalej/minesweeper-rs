use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::ops::{Index, IndexMut};

use crate::resources::Coordinates;
use crate::MINE_COUNT;

use super::{Shade, Tile};

#[derive(Debug, Resource)]
pub struct TileMap {
    pub width: i64,
    pub height: i64,
    pub tiles: Vec<Vec<Tile>>,
}

impl Index<Coordinates> for TileMap {
    type Output = Tile;

    fn index(&self, coordinates: Coordinates) -> &Self::Output {
        let col = coordinates.col as usize;
        let row = coordinates.row as usize;
        &self.tiles[row][col]
    }
}

impl IndexMut<Coordinates> for TileMap {
    fn index_mut(&mut self, coordinates: Coordinates) -> &mut Self::Output {
        let col = coordinates.col as usize;
        let row = coordinates.row as usize;

        &mut self.tiles[row][col]
    }
}

impl TileMap {
    pub fn new(commands: &mut Commands, width: i64, height: i64) -> Self {
        let mut tiles = Vec::new();
        for row in 0..height {
            let mut tile_row = Vec::new();
            for col in 0..width {
                let shade = Shade::from_coordinates(row, col);
                let tile = Tile::new(commands, shade);
                tile_row.push(tile);
            }

            tiles.push(tile_row);
        }

        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn generate_mines(&mut self, mouse_coordinates: &Coordinates) {
        let mine_count = MINE_COUNT;

        let mut mines = 0;

        while mines < mine_count {
            let mine_coords = Coordinates::new(
                thread_rng().gen_range(0..self.width),
                thread_rng().gen_range(0..self.height),
            );

            if mine_coords.manhattan_distance(mouse_coordinates) <= 3 {
                continue;
            }

            let tile = &mut self[mine_coords];

            if tile.set_mine() {
                mines += 1;
            }
        }

        self.update_tile_numbers();
    }

    fn update_tile_numbers(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let coordinates = Coordinates::new(col, row);
                let mine_count = self
                    .get_neighbors(&coordinates)
                    .into_iter()
                    .filter(|&neighbor| self[neighbor].contains_mine())
                    .count();

                self[coordinates].set_number(mine_count);
            }
        }
    }

    pub fn get_neighbors(&self, coordinates: &Coordinates) -> Vec<Coordinates> {
        let mut neighbors = Vec::new();

        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                let neighbor_row = coordinates.row + row_offset;
                let neighbor_col = coordinates.col + col_offset;

                if neighbor_col < 0 || neighbor_col >= self.width {
                    continue;
                }

                if neighbor_row < 0 || neighbor_row >= self.height {
                    continue;
                }

                neighbors.push(Coordinates::new(neighbor_col, neighbor_row));
            }
        }

        neighbors
    }

    pub fn can_reveal_neighbors(&self, coordinates: Coordinates) -> bool {
        let tile = &self[coordinates];

        let mut mark_count = 0;
        for neighbour in self.get_neighbors(&coordinates) {
            if self[neighbour].is_marked() {
                mark_count += 1;
            }
        }

        tile.number() == Some(mark_count)
    }
}
