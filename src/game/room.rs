use super::item::Item;
use super::monster::Monster;
use std::{collections::HashMap};

/// Represents a room in the game world
pub struct Room {
    //pub id: u32,
    pub name: String,
    pub description: String,
    pub exits: HashMap<String, u32>, // direction -> room id
    pub items: HashMap<String, Item>, // Items present in the room
    pub monsters: Vec<Monster>,
    pub dark: bool, // Indicates if the room is dark
}

impl Room {
    #[allow(dead_code)]
    pub fn new(name: &str, description: &str) -> Self {
        Room {
            name: name.to_string(),
            description: description.to_string(),
            exits: HashMap::new(),
            items: HashMap::new(),
            monsters: Vec::new(),
            dark: false, // Default to not dark
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.insert(item.name.to_string(),item);
    }

    /// Removes the monster at the given index, dropping its items into the room.
    pub fn defeat_monster(&mut self, monster_index: usize) {
        if monster_index < self.monsters.len() {
            let mut monster = self.monsters.remove(monster_index);
            for (item_name, item) in monster.items.drain() {
                self.items.insert(item_name, item);
            }
            println!("{} drops their items!", monster.name);
        }
    }
}