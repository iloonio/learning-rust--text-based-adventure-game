use std::collections::HashMap;
use crate::game::item::Item;

pub struct Monster {
    pub name: String,
    pub description: String,
    pub items: HashMap<String, Item>,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: u32,
}