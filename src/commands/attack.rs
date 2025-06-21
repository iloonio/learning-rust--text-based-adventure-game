use super::Command;
use crate::game::{GameState, GameMode, CombatTurn};
use std::thread;
use std::time::Duration;

pub struct AttackCommand;
impl Command for AttackCommand {
    fn name(&self) -> &'static str {
        "attack"
    }

    fn execute(&self, args: &[&str], game: &mut GameState) {
        if let GameMode::InCombat = game.mode {
            if game.combat_turn != Some(CombatTurn::Player){
                println!("It's not your turn to attack.");
                return;
            }
            if args.is_empty() {
                println!("Usage: attack <npc_name>");
                return;
            }

            let room = game.rooms.get_mut(&game.current_location).unwrap();
            if let Some(monster) = room.monsters.first_mut(){
                println!("You attack the {}, dealing {} damage!", monster.name, game.attack);
                monster.hp -= game.attack as i32;
                if monster.hp <= 0 {
                    println!("You have defeated the {}!", monster.name);
                    room.defeat_monster(0);
                    game.mode = GameMode::OutOfCombat; // Exit combat mode
                    game.combat_turn = None; // Reset combat turn
                    if room.monsters.is_empty(){
                        game.mode = GameMode::OutOfCombat; // Exit combat mode if no monsters left
                        game.combat_turn = None;
                        println!("You have cleared the room of monsters."); 
                        return;
                    }
                } else {
                    thread::sleep(Duration::from_millis(1000));
                    game.combat_turn = Some(CombatTurn::Monster); // Switch turn to enemy
                    let monster = room.monsters.first().unwrap();
                    println!("the {} attacks you, dealing {} damage!", monster.name, monster.attack); 
                    game.hp -= monster.attack as i32; // Reduce player's HP by monster's attack
                    if game.hp <= 0 {
                        println!("you have been slain by the {}!", monster.name);
                        game.mode = GameMode::GameOver; // Set game mode to GameOver
                        game.combat_turn = None; // Reset combat turn
                        return;
                    } else {
                        
                        println!("You have {} HP left.", game.hp);
                        game.combat_turn = Some(CombatTurn::Player); // Switch turn back to player
                    }
                }
            } else {
                println!("There are no monsters to attack in this room.");
                game.mode = GameMode::OutOfCombat; // Set game mode to OutOfCombat if no monsters
                game.combat_turn = None; // Reset combat turn
            }
            
        } else {
            println!("You can only attack during combat.");
        }
    }
}