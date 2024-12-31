use bevy::prelude::*;

use crate::resources::Coordinates;

#[derive(Event, Debug)]
pub struct ToggleMarkEvent {
    pub coordinates: Coordinates,
}
