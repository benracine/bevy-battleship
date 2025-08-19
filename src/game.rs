//! Game setup and orchestration for Battleship

use crate::resources::GameState;
use crate::systems::{setup_battleship, setup_camera, spawn_boards, spawn_cells, spawn_players};
use bevy::prelude::*;

pub fn run_game_app() {
    App::new()
        .init_resource::<GameState>()
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_battleship,
                spawn_players,
                spawn_boards,
                spawn_cells,
            ),
        )
        .run();
}
