use std::{collections::HashMap, io::{self, Write}};

mod commands;
mod game;

use commands::{register_commands};
use game::{build_world, GameState, clear_screen};


fn main() {
    let mut game = GameState {
        rooms: build_world(),
        current_location: 1,            // Start in room 1
        previous_location: Vec::new(),
        inventory: HashMap::new(),  // No previous location at the start
        max_hp: 32,
        hp: 32, // Player starts with full health
        conditions: HashMap::new(), // No conditions at the start
        mode: game::GameMode::OutOfCombat, // Start in out-of-combat mode
        combat_turn: None, // No combat turn at the start
        attack: 5, // Player's attack power
    };

    let commands = register_commands();

    clear_screen();
    println!("Welcome to the game!");
    commands.get("look").unwrap().execute(&[], &mut game);

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); //ensure prompt is printed before input

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Parse and handle the command
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();
        if let Some((cmd, args)) = tokens.split_first() {
            
            // Check if the game is over
            if let game::GameMode::GameOver = game.mode {
                if *cmd != "quit" {
                    println!("Game over! Type 'quit' to exit.");
                    continue;
                }
            }
            if let Some(handler) = commands.get(*cmd) {
                handler.execute(args, &mut game);
            } else {
                println!("Unknown command: {}", cmd);
            }
        }

        if game.hp <= 0 {
            game.mode = game::GameMode::GameOver;
            println!("You have died. Game over.");
            continue;
        }
    }
}
