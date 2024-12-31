use bevy::prelude::*;

use crate::resources::Coordinates;

#[derive(Event, Debug)]
pub struct GameStartEvent {
    mouse_coordinates: Coordinates,
}

impl GameStartEvent {
    pub fn new(mouse_coordinates: Coordinates) -> Self {
        GameStartEvent {
            mouse_coordinates,
        }
    }
    
    pub fn mouse_coordinates(&self) -> Coordinates {
        self.mouse_coordinates
    }
}
