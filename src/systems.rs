use crate::components::{
    Board, Cell, CellState, Player, PlayerId, PlayerType, Ship, ShipDirection, ShipName,
};
use bevy::prelude::*;
use bevy::sprite::Sprite;
use rand::Rng;
use std::collections::HashSet;
use strum::IntoEnumIterator;

// Define the CellTransform component that was missing
#[derive(Component)]
pub struct CellTransform {
    pub position: Vec2,
}

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
                let direction = if rng.random() {
                    // Fixed: use random() instead of gen_bool
                    ShipDirection::Horizontal
                } else {
                    ShipDirection::Vertical
                };
                let (max_x, max_y) = match direction {
                    ShipDirection::Horizontal => (board.size.x - ship_length, board.size.y - 1),
                    ShipDirection::Vertical => (board.size.x - 1, board.size.y - ship_length),
                };
                let start_x = rng.random_range(0..=max_x); // Fixed: use random_range
                let start_y = rng.random_range(0..=max_y); // Fixed: use random_range
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
                    CellTransform {
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

pub fn render_boards(
    mut commands: Commands,
    cell_query: Query<(Entity, &Cell, &CellTransform), Without<Sprite>>,
    board_query: Query<&Board>,
) {
    // Get board positions - player 1 on left, player 2 on right
    let mut player_boards = std::collections::HashMap::new();
    for board in board_query.iter() {
        let board_offset = match board.owner.0 {
            0 => Vec2::new(-6.0, 0.0), // Player 1 board on left
            1 => Vec2::new(6.0, 0.0),  // Player 2 board on right
            _ => Vec2::ZERO,
        };
        player_boards.insert(board.owner, board_offset);
    }

    // Render cells for each board
    for (entity, cell, cell_transform) in cell_query.iter() {
        // Get the board this cell belongs to
        if let Ok(board) = board_query.get(cell.board) {
            let board_offset = player_boards.get(&board.owner).copied().unwrap_or(Vec2::ZERO);
            
            let color = match cell.state {
                CellState::Empty => Srgba::rgb(0.7, 0.7, 1.0),
                CellState::Occupied(_) => Srgba::rgb(0.3, 0.3, 0.8),
                CellState::Hit => Srgba::RED,
                CellState::Miss => Srgba::rgb(0.5, 0.5, 0.5),
            };
            
            commands
                .entity(entity)
                .insert(Sprite {
                    color: color.into(),
                    custom_size: Some(Vec2::splat(0.9)),
                    ..Default::default()
                })
                .insert(Transform::from_translation(
                    (cell_transform.position + board_offset).extend(0.0),
                ));
        }
    }
}
