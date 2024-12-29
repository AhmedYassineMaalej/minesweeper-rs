use bevy::prelude::*;

use crate::resources::Coordinates;

#[derive(Event, Debug)]
pub struct FlipTileEvent {
    pub coordinates: Coordinates,
}
