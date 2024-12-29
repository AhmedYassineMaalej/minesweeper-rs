use bevy::prelude::*;

use crate::resources::Coordinates;

#[derive(Event, Debug)]
pub struct GameStartEvent {
    pub mouse_coords: Coordinates,
}
