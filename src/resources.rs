use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub current_turn_id: u32,
    pub winner: Option<u32>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            current_turn_id: 0,
            winner: None,
        }
    }        
}
