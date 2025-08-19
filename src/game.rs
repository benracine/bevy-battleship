use crate::resources::GameState;
use crate::systems::{render_boards, setup_camera, spawn_boards, spawn_cells, spawn_players, spawn_ships, test_simple_render};
use bevy::prelude::*;

pub fn run_game_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GameState>()
        .add_systems(
            Startup,
            (
                setup_camera,
                spawn_players,
                spawn_boards.after(spawn_players),
                spawn_cells.after(spawn_boards),
                spawn_ships.after(spawn_boards),
                render_boards.after(spawn_cells),
            ),
        )
        .run();
}
