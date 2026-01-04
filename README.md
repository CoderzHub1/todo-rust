# todo

A small command-line TODO application written in Rust. It uses `sled` as an embedded key-value store and `serde` + `bincode` for serialization. The app is minimal and intended for learning and experimentation with persistent storage in Rust.

## Features
- Add tasks
- List tasks (in-memory view)
- Update task completion status
- Refresh in-memory tasks from the persistent `sled` DB
- Simple, dependency-light implementation

## Requirements
- Rust toolchain (rustup + cargo)
- Works on platforms supported by the `sled` crate (Linux, macOS, Windows)

## Dependencies
Primary dependencies are declared in `Cargo.toml`:
- `serde` (with `derive`)
- `bincode`
- `sled`

## Quick start
1. Clone the repo or copy the project into a directory.
2. Run:
   - `cargo run` (for debug) or `cargo run --release` (for optimized build)

The application will create a `sled` DB directory named `tasks` in the current working directory.

## Usage
When the program runs it prompts for commands. Supported commands:

- `add` — Add a new task. You'll be prompted for the task name.
- `list` — Print all tasks currently loaded into memory.
- `update` — Change a task's completion status by task `id`.
- `refresh` — Reload tasks from the sled DB into memory.
- `exit` — Quit the program.

Example session:
1. Start program: `cargo run`
2. Enter `add`, then provide a name (e.g. `Buy groceries`)
3. Enter `list` to see tasks
4. Enter `update` then the task `id` and status (`C` for completed, `n` for not completed)
5. Enter `refresh` after external changes (if any)
6. Enter `exit` to quit

## Data model & storage
- `Task` struct:
  - `id: u32`
  - `name: String`
  - `completed: bool`

Tasks are serialized with `bincode` and stored in `sled`. Keys are the big-endian bytes of the `u32` `id` (`id.to_be_bytes()`), and values are the `bincode`-encoded `Task`.

Core code locations:
- Entrypoint and REPL loop: `src/main.rs`
- Prompt helper: `src/prompts/prompts.rs`
- Task definition: `src/tasks/tasks.rs`
- Persistence helpers (`add`, `extract`, `update`): `src/tasks/manage_tasks.rs`, `src/tasks/extract_tasks.rs`

## Project structure
- `Cargo.toml` — project metadata and dependencies
- `src/main.rs` — application entry point and command loop
- `src/lib.rs` — modules
- `src/prompts/` — input prompt helper
- `src/tasks/` — task model and persistence functions

## Notes & limitations
- The app keeps an in-memory `Vec<Task>` as the session view. Use `refresh` to reload the authoritative state from disk.
- There is no delete command yet — deleting a task would require removing the DB key and the in-memory entry.
- IDs are assigned sequentially by reading the last loaded task's `id`. Deleting tasks can leave gaps.
- Error handling is minimal and primarily returns errors via `Result`.
- Single-process CLI — no concurrency handling for multiple simultaneous clients.

## Possible improvements
- Add `delete` command.
- Use DB as the single source of truth for listing instead of the in-memory `Vec`.
- Add more robust input validation and nicer UI (e.g., table output).
- Add tests and CI.

## Contributing
This project is a learning exercise. If you'd like to contribute, open an issue or submit a PR with a focused improvement (e.g., adding `delete`, improving error messages, or adding tests).
<<<<<<< HEAD
=======

## License
No license included. Add a license file if you want to make this project permissively reusable.
>>>>>>> b144cb5c49068cf4de2d24644616f5ebb9790a36
