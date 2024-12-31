use bevy::prelude::*;
mod components;
pub mod events;
mod plugins;
pub mod resources;
pub mod systems;

use events::{DisplayNumberEvent, FlipTileEvent, GameStartEvent, RevealNeighborsEvent, ToggleMarkEvent};
use plugins::MinesweeperPlugins;
use resources::GameState;
use systems::*;

pub const ROWS: i64 = 15;
pub const COLS: i64 = 15;

pub const TILE_SIZE: f32 = 40.0;

pub const WINDOW_WIDTH: f32 = TILE_SIZE * COLS as f32;
pub const WINDOW_HEIGHT: f32 = TILE_SIZE * ROWS as f32;

pub const MINE_COUNT: u64 = 50;

pub const FONT_SIZE: f32 = 35.0;

fn main() {
    App::new()
        .add_plugins(MinesweeperPlugins)
        .add_event::<FlipTileEvent>()
        .add_event::<ToggleMarkEvent>()
        .add_event::<GameStartEvent>()
        .add_event::<DisplayNumberEvent>()
        .add_event::<RevealNeighborsEvent>()
        .init_resource::<GameState>()
        .add_systems(
            Startup,
            (
                (
                    (setup_materials, setup_mesh),
                    (setup_tilemap, spawn_hover_tile),
                )
                    .chain(),
                setup_camera,
                load_font,
            ),
        )
        .add_systems(
            Update,
            (
                handle_mouse_movement,
                (
                    handle_click,
                    handle_game_start,
                    handle_auto_reveal,
                    handle_flip_tile,
                    handle_toggle_mark,
                    handle_display_number,
                )
                    .chain(),
            ),
        )
        .run();
}
