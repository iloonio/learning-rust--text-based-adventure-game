use super::Command;
use super::look::LookCommand;
use crate::game::{GameState, clear_screen};

// Go command changes the player's location
pub struct GoCommand;
impl Command for GoCommand {
    fn name(&self) -> &'static str {
        "go"
    }

    fn execute(&self, args: &[&str], game: &mut GameState) {
        if args.is_empty() {
            println!("Go where?");
            return;
        }
        let dir = args[0].to_lowercase();

        if dir == "back" {
            if let Some(_) = game.previous_location {
                clear_screen();
                println!("You go back.\n");
                std::mem::swap(
                    &mut game.current_location,
                    game.previous_location.as_mut().unwrap(),
                );
                LookCommand.execute(&[], game); //describe the new room
            } else {
                println!("You can't go back from here.");
            }
            return;
        }

        let room = &game.rooms[&game.current_location];
        if let Some(&next_room_id) = room.exits.get(&dir) {
            clear_screen(); //clears the screen before describing the new room
            game.current_location = next_room_id; //update the current location
            game.previous_location = Some(room.id); //set the previous location to the current room
            println!("You go {}.\n", dir);
            LookCommand.execute(&[], game); //describe the new room
        } else {
            println!("You can't go {} from here.", dir);
        }
    }
}