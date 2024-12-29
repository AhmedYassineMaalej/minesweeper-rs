use bevy::prelude::*;

#[derive(Debug, Resource, PartialEq)]
pub enum GameState {
    Pending,
    Ongoing,
}
