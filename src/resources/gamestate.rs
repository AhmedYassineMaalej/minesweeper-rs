use bevy::prelude::*;

#[derive(Debug, Resource, PartialEq)]
pub enum GameState {
    Pending,
    Ongoing,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Pending
    }
}