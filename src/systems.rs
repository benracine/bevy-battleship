
use std::collections::HashSet;
use bevy::prelude::*;
use rand::Rng;
use strum::IntoEnumIterator;
use crate::components::{
    Board, Cell, CellState, Player, PlayerId, PlayerType, Ship, ShipDirection, ShipName, Transform,
};

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

pub fn spawn_ships(mut commands: Commands, query: Query<&Board>) {
    for board in query.iter() {
        let mut occupied = HashSet::new();
        let mut rng = rand::rng();
        for ship_name in ShipName::iter() {
            let ship_length = ship_name.length() as u32;
            'placement: loop {
                let direction = if rng.random_bool(0.5) {
                    ShipDirection::Horizontal
                } else {
                    ShipDirection::Vertical
                };
                let (max_x, max_y) = match direction {
                    ShipDirection::Horizontal => (board.size.x - ship_length, board.size.y - 1),
                    ShipDirection::Vertical => (board.size.x - 1, board.size.y - ship_length),
                };
                let start_x = rng.random_range(0..=max_x);
                let start_y = rng.random_range(0..=max_y);
                let mut cells = Vec::new();
                for i in 0..ship_length {
                    let coord = match direction {
                        ShipDirection::Horizontal => UVec2::new(start_x + i, start_y),
                        ShipDirection::Vertical => UVec2::new(start_x, start_y + i),
                    };
                    if occupied.contains(&coord) {
                        continue 'placement;
                    }
                    cells.push(coord);
                }
                // All cells are free, place the ship
                for &coord in &cells {
                    occupied.insert(coord);
                }
                commands.spawn(Ship {
                    name: ship_name,
                    owner: board.owner,
                    direction,
                    cells,
                });
                break;
            }
        }
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
