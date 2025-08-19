use bevy::prelude::*;

use crate::components::{Board, Player, PlayerType};
use crate::resources::GameState;

pub fn setup_battleship(mut commands: Commands) {
    commands.spawn((
        Player {
            id: 0,
            player_type: PlayerType::Human,
        },
        Board::new_10x10(),
    ));

    commands.spawn((
        Player {
            id: 1,
            player_type: PlayerType::Computer,
        },
        Board::new_10x10(),
    ));

    commands.insert_resource(GameState {
        current_turn_id: 0,
        winner: None,
    });
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
