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

# Fast Development Loop

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
