use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::Hover;
use crate::events::{DisplayNumberEvent, FlipTileEvent, GameStartEvent, ToggleMarkEvent};
use crate::resources::{Coordinates, GameState, MeshHandles, TileMap, TileMaterialHandles};
use crate::{COLS, FONT_SIZE, ROWS, TILE_SIZE};

#[derive(Resource, Debug)]
pub struct FontHandle(Handle<Font>);

pub fn setup_materials(mut commands: Commands, materials: ResMut<Assets<ColorMaterial>>) {
    let tile_material_handles = TileMaterialHandles::new(materials);
    commands.insert_resource(tile_material_handles);
}

pub fn setup_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let tile_mesh = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));
    commands.insert_resource(MeshHandles { tile_mesh });
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
    ));
}

pub fn setup_tilemap(
    mut commands: Commands,
    meshes: Res<MeshHandles>,
    material_handles: Res<TileMaterialHandles>,
) {
    let tilemap = TileMap::new(&mut commands, COLS, ROWS);

    for row in 0..ROWS {
        for col in 0..COLS {
            let coordinates = Coordinates::new(col, row);
            let tile = &tilemap[coordinates];
            let id = tile.id();

            let mut entity = commands.get_entity(id).unwrap();

            let material = material_handles.get_material(tile);
            let transform = coordinates.get_transform(0.0);

            entity.insert((
                Mesh2d(meshes.tile_mesh.clone()),
                MeshMaterial2d(material),
                transform,
            ));
        }
    }

    commands.insert_resource(tilemap);
}

pub fn handle_click_tile(
    mut query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut click_event_reader: EventReader<FlipTileEvent>,
    mut display_number_events: EventWriter<DisplayNumberEvent>,
    mut tilemap: ResMut<TileMap>,
    material_handles: Res<TileMaterialHandles>,
) {
    let mut to_flip: Vec<Coordinates> = click_event_reader
        .read()
        .map(|event| event.coordinates)
        .collect();

    while let Some(coordinates) = to_flip.pop() {
        let tile = &mut tilemap[coordinates];

        if !tile.flip() {
            continue;
        }

        let material_handle = material_handles.get_material(tile);

        let mut mesh = query.get_mut(tile.id()).unwrap();
        mesh.0 = material_handle;

        match tile.number() {
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

        let textfont = TextFont {
            font: font.0.clone(),
            font_size: FONT_SIZE,
            ..default()
        };

        let transform = coordinates.get_transform(1.0);
        commands.spawn((
            Text2d(number.to_string()),
            textfont,
            TextColor(event.get_color()),
            transform,
        ));
    }
}

pub fn handle_click(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut flip_events: EventWriter<FlipTileEvent>,
    mut mark_events: EventWriter<ToggleMarkEvent>,
    mut game_start_events: EventWriter<GameStartEvent>,
    gamestate: Res<GameState>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let Some(mouse_pos) = q_windows.single().cursor_position() else {
        return;
    };

    let coordinates = Coordinates::from_screen_position(mouse_pos);

    if buttons.just_pressed(MouseButton::Left) {
        if *gamestate == GameState::Pending {
            game_start_events.send(GameStartEvent::new(coordinates));
        }

        flip_events.send(FlipTileEvent { coordinates });
    }

    if buttons.just_pressed(MouseButton::Right) {
        mark_events.send(ToggleMarkEvent { coordinates });
    }
}

pub fn handle_game_start(
    mut game_start_events: EventReader<GameStartEvent>,
    mut bloom: Single<&mut Bloom>,
    mut gamestate: ResMut<GameState>,
    mut tilemap: ResMut<TileMap>,
) {
    let Some(event) = game_start_events.read().next() else {
        return;
    };
    *gamestate = GameState::Ongoing;

    let mouse_coordinates = event.mouse_coordinates();
    tilemap.generate_mines(&mouse_coordinates);

    bloom.intensity = 0.1;
    bloom.low_frequency_boost = 0.35;
}

pub fn handle_mouse_movement(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut query_hover: Query<(&mut Transform, &mut Visibility), With<Hover>>,
    tilemap: Res<TileMap>,
) {
    let (mut transform, mut visibility) = query_hover.single_mut();
    let cursor_pos = q_windows.single().cursor_position();

    // check if cursor is outside window
    if cursor_pos.is_none() {
        *visibility = Visibility::Hidden;
        return;
    }

    let cursor_pos = cursor_pos.unwrap();
    let cursor_coordinates = Coordinates::new(
        (cursor_pos.x / TILE_SIZE) as i64,
        (cursor_pos.y / TILE_SIZE) as i64,
    );

    if !tilemap[cursor_coordinates].is_hoverable() {
        *visibility = Visibility::Hidden;
        return;
    }

    *transform = cursor_coordinates.get_transform(1.0);
    *visibility = Visibility::Visible;
}

pub fn spawn_hover_tile(
    mut commands: Commands,
    mesh_handles: Res<MeshHandles>,
    material_handles: Res<TileMaterialHandles>,
) {
    let tile_mesh = mesh_handles.tile_mesh.clone();
    let transform = Coordinates::new(0, 0).get_transform(1.0);

    commands.spawn((
        Hover,
        Mesh2d(tile_mesh),
        MeshMaterial2d(material_handles.hover.clone()),
        transform,
        Visibility::Hidden,
    ));
}

pub fn handle_toggle_mark(
    mut mark_events: EventReader<ToggleMarkEvent>,
    mut tilemap: ResMut<TileMap>,
    mut query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    material_handles: Res<TileMaterialHandles>,
) {
    for event in mark_events.read() {
        let coordinates = event.coordinates;
        let tile = &mut tilemap[coordinates];

        if !tile.toggle_mark() {
            continue;
        }

        query.get_mut(tile.id()).unwrap().0 = material_handles.get_material(tile);
    }
}
