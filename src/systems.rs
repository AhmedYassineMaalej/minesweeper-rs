use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use crate::events::{DisplayNumberEvent, FlipTileEvent, GameStartEvent, HoverTileEvent};
use crate::resources::{Coordinates, GameState, HoveredTile, Tile, TileMap, TileMaterialHandles};

#[derive(Resource, Debug)]
pub struct FontHandle(Handle<Font>);

pub fn setup_materials(mut commands: Commands, materials: ResMut<Assets<ColorMaterial>>) {
    let tile_material_handles = TileMaterialHandles::new(materials);
    commands.insert_resource(tile_material_handles);
}

pub fn load_font(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load::<Font>("FiraCode-Medium.ttf");
    commands.insert_resource(FontHandle(font));
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
    ));}

pub fn setup_tilemap(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material_handles: Res<TileMaterialHandles>,
) {
    let tile_mesh = meshes.add(Rectangle::new(40.0, 40.0));

    let width = 20;
    let height = 20;

    let mut tiles = Vec::new();

    for row in 0..height {
        let mut tile_row = Vec::new();
        for col in 0..width {
            let coordinates = Coordinates { col, row };
            let (x, y) = coordinates.get_screen_position();

            let material = material_handles.get_tile_material(&coordinates);

            let id = commands
                .spawn((
                    Mesh2d(tile_mesh.clone()),
                    MeshMaterial2d(material),
                    Transform::from_xyz(x, y, 0.0),
                ))
                .id();

            let tile = Tile {
                id,
                contains_mine: false,
                flipped: false,
                number: None,
            };

            tile_row.push(tile);
        }
        tiles.push(tile_row);
    }

    commands.insert_resource(TileMap {
        width,
        height,
        tiles,
    });
}

pub fn handle_flip_tile(
    mut query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut flip_event_reader: EventReader<FlipTileEvent>,
    mut display_number_events: EventWriter<DisplayNumberEvent>,
    mut tilemap: ResMut<TileMap>,
    material_handles: Res<TileMaterialHandles>,
) {
    let mut to_flip: Vec<Coordinates> = flip_event_reader
        .read()
        .map(|event| event.coordinates)
        .collect();

    while let Some(coordinates) = to_flip.pop() {
        if tilemap[coordinates].flipped {
            continue;
        }

        tilemap[coordinates].flipped = true;

        let mut material = material_handles.get_flipped_tile_material(&coordinates);

        if tilemap[coordinates].contains_mine {
            material = material_handles.red_mine.clone();
        }

        let tile_id = tilemap[coordinates].id;
        let mut handle = query.get_mut(tile_id).unwrap();
        handle.0 = material;

        match tilemap[coordinates].number {
            Some(number) => {
                display_number_events.send(DisplayNumberEvent {
                    number,
                    coordinates,
                });
            }
            None => {
                to_flip.extend(tilemap.get_neighbors(&coordinates));
            }
        }
    }
}

pub fn handle_display_number(
    mut commands: Commands,
    mut display_number_events: EventReader<DisplayNumberEvent>,
    font: Res<FontHandle>,
) {
    for event in display_number_events.read() {
        let &DisplayNumberEvent {
            number,
            coordinates,
        } = event;

        let textstyle = TextFont {
            font: font.0.clone(),
            font_size: 40.0,
            ..default()
        };

        let (x, y) = coordinates.get_screen_position();
        commands.spawn((
            Text2d(number.to_string()),
            textstyle,
            TextColor(event.get_color()),
            Transform::from_xyz(x, y, 1.0),
        ));
    }
}

pub fn handle_click(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut flip_events: EventWriter<FlipTileEvent>,
    mut game_start_events: EventWriter<GameStartEvent>,
    gamestate: Res<GameState>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            let col = (position.x / 40.0) as i64;
            let row = (position.y / 40.0) as i64;
            let coordinates = Coordinates { col, row };

            if *gamestate == GameState::Pending {
                game_start_events.send(GameStartEvent {
                    mouse_coords: coordinates,
                });
            }

            flip_events.send(FlipTileEvent { coordinates });
        }
    }
}

pub fn handle_game_start(
    mut game_start_events: EventReader<GameStartEvent>,
    mut bloom: Single<&mut Bloom>,
    mut gamestate: ResMut<GameState>,
    mut tilemap: ResMut<TileMap>,
) {
    let Some(GameStartEvent { mouse_coords }) = game_start_events.read().next() else {
        return;
    };
        *gamestate = GameState::Ongoing;

        tilemap.generate_mines(mouse_coords);
    
    bloom.intensity = 0.1;
    bloom.low_frequency_boost = 0.35;
}

pub fn handle_mouse_movement(
    mut hover_events: EventWriter<HoverTileEvent>,
    hovered_tile: Res<HoveredTile>,
    tilemap: Res<TileMap>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    match q_windows.single().cursor_position() {
        Some(position) => {
            let coordinates =
                Coordinates::new((position.x / 40.0) as i64, (position.y / 40.0) as i64);

            if tilemap[coordinates].flipped {
                hover_events.send(HoverTileEvent { coordinates: None });
                return;
            }

            if hovered_tile.coordinates == Some(coordinates) {
                // no need to update
                return;
            }

            hover_events.send(HoverTileEvent {
                coordinates: Some(coordinates),
            })
        }
        None => hover_events.send(HoverTileEvent { coordinates: None }),
    };
}

pub fn handle_hover_tile(
    mut hover_events: EventReader<HoverTileEvent>,
    tilemap: Res<TileMap>,
    mut hovered_tile: ResMut<HoveredTile>,
    material_handles: Res<TileMaterialHandles>,
    mut query: Query<&mut MeshMaterial2d<ColorMaterial>>,
) {
    let Some(hover_event) = hover_events.read().last() else {
        return;
    };

    // restore old tile color if it's not flipped
    if let Some(old_coordinates) = hovered_tile.coordinates {
        if !tilemap[old_coordinates].flipped {
            let tile_id = tilemap[old_coordinates].id;
            let mut handle = query.get_mut(tile_id).unwrap();
            handle.0 = material_handles.get_tile_material(&old_coordinates);
        }
    }

    hovered_tile.coordinates = hover_event.coordinates;

    let Some(coordinates) = hover_event.coordinates else {
        return;
    };

    if tilemap[coordinates].flipped {
        return;
    }

    // color new hover tile
    let tile_id = tilemap[coordinates].id;
    let mut handle = query.get_mut(tile_id).unwrap();
    handle.0 = material_handles.hover.clone();
}

