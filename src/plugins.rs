use bevy::{prelude::*, window::{EnabledButtons, WindowResolution}};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct MinesweeperPlugins;
impl Plugin for MinesweeperPlugins {
    fn build(&self, app: &mut App) {
        let plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        });
        app.add_plugins(plugins);
    }
}