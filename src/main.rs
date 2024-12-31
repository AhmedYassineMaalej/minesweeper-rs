use bevy::{prelude::*, window::WindowResolution};

mod components;
pub mod events;
pub mod resources;
pub mod systems;

use events::{DisplayNumberEvent, FlipTileEvent, GameStartEvent, ToggleMarkEvent};
use resources::GameState;
use systems::*;

pub const ROWS: i64 = 20;
pub const COLS: i64 = 20;

pub const TILE_SIZE: f32 = 40.0;

pub const WINDOW_WIDTH: f32 = TILE_SIZE * COLS as f32;
pub const WINDOW_HEIGHT: f32 = TILE_SIZE * ROWS as f32;

pub const MINE_COUNT: u64 = 80;

pub const FONT_SIZE: f32 = 40.0;

struct MyPlugins;
impl Plugin for MyPlugins {
    fn build(&self, app: &mut App) {
        let plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_HEIGHT, WINDOW_WIDTH),
                resizable: false,
                ..default()
            }),
            ..default()
        });
        app.add_plugins(plugins);
    }
}

fn main() {
    App::new()
        .add_plugins(MyPlugins)
        .add_event::<FlipTileEvent>()
        .add_event::<ToggleMarkEvent>()
        .add_event::<GameStartEvent>()
        .add_event::<DisplayNumberEvent>()
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
                    handle_click_tile,
                    handle_toggle_mark,
                    handle_display_number,
                )
                    .chain(),
            ),
        )
        .run();
}
