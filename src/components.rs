//! ECS components for Battleship
use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerType {
    Human,
    Computer,
}

#[derive(Component)]
pub struct Player {
    pub id: u32,
    pub player_type: PlayerType,
}

#[derive(Component)]
pub struct Board {
    pub cells: Vec<Cell>,
}

impl Board {
    pub fn new_10x10() -> Self {
        let mut cells = Vec::with_capacity(100);
        for y in 0..10 {
            for x in 0..10 {
                cells.push(Cell {
                    coord: Coord { x, y },
                    cell_state: CellState::Empty,
                    transform: Transform {
                        translation: [x as f32, y as f32],
                    },
                });
            }
        }
        Board { cells }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub coord: Coord,
    pub cell_state: CellState,
    pub transform: Transform,
}

/// 2D transform for grid layout (position in world space)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub translation: [f32; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Empty,
    Occupied,
    Hit,
    Miss,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShipName {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl ShipName {
    pub fn length(self) -> u8 {
        match self {
            ShipName::Carrier => 5,
            ShipName::Battleship => 4,
            ShipName::Cruiser => 3,
            ShipName::Submarine => 3,
            ShipName::Destroyer => 2,
        }
    }
}
