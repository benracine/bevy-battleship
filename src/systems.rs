use bevy::prelude::*;

use crate::components::{Board, Cell, CellState, Player, PlayerId, PlayerType, Transform};

pub fn spawn_players(mut commands: Commands) {
    commands.spawn(Player {
        id: PlayerId(0),
        name: "Player 1".to_string(),
        player_type: PlayerType::Human,
    });
    commands.spawn(Player {
        id: PlayerId(1),
        name: "Player 2".to_string(),
        player_type: PlayerType::Computer,
    });
}

pub fn spawn_boards(mut commands: Commands, query: Query<&Player>) {
    let board_size = UVec2::new(10, 10);
    for player in query.iter() {
        commands.spawn(Board {
            size: board_size,
            owner: player.id,
        });
    }
}

pub fn spawn_cells(mut commands: Commands, query: Query<(Entity, &Board)>) {
    for (board_entity, board) in query.iter() {
        for y in 0..board.size.y {
            for x in 0..board.size.x {
                let coord = UVec2::new(x, y);
                commands.spawn((
                    Cell {
                        coord,
                        state: CellState::Empty,
                        board: board_entity,
                    },
                    Transform {
                        position: Vec2::new(x as f32, y as f32),
                    },
                ));
            }
        }
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
