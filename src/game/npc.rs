pub struct Npc {
    pub name: String,
    pub description: String,
    pub items: Vec<String>, // List of item names the NPC has
}
impl Npc {
    pub fn new(name: &str, description: &str) -> Self {
        Npc {
            name: name.to_string(),
            description: description.to_string(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item_name: String) {
        self.items.push(item_name);
    }

    pub fn remove_item(&mut self, item_name: &str) -> Option<String> {
        if let Some(pos) = self.items.iter().position(|x| x == item_name) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }
}
impl std::fmt::Display for Npc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}