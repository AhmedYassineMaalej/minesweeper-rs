use bevy::prelude::*;

#[derive(Resource)]
pub struct MeshHandles {
    pub tile_mesh: Handle<Mesh>,
}