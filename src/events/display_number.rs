use bevy::prelude::*;

use crate::resources::Coordinates;

#[derive(Event, Debug)]
pub struct DisplayNumberEvent {
    pub number: usize,
    pub coordinates: Coordinates,
}

impl DisplayNumberEvent {
    pub fn get_color(&self) -> Color {
        match self.number {
            1 => Color::srgb(10.0, 0.0, 0.0),
            2 => Color::srgb(0.0, 10.0, 0.0),
            3 => Color::srgb(0.0, 0.0, 10.0),
            4 => Color::srgb(7.5, 0.0, 7.5),
            5 => Color::srgb(0.0, 7.5, 7.5),
            6 => Color::srgb(7.5, 7.5, 0.0),
            7 => Color::srgb(7.5, 3.5, 3.5),
            8 => Color::srgb(4.0, 6.0, 8.5),
            _ => unreachable!()
        }
    }
}