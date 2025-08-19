#![allow(dead_code)]
//! Classic Battleship game in Bevy.
//! - 1 human player, 1 computer player (initially)
//! - Boards are randomly set at first; later, allow player to set their own
//! - Human provides real moves; computer makes automated moves

mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use resources::GameState;
use systems::{setup_battleship, setup_camera};

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_systems(Startup, (setup_camera, setup_battleship))
        .run();
}
