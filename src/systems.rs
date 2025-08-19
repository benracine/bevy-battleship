use bevy::prelude::*;

use crate::components::{Board, Cell, CellState, Player, PlayerId, PlayerType, Transform};
use crate::resources::GameState;

pub fn setup_battleship(mut commands: Commands) {
    // Create two players
    let player1_id = PlayerId(0);
    let player2_id = PlayerId(1);

    let _player1 = commands
        .spawn(Player {
            id: player1_id,
            name: "Player 1".to_string(),
            player_type: PlayerType::Human,
        })
        .id();

    let _player2 = commands
        .spawn(Player {
            id: player2_id,
            name: "Player 2".to_string(),
            player_type: PlayerType::Computer,
        })
        .id();

    // Create boards for each player
    let board_size = UVec2::new(10, 10);
    let board1 = commands
        .spawn(Board {
            size: board_size,
            owner: player1_id,
        })
        .id();
    let board2 = commands
        .spawn(Board {
            size: board_size,
            owner: player2_id,
        })
        .id();

    // Spawn cells for each board
    for y in 0..board_size.y {
        for x in 0..board_size.x {
            let coord = UVec2::new(x, y);
            commands.spawn((
                Cell {
                    coord,
                    state: CellState::Empty,
                    board: board1,
                },
                Transform {
                    position: Vec2::new(x as f32, y as f32),
                },
            ));
            commands.spawn((
                Cell {
                    coord,
                    state: CellState::Empty,
                    board: board2,
                },
                Transform {
                    position: Vec2::new(x as f32, y as f32),
                },
            ));
        }
    }

    commands.insert_resource(GameState {
        current_turn_id: 0,
        winner: None,
    });
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
