use bevy::{prelude::*, window::WindowResolution};

pub mod events;
pub mod resources;
pub mod systems;

use events::{DisplayNumberEvent, FlipTileEvent, GameStartEvent, HoverTileEvent};
use resources::{GameState, HoveredTile};
use systems::*;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

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
        .add_event::<GameStartEvent>()
        .add_event::<HoverTileEvent>()
        .add_event::<DisplayNumberEvent>()
        .insert_resource(GameState::Pending)
        .insert_resource(HoveredTile { coordinates: None })
        .add_systems(PreStartup, setup_materials)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_tilemap)
        .add_systems(Startup, load_font)
        .add_systems(
            Update,
            (
                (handle_mouse_movement, handle_hover_tile).chain(),
                (
                    handle_click,
                    handle_game_start,
                    handle_flip_tile,
                    handle_display_number,
                )
                    .chain(),
            ),
        )
        .run();
}
