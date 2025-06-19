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
            if let Some(handler) = commands.get(*cmd) {
                handler.execute(args, &mut game);
            } else {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
