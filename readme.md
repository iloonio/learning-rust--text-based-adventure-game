# Text-based Adventure Game in Rust
This project aims to learn Rust by making a text-based adventure game with command prompts, room navigation, inventory system, and a barebones combat system using dice rolls for attack damage. 

## Building & Running
- `cargo build --release --target x86_64-pc-windows-gnu`: creates an executable file that can run on windows
- `cargo build --release`: creates a standard ELF file that can be run in linux using ./learning_rust
- `cargo run`: builds and runs the file directly in vsc console.

## Creating new commands 
- make a new file in /commands directory (such as name.rs), and be sure to update mod.rs to include the following: 
```rust
pub mod name;
// ...
use name::nameCommand;
```
- If you need to use mutable variables such as `GameState` in your new command, you can call on it using 
```rust
use crate::game::GameState;
//or if you are fetching multiple items out of game.rs:
use crate::game::{GameState, clear_screen}
```
- Note that only code labeled with `pub` can be imported with `use`. 