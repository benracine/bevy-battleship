#![allow(dead_code)]
//! Classic Battleship game in Bevy.
//! - 1 human player, 1 computer player (initially)
//! - Boards are randomly set at first; later, allow player to set their own
//! - Human provides real moves; computer makes automated moves

mod game;
mod components;
mod resources;
mod systems;

fn main() {
    game::run_game_app();
}
