use super::Command;
use crate::game::GameState;

// Look command describes the current room
pub struct LookCommand;
impl Command for LookCommand {
    fn name(&self) -> &'static str {
        "look"
    }

    // args field is currently unused but it will be implemented in the future
    fn execute(&self, args: &[&str], game: &mut GameState) {
        let room = &game.rooms[&game.current_location];

        if room.dark && !game.conditions.get("light").copied().unwrap_or(false) {
            println!("It's too dark to see anything! You need a light source.");
            return;
        }

        if args.is_empty() {
            println!("You are in {}.\n{} ", room.name, room.description);
            if !room.exits.is_empty() {
                println!("Exits: ");
                for dir in room.exits.keys() {
                    print!("{} ", dir);
                }
                println!();
            }
            if !room.items.is_empty() {
                println!("You see the following items: ");
                for item in room.items.keys() {
                    print!("{} ", item);
                }
                println!();
            }
            if !room.monsters.is_empty() {
                println!("You see the following monsters:");
                for monster in &room.monsters {
                println!(
                    "- {}: {} (HP: {}/{}, ATK: {})",
                    monster.name, monster.description, monster.hp, monster.max_hp, monster.attack
                    );
                }
            }
        } else {
            match args[0].to_lowercase().as_str() {
                "desc" | "description" => {
                    println!("{}", room.description);
                }
                "exit" | "exits" => {
                    if !room.exits.is_empty() {
                        println!("Exits: ");
                        for dir in room.exits.keys() {
                            print!("{} ", dir);
                        }
                        println!();
                    } else {
                        println!("No exits available.");
                    }
                }
                "items" | "item" => {
                    if !room.items.is_empty() {
                        println!("You see the following items: ");
                        for item in room.items.keys() {
                            print!("{} ", item);
                        }
                        println!();
                    } else {
                        println!("There are no items in this room.");
                    }
                }
                "inventory" | "inv" => {
                    if game.inventory.is_empty() {
                        println!("Your inventory is empty.");
                    } else {
                        println!("You have the following items in your inventory: ");
                        for (name, items) in &game.inventory {
                            println!("{} x{}", name, items.len());
                        }
                        println!();
                    }
                }
                "monster | monsters" => {
                    if !room.monsters.is_empty() {
                        println!("You see the following monsters:");
                        for monster in &room.monsters {
                            println!(
                                "- {}: {} (HP: {}/{}, ATK: {})",
                                monster.name, monster.description, monster.hp, monster.max_hp, monster.attack
                            );
                        }
                    } else {
                        println!("There are no monsters in this room.");
                    }
                }
                "self" | "player" | "me" => {
                    println!("Your current HP: {}/{}", game.hp, game.max_hp);
                }
                monster_name => {
                    if !room.monsters.is_empty(){
                                            //try to find a monster by name (case insensitive)
                    if let Some(monster) = room.monsters.iter().find(|m| m.name.to_lowercase() == monster_name) {
                        println!(
                            "{}: {} (HP: {}/{}, ATK: {})",
                            monster.name, monster.description, monster.hp, monster.max_hp, monster.attack
                        );
                    } else {
                        println!("No such monster found in this room.");
                    }
                    } else {
                        println!("Unknown argument: {}", args[0]);
                    }
                }
            }
        }
    }
}
