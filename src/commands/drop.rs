//drops an item from the inventory, placing it in the current toom's items hashmap. 

use super::Command;
use crate::game::GameState;

pub struct DropCommand;
impl Command for DropCommand {
    fn name(&self) -> &'static str {
        "drop"
    }

    fn execute(&self, args: &[&str], game: &mut GameState) {
        if args.is_empty() {
            println!("Usage: drop <item_name>");
            return;
        }

        let item_name = args[0];

        // Try to remove one item from the inventory vector
        if let Some(items) = game.inventory.get_mut(item_name) {
            if let Some(item) = items.pop() {
                // Remove the key if no more items left
                if items.is_empty() {
                    game.inventory.remove(item_name);
                }
                // Add the item to the current room's items
                if let Some(room) = game.rooms.get_mut(&game.current_location) {
                    room.items.insert(item_name.to_string(), item);
                    println!("You have dropped the {}. It is now in the room.", item_name);
                } else {
                    println!("You are not in a valid room.");
                }
            } else {
                println!("You don't have a {} in your inventory.", item_name);
            }
        } else {
            println!("You don't have a {} in your inventory.", item_name);
        }
    }
}