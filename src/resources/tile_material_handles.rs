use bevy::prelude::*;

use super::Coordinates;

#[derive(Debug, Resource)]
pub struct TileMaterialHandles {
    pub unflipped_1: Handle<ColorMaterial>,
    pub unflipped_2: Handle<ColorMaterial>,
    pub hover: Handle<ColorMaterial>,
    pub flipped_1: Handle<ColorMaterial>,
    pub flipped_2: Handle<ColorMaterial>,
    pub red_mine: Handle<ColorMaterial>,
}

impl TileMaterialHandles {
    pub fn new(mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        let unflipped_1 = materials.add(Color::srgb_u8(10, 10, 10));
        let unflipped_2 = materials.add(Color::srgb_u8(0, 0, 0));
        let flipped_1 = materials.add(Color::srgb_u8(50, 50, 50));
        let flipped_2 = materials.add(Color::srgb_u8(40, 40, 40));
        let hover = materials.add(Color::srgb(7.5, 7.5, 7.5));
        let red_mine = materials.add(Color::srgb_u8(245, 42, 32));

        Self {
            unflipped_1,
            unflipped_2,
            hover,
            flipped_1,
            flipped_2,
            red_mine,
        }
    }

    pub fn get_tile_material(&self, coordinates: &Coordinates) -> Handle<ColorMaterial> {
        let parity = (coordinates.row + coordinates.col) % 2;
        match parity {
            0 => self.unflipped_1.clone(),
            1 => self.unflipped_2.clone(),
            _ => unreachable!(),
        }
    }

    pub fn get_flipped_tile_material(&self, coordinates: &Coordinates) -> Handle<ColorMaterial> {
        let parity = (coordinates.row + coordinates.col) % 2;
        match parity {
            0 => self.flipped_1.clone(),
            1 => self.flipped_2.clone(),
            _ => unreachable!(),
        }
    }
}
