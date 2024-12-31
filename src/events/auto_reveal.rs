use bevy::prelude::*;

use crate::resources::Coordinates;

#[derive(Debug, Event)]
pub struct RevealNeighborsEvent {
    pub coordinates: Coordinates,
}
