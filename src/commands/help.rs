use crate::game::{GameMode, GameState};

use super::Command; 

pub struct HelpCommand;
impl Command for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn execute(&self, _args: &[&str], _game: &mut GameState) {

        if let GameMode::InCombat = _game.mode {
            println!("Available commands during combat:");
            println!("  help - Show this help message");
            println!("  flee - attempt to flee to previous location");
            println!("  attack <monster_name> - Attack a monster in the room");
            return;
        }

        println!("Available commands:");
        println!("  help - Show this help message");
        println!("  look - Describe the current room. can also be used to look at exits, items, inventory, and monsters");
        println!("  go <direction> - Move in the specified direction (e.g., 'go north')");
        println!("  quit - Exit the game");
        println!("  use <item_name> - Use an item from your inventory");
        println!(" grab <item_name> - Pick up an item from the room");
        println!("  drop <item_name> - Drop an item from your inventory");
    }
}