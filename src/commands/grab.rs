//This command is used for grabbing items from the current room and adding them to the player's inventory.

// essentially imlemented the same way as use_on command, but instead of fetching from the invetnroy, we fetch from the room's items hashmap. 

use super::Command;
use crate::game::GameState;

pub struct GrabCommand;
impl Command for GrabCommand {
    fn name(&self) -> &'static str {
        "grab"
    }

    fn execute(&self, args: &[&str], game: &mut GameState) {
        if args.is_empty() {
            println!("Usage: grab <item_name>");
            return;
        }

        let item_name = args[0].to_ascii_lowercase();

        if let Some(room) = game.rooms.get_mut(&game.current_location){
            if let Some(item) = room.items.remove(&item_name){
                game.inventory.entry(item_name.clone())
                .or_insert_with(Vec::new).
                push(item);
                
                println!("You have grabbed the {}. It is now in your inventory.", item_name);
            } else {
                println!("There is no {} in this room.", item_name);
            }
        } else {
            println!("You are not in a valid room.");
        }
    }
}