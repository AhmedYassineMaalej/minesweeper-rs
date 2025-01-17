use std::f32::consts::PI;

use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::window::WindowCloseRequested;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::Hover;
use crate::events::{
    DisplayNumberEvent, FlipTileEvent, GameStartEvent, RevealNeighborsEvent, SpawnEffectsEvent,
    ToggleMarkEvent,
};
use crate::resources::{Coordinates, GameState, MeshHandles, TileMap, TileMaterialHandles};
use crate::{COLS, FONT_PATH, FONT_SIZE, ROWS, TILE_SIZE};

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
    let font = asset_server.load::<Font>(FONT_PATH);
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

pub fn handle_flip_tile(
    mut query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut flip_event_reader: EventReader<FlipTileEvent>,
    mut flip_effects_writer: EventWriter<SpawnEffectsEvent>,
    mut display_number_events: EventWriter<DisplayNumberEvent>,
    mut tilemap: ResMut<TileMap>,
    material_handles: Res<TileMaterialHandles>,
) {
    let mut to_flip: Vec<Coordinates> = flip_event_reader
        .read()
        .map(|event| event.coordinates)
        .collect();

    while let Some(coordinates) = to_flip.pop() {
        let tile = &mut tilemap[coordinates];

        if !tile.flip() {
            continue;
        }

        flip_effects_writer.send(SpawnEffectsEvent { coordinates });

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
    mut auto_reveal_events: EventWriter<RevealNeighborsEvent>,
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

    if buttons.just_pressed(MouseButton::Middle) {
        auto_reveal_events.send(RevealNeighborsEvent { coordinates });
    }
}

pub fn handle_window_close(
    mut window_close_events: EventReader<WindowCloseRequested>,
    mut exit: EventWriter<AppExit>,
) {
    if let Some(_event) = window_close_events.read().next() {
        exit.send(AppExit::Success);
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

pub fn handle_auto_reveal(
    mut reveal_neighbors_events: EventReader<RevealNeighborsEvent>,
    mut flip_events: EventWriter<FlipTileEvent>,
    tilemap: Res<TileMap>,
) {
    for event in reveal_neighbors_events.read() {
        let coordinates = event.coordinates;
        let tile = &tilemap[coordinates];
        let neighbours = tilemap.get_neighbors(&coordinates);

        let mark_count = neighbours
            .iter()
            .filter(|&neighbour| tilemap[*neighbour].is_marked())
            .count();

        if tile.number() != Some(mark_count) {
            continue;
        }

        for neighbor in tilemap.get_neighbors(&coordinates) {
            flip_events.send(FlipTileEvent::new(neighbor));
        }
    }
}

#[derive(Component)]
pub struct ParticleVelocity {
    velocity: Vec2,
}

pub fn handle_spawn_effects(
    mut commands: Commands,
    meshes: Res<MeshHandles>,
    materials: Res<TileMaterialHandles>,
    mut event_reader: EventReader<SpawnEffectsEvent>,
) {
    for SpawnEffectsEvent { coordinates } in event_reader.read() {
        let mut transform = coordinates.get_transform(2.0);
        transform.translation.x += (rand::random::<f32>() - 0.5) * TILE_SIZE;
        transform.translation.y += (rand::random::<f32>() - 0.5) * TILE_SIZE;
        transform.scale = Vec3::splat(rand::random::<f32>() * 0.5 + 0.2);

        let angle = PI * rand::random::<f32>();
        let velocity = ParticleVelocity {
            velocity: Vec2::from_angle(angle) * 200.0,
        };

        commands.spawn((
            Mesh2d(meshes.tile_mesh.clone()),
            MeshMaterial2d(materials.hover.clone()),
            transform,
            velocity,
        ));
    }
}

pub fn update_particles(
    mut particles: Query<(&mut ParticleVelocity, &mut Transform)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut velocity, mut transform) in particles.iter_mut() {
        transform.translation += velocity.velocity.extend(0.0) * dt;

        velocity.velocity += Vec2::Y * -600.0 * dt;
    }
}
