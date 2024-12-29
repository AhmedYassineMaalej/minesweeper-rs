use bevy::prelude::*;

use super::Coordinates;

#[derive(Resource, Debug, Default)]
pub struct HoveredTile {
    pub coordinates: Option<Coordinates>,
}
