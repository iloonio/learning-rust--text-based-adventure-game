use crate::game::GameState;


pub enum ItemType {
    Torch,
    Herb,
    Other,
}

pub struct Item{
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
}
impl Item {
    pub fn new(name: &str, description: &str, item_type: &str) -> Self {
        Item {
            name: name.to_string(),
            description: description.to_string(),
            item_type: match item_type {
                "Torch" => ItemType::Torch,
                "Herb" => ItemType::Herb,
                "Other" => ItemType::Other,
                _ => panic!("Unknown item type"),
            },
        }
    }
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}

pub trait Usable {
    fn use_on(&self, game: &mut GameState);
    fn remove_after_use(&self) -> bool {
        false // Default behavior, can be overridden
    }
}
impl Usable for Item {
    fn use_on(&self, game: &mut GameState) {
        match &self.item_type {
            ItemType::Torch => {
                println!("You light the torch.");
                game.conditions.insert("light".to_string(), true);
            }
            ItemType::Herb => {
                println!("You use the herb to restore health.");
                game.hp = (game.hp + 10).min(game.max_hp); // Restore 10 HP, maxing out at max_hp
            }
            _ => {
                println!("You can't use this item.");
            }
        }
    }
    fn remove_after_use(&self) -> bool {
        match &self.item_type {
            ItemType::Herb => true, // Herb is consumed after use
            _ => false, // Other items are not consumed
        }
    }
}



