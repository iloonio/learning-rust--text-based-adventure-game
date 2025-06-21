use super::Command;
use crate::game::GameState;
use crate::game::item::{Usable};

pub struct UseCommand;
impl Command for UseCommand {
    fn name(&self) -> &'static str {
        "use"
    }

    fn execute(&self, args: &[&str], game: &mut GameState) {
        if args.is_empty() {
            println!("Usage: use <item_name>");
            return;
        }

        let item_name = args[0];

        // item is temporarily removed from the inventory to prevent multiple borrows. 
        let maybe_item = {
            if let Some(items) = game.inventory.get_mut(item_name) {
                items.pop()
            } else {
                println!("You don't have a {} in your inventory.", item_name);
                return;
            }
        };

        if let Some(item) = maybe_item {
            item.use_on(game);

            if !item.remove_after_use() {
                // Now it's safe to borrow mutably again
                if let Some(items) = game.inventory.get_mut(item_name) {
                    items.push(item);
                }
            }
        }
    }
}