# Bevy Battleship

A simple implementation of the classic Battleship game using the [Bevy](https://bevyengine.org/) game engine in Rust.

## Features

- 1 human player vs 1 computer player
- 10x10 board for each player
- ECS-based architecture ([Entities, Components, Systems](https://bevyengine.org/learn/book/getting-started/ecs/))
- Board and cell grid logic
- Ready for further development (ships, UI, gameplay)

## Getting Started

1. Install Rust: <https://rustup.rs/>
2. Clone this repository
3. Run the game:

   ```sh
   cargo run
   ```

## Project Structure

- `src/components.rs` - ECS components and data types
- `src/systems.rs` - Game systems (setup, camera, etc.)
- `src/resources.rs` - Global resources (game state)
- `src/main.rs` - App entry point

## Development Loop

For a rapid inner development loop, you can use [`cargo-watch`](https://crates.io/crates/cargo-watch) and [`cargo-update`](https://crates.io/crates/cargo-update).

Install them with:

```sh
cargo install cargo-watch cargo-update
```

Then, you can automatically check, build, and run your project on file changes with:

```sh
cargo watch -x check -x build -x run
```

This will re-run your code every time you save a file, speeding up your workflow.

## Some Bevy Examples

Here are some official Bevy examples you can explore:

- [hello_world](https://github.com/bevyengine/bevy/blob/main/examples/hello_world.rs): The simplest Bevy app.
- [ecs_guide](https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs): A comprehensive ECS usage guide.
- [Examples on GitHub](https://github.com/bevyengine/bevy/tree/main/examples)
- [Bevy Book: Examples](https://bevyengine.org/learn/book/examples/)
- [Bevy Cheat Book: Examples](https://bevy-cheatbook.github.io/examples.html)
- [breakout](https://github.com/bevyengine/bevy/blob/main/examples/game/breakout.rs): A full game example.
- [sprite](https://github.com/bevyengine/bevy/blob/main/examples/2d/sprite.rs): Basic 2D sprite rendering.
- [3d_scene](https://github.com/bevyengine/bevy/blob/main/examples/3d/3d_scene.rs): Basic 3D scene setup.
