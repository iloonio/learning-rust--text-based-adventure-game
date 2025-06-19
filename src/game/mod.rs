pub mod item;
pub mod npc;

use item::*;
use npc::*;

use std::{collections::HashMap};
use std::io;
use crossterm::{execute, terminal::{Clear, ClearType}};

/// Represents a room in the game world
pub struct Room {
    //pub id: u32,
    pub name: String,
    pub description: String,
    pub exits: HashMap<String, u32>, // direction -> room id
}

/// Holds the full game state, including rooms and player location history
pub struct GameState {
    pub rooms: HashMap<u32, Room>,
    pub current_location: u32,
    pub previous_location: Vec<u32>,
    pub inventory: HashMap<String, Item>,
}

/// Clears the terminal screen using Crossterm
pub fn clear_screen() {
    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
}

/// Constructs and returns the world map: 7 interconnected rooms
pub fn build_world() -> HashMap<u32, Room> {
    let mut rooms = HashMap::new();
    for id in 1..=7 {
        rooms.insert(id, Room {
            name: format!("Room {}", id),
            description: format!("This is room number {}.", id),
            exits: HashMap::new(),
        });
    }

    // Bidirectional connections
    for &(from, dir, to) in &[
        (1, "north", 2), (2, "south", 1),
        (1, "east", 3),  (3, "west", 1),
        (2, "east", 4),  (4, "west", 2),
        (3, "east", 5),  (5, "west", 3),
        (4, "north", 6), (6, "south", 4),
        (5, "north", 7), (7, "south", 5),
    ] {
        if let Some(room) = rooms.get_mut(&from) {
            room.exits.insert(dir.to_string(), to);
        }
    }
    rooms
}

