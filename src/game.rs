//! Game setup and orchestration for Battleship

use bevy::prelude::*;
use crate::resources::GameState;
use crate::systems::{setup_battleship, setup_camera};

pub fn run_game_app() {
    App::new()
        .init_resource::<GameState>()
        .add_systems(Startup, (setup_camera, setup_battleship))
        .run();
}
