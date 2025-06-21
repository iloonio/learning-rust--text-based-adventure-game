use super::Command;
use super::look::LookCommand;
use crate::game::{clear_screen, GameMode, GameState};

// Go command changes the player's location
pub struct GoCommand;
impl Command for GoCommand {
    fn name(&self) -> &'static str {
        "go"
    }

    fn execute(&self, args: &[&str], game: &mut GameState) {
        if let GameMode::InCombat = game.mode {
            println!("You cannot move freely while in combat! use flee to escape back to the previous location.");
            return;
        }

        if args.is_empty() {
            println!("Go where?");
            return;
        }

        let dir = args[0].to_lowercase();

        if dir == "back" {  
            // pop() returns an Option Type, so we let Some value be the return type. We then handle the two possible Option cases, either we have Some u32 value, or we have None. 
            if let Some(prev_room_id) = game.previous_location.pop() {
                clear_screen();
                println!("You go back.\n");
                game.current_location = prev_room_id;
                LookCommand.execute(&[], game); //describe new room
            } else {
                println!("You cannot go back from here.");
            }
            return;
        }

        let room = &game.rooms[&game.current_location];
        if let Some(&next_room_id) = room.exits.get(&dir) {
            clear_screen(); 
            game.previous_location.push(game.current_location);
            game.current_location = next_room_id; //update the current location
            println!("You go {}.\n", dir);
            LookCommand.execute(&[], game); //describe the new room
        } else {
            println!("You can't go {} from here.", dir);
        }

        let room = &game.rooms[&game.current_location];
        if !room.monsters.is_empty() {
            game.mode = GameMode::InCombat;
            game.combat_turn = Some(crate::game::CombatTurn::Player);
            print!("A ");
            for (i, monster) in room.monsters.iter().enumerate() {
                if i > 0 {
                    print!(" and ");
                }
                print!("{}", monster.name);
            }
            println!(" appears! Prepare for combat!");
        }
    }
}