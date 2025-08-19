// ...existing code...
use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PlayerId(pub u8);

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub coord: UVec2,
    pub state: CellState,
    pub board: Entity,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub position: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellState {
    Empty,
    Occupied(ShipName),
    Hit,
    Miss,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShipDirection {
    Vertical,
    Horizontal,
}

#[derive(Component, Debug)]
pub struct Ship {
    pub name: ShipName,
    pub owner: PlayerId,
    pub direction: ShipDirection,
    pub cells: Vec<UVec2>,
}

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
