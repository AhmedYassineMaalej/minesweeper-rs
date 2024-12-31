use bevy::prelude::*;

use super::{Shade, Tile};

#[derive(Debug, Resource)]
pub struct TileMaterialHandles {
    pub unflipped_light: Handle<ColorMaterial>,
    pub unflipped_dark: Handle<ColorMaterial>,
    pub flipped_light: Handle<ColorMaterial>,
    pub flipped_dark: Handle<ColorMaterial>,
    pub hover: Handle<ColorMaterial>,
    pub mine: Handle<ColorMaterial>,
    pub mark: Handle<ColorMaterial>,
}

impl TileMaterialHandles {
    pub fn new(mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        let unflipped_light = materials.add(Color::srgb_u8(10, 10, 10));
        let unflipped_dark = materials.add(Color::srgb_u8(0, 0, 0));
        let flipped_light = materials.add(Color::srgb_u8(50, 50, 50));
        let flipped_dark = materials.add(Color::srgb_u8(40, 40, 40));
        let hover = materials.add(Color::srgb(7.5, 7.5, 7.5));
        let mine = materials.add(Color::srgb_u8(245, 42, 32));
        let mark = materials.add(Color::srgb(0.0, 4.5, 0.0));

        Self {
            unflipped_light,
            unflipped_dark,
            flipped_light,
            flipped_dark,
            hover,
            mine,
            mark,
        }
    }

    pub fn get_material(&self, tile: &Tile) -> Handle<ColorMaterial> {
        if tile.contains_mine() && tile.is_flipped() {
            return self.mine.clone();
        }

        if tile.is_marked() {
            return self.mark.clone();
        }

        match tile.is_flipped() {
            true => self.get_flipped_tile_material(tile),
            false => self.get_unflipped_tile_material(tile),
        }
    }

    fn get_unflipped_tile_material(&self, tile: &Tile) -> Handle<ColorMaterial> {
        match tile.shade() {
            Shade::Light => self.unflipped_light.clone(),
            Shade::Dark => self.unflipped_dark.clone(),
        }
    }

    fn get_flipped_tile_material(&self, tile: &Tile) -> Handle<ColorMaterial> {
        match tile.shade() {
            Shade::Light => self.flipped_light.clone(),
            Shade::Dark => self.flipped_dark.clone(),
        }
    }
}
