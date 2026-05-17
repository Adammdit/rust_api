use std::sync::Mutex;
use crate::models::Item;
use uuid::Uuid;

pub struct AppState {
    pub items: Mutex<Vec<Item>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(Vec::new()),
        }
    }

    pub fn add_item(&self, name: String) -> Item {
        let item = Item { id: Uuid::new_v4(), name };
        self.items.lock().unwrap().push(item.clone());
        item
    }
}
