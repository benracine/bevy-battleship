use crate::components::{
    Board, Cell, CellState, Player, PlayerId, PlayerType, Ship, ShipDirection, ShipName,
};
use bevy::prelude::*;
use bevy::sprite::Sprite;
use rand::Rng;
use std::collections::HashSet;
use strum::IntoEnumIterator;

const GRAY: Srgba = Srgba::rgb(0.5, 0.5, 0.5);
const BLUE: Srgba = Srgba::rgb(0.0, 0.3, 0.6);
const RED: Srgba = Srgba::rgb(1.0, 0.0, 0.0);

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
    for player in query.iter() {
        commands.spawn(Board {
            size: UVec2::new(10, 10),
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
                    cells.push(coord);
                }
                
                let mut can_place = true;
                for &cell_coord in &cells {
                    if occupied.contains(&cell_coord) {
                        can_place = false;
                        break;
                    }
                    
                    for dx in -1i32..=1 {
                        for dy in -1i32..=1 {
                            if dx == 0 && dy == 0 { continue; }
                            
                            let check_x = cell_coord.x as i32 + dx;
                            let check_y = cell_coord.y as i32 + dy;
                            
                            if check_x >= 0 && check_y >= 0 
                                && check_x < board.size.x as i32 
                                && check_y < board.size.y as i32 {
                                let check_coord = UVec2::new(check_x as u32, check_y as u32);
                                if occupied.contains(&check_coord) {
                                    can_place = false;
                                    break;
                                }
                            }
                        }
                        if !can_place { break; }
                    }
                    if !can_place { break; }
                }
                
                if !can_place {
                    continue 'placement;
                }
                
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

//

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
                    Transform::from_translation(Vec3::new(x as f32, -(y as f32), 0.0)),
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
    cell_query: Query<(Entity, &Cell, &Transform), Without<Sprite>>,
    board_query: Query<&Board>,
    ship_query: Query<&Ship>,
) {
    let mut player_boards = std::collections::HashMap::new();
    for board in board_query.iter() {
        let board_offset = match board.owner.0 {
            0 => Vec3::new(0.0, -250.0, 0.0),  
            1 => Vec3::new(0.0, 250.0, 0.0),
            _ => Vec3::ZERO,
        };
        player_boards.insert(board.owner, board_offset);
    }

    for (entity, cell, transform) in cell_query.iter() {
        if let Ok(board) = board_query.get(cell.board) {
            let board_offset = player_boards.get(&board.owner).copied().unwrap_or(Vec3::ZERO);
            
            let is_occupied = ship_query.iter()
                .filter(|ship| ship.owner == board.owner)
                .any(|ship| ship.occupies_cell(cell.coord));
            
            let color = match cell.state {
                CellState::Empty => {
                    if is_occupied {
                        GRAY
                    } else {
                        BLUE
                    }
                }
                CellState::Hit => RED,
                CellState::Miss => Srgba::rgb(0.8, 0.8, 0.8),
            };
            
            commands
                .entity(entity)
                .insert(Sprite {
                    color: color.into(),
                    custom_size: Some(Vec2::splat(40.0)),
                    ..Default::default()
                })
                .insert(Transform::from_translation(
                    Vec3::new(
                        (transform.translation.x * 45.0) + board_offset.x - 200.0,
                        (transform.translation.y * 45.0) + board_offset.y + 200.0,
                        0.0
                    ),
                ));
        }
    }
}
