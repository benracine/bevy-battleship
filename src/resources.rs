use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameState {
    pub current_turn_id: u32,
    pub winner: Option<u32>,
}

