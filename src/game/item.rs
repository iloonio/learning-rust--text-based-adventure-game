use super::npc;

pub struct Item{
    pub name: String,
    pub description: String,
}
impl Item {
    pub fn new(name: &str, description: &str) -> Self {
        Item {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}

pub trait Usable{
    fn use_item(&self, );
}

pub trait Targeting {
    fn target(&self, target: &npc::Npc) -> Result<(), String>;
}

