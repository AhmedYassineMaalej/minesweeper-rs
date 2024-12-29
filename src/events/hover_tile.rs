use bevy::prelude::*;

use crate::resources::Coordinates;

#[derive(Event, Debug)]
pub struct HoverTileEvent {
    pub coordinates: Option<Coordinates>,
}
