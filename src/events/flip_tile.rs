use bevy::prelude::*;

use crate::resources::Coordinates;

#[derive(Event, Debug)]
pub struct FlipTileEvent {
    pub coordinates: Coordinates,
}

impl FlipTileEvent {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }
}
