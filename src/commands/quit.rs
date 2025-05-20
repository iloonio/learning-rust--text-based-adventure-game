use super::Command;
use crate::game::GameState;

pub struct QuitCommand;
impl Command for QuitCommand {
    fn name(&self) -> &'static str {
        "quit"
    }

    fn execute(&self, _args: &[&str], _game: &mut GameState) {
        println!("Goodbye, World");
        std::process::exit(0);
    }
}