use bevy::prelude::*;

use crate::resources::Coordinates;

#[derive(Event, Debug)]
pub struct SpawnEffectsEvent {
    pub coordinates: Coordinates,
}
