pub mod look;
pub mod go;
pub mod quit;

use std::collections::HashMap;
use crate::game::GameState;

// re-export command implementations
use look::LookCommand;
use go::GoCommand;
use quit::QuitCommand;


pub trait Command {
    fn name(&self) -> &'static str;
    fn execute(&self, args: &[&str], game: &mut GameState);
}

pub type CommandMap = HashMap<String, Box<dyn Command>>;


pub fn register_commands() -> CommandMap {
    let mut commands: CommandMap = HashMap::new();

    let core: Vec<Box<dyn Command>> = vec![
        Box::new(LookCommand),
        Box::new(GoCommand),
        Box::new(QuitCommand),
    ];

    for cmd in core {
        commands.insert(cmd.name().to_string(), cmd);
    }

    commands
}


