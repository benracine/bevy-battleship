use crate::resources::GameState;
use crate::systems::{render_boards, setup_camera, spawn_board_labels, spawn_boards, spawn_cells, spawn_players, spawn_ships};
use bevy::prelude::*;

pub fn run_game_app() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Battleship".to_string(),
                resolution: (600.0, 1000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_systems(
            Startup,
            (
                setup_camera,
                spawn_players,
                spawn_boards.after(spawn_players),
                spawn_cells.after(spawn_boards),
                spawn_board_labels.after(spawn_boards),
                spawn_ships.after(spawn_boards),
                render_boards.after(spawn_cells),
            ),
        )
        .run();
}
