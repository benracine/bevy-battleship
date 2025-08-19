//! Game setup and orchestration for Battleship

use crate::resources::GameState;
use crate::systems::{render_boards, setup_camera, spawn_boards, spawn_cells, spawn_players};
use bevy::prelude::*;

pub fn run_game_app() {
    App::new()
        .init_resource::<GameState>()
        .add_systems(
            Startup,
            (setup_camera, spawn_players, spawn_boards, spawn_cells),
        )
        .add_systems(Update, render_boards)
        .run();
}
