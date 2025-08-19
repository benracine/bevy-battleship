use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Component, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PlayerId(pub u8);

#[derive(Component, Copy, Clone, Debug)]
pub struct Transform {
    pub position: Vec2,
}

#[derive(Component, Copy, Clone, Debug)]
pub enum PlayerType {
    Human,
    Computer,
}

#[derive(Component, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub player_type: PlayerType,
}

#[derive(Component, Debug)]
pub struct Board {
    pub size: UVec2,
    pub owner: PlayerId,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CellState {
    Empty,
    Hit,
    Miss,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Cell {
    pub coord: UVec2,
    pub state: CellState,
    pub board: Entity,
}

/// Ships
#[derive(Component, Debug)]
pub struct Ship {
    pub name: ShipName,
    pub owner: PlayerId,
    pub direction: ShipDirection,
    pub cells: Vec<UVec2>,
}

#[derive(Copy, Clone, Debug)]
pub enum ShipDirection {
    Vertical,
    Horizontal,
}

#[derive(EnumIter, Copy, Clone, PartialEq, Eq, Hash, Debug)]
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

impl Ship {
    pub fn occupies_cell(&self, coord: UVec2) -> bool {
        self.cells.contains(&coord)
    }
    
    pub fn is_at_coord(&self, coord: UVec2) -> Option<ShipName> {
        if self.occupies_cell(coord) {
            Some(self.name)
        } else {
            None
        }
    }
}
