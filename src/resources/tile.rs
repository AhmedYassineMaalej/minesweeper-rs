use bevy::prelude::*;

#[derive(Debug)]
pub struct Tile {
    pub id: Entity,
    pub contains_mine: bool,
    pub flipped: bool,
    pub number: Option<usize>,
}