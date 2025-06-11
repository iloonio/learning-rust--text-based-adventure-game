use super::Command; 

pub struct HelpCommand;
impl Command for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn execute(&self, _args: &[&str], _game: &mut crate::game::GameState) {
        println!("Available commands:");
        println!("  help - Show this help message");
        println!("  look - Describe the current room");
        println!("  go <direction> - Move in the specified direction (e.g., 'go north')");
        println!("  quit - Exit the game");
    }
}