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

        if args.is_empty() {
            println!("You are in {}.\n{} ", room.name, room.description);
            if !room.exits.is_empty() {
                println!("Exits: ");
                for dir in room.exits.keys() {
                    print!("{} ", dir);
                }
                println!();
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
                _ => {
                    println!("Unknown argument: {}", args[0]);
                }
            }
        }
    }
}
