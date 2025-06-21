pub mod item;
pub mod room;
pub mod monster;

use item::*;
//use npc::*;
use room::*;

use std::{collections::HashMap};
use std::io;
use crossterm::{execute, terminal::{Clear, ClearType}};

use crate::game::monster::Monster;


pub enum GameMode {
    OutOfCombat,
    InCombat,
    GameOver,
}

#[derive(PartialEq)]
pub enum CombatTurn {
    Player,
    Monster,
}

/// Holds the full game state, including rooms and player location history
pub struct GameState {
    pub rooms: HashMap<u32, Room>,
    pub current_location: u32,
    pub previous_location: Vec<u32>,
    pub inventory: HashMap<String, Vec<Item>>,
    pub conditions: HashMap<String, bool>,
    pub hp: i32,
    pub max_hp: i32,
    pub mode: GameMode,
    pub combat_turn: Option<CombatTurn>, // Current turn in combat, if applicable
    pub attack: u32, // Player's attack power
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
            items: HashMap::new(),
            dark: id == 4 || id == 6, // Rooms 4 and 6 are dark
            monsters: Vec::new(),
        });
    }

    let mut goblin = Monster {
        name: "Goblin".to_string(),
        description: "A small, green-skinned creature with a mischievous grin.".to_string(),
        items: HashMap::new(),
        hp: 16,
        max_hp: 16,
        attack: 4,
    };
    goblin.items.insert("herb".to_string(), Item::new(
        "herb",
        "A healing herb that restores health when used.",
        "Herb"
    ));
    rooms.get_mut(&5).unwrap().monsters.push(goblin);

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

    let torch = Item::new("torch", "A wooden torch that can be lit to provide light.", "Torch");

    rooms.get_mut(&2).unwrap().add_item(torch);

    rooms.get_mut(&3).unwrap().add_item(
        Item::new("herb", 
        "A healing herb that restores health when used.", 
        "Herb"
        )
    );
        rooms.get_mut(&1).unwrap().add_item(
        Item::new("herb", 
        "A healing herb that restores health when used.", 
        "Herb"
        )
    );

    rooms
}

