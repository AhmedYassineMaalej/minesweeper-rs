use bevy::prelude::*;

use super::Shade;

#[derive(Debug)]
pub struct Tile {
    id: Entity,
    shade: Shade,
    mine: bool,
    flipped: bool,
    number: Option<usize>,
    mark: bool,
}

impl Tile {
    pub fn new(commands: &mut Commands, shade: Shade) -> Self {
        Self {
            id: commands.spawn_empty().id(),
            mine: false,
            flipped: false,
            mark: false,
            number: None,
            shade,
        }
    }

    pub fn is_hoverable(&self) -> bool {
        !self.flipped && !self.mark
    }

    /// adds mine to the tile.
    ///
    /// returns `true` if the tile did not previously have a mine
    /// `false` otherwise.
    pub fn set_mine(&mut self) -> bool {
        if self.mine {
            return false;
        }

        self.mine = true;
        true
    }

    pub fn flip(&mut self) -> bool {
        if self.mark || self.flipped {
            return false;
        }

        self.flipped = true;
        true
    }

    pub fn is_flipped(&self) -> bool {
        self.flipped
    }

    pub fn contains_mine(&self) -> bool {
        self.mine
    }

    pub fn id(&self) -> Entity {
        self.id
    }

    pub fn number(&self) -> Option<usize> {
        self.number
    }

    pub fn set_number(&mut self, number: usize) {
        if number == 0 {
            return;
        }

        self.number = Some(number);
    }

    pub fn toggle_mark(&mut self) -> bool {
        if self.flipped {
            return false;
        }

        self.mark = !self.mark;
        true
    }

    pub fn shade(&self) -> Shade {
        self.shade
    }

    pub fn is_marked(&self) -> bool {
        self.mark
    }
}
