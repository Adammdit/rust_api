use crate::models::Item;
use serde_json;
use sled::Db;
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;
use uuid::Uuid;

pub struct AppState {
    db: Arc<Mutex<Db>>,
}

impl AppState {
    pub fn new() -> Self {
        std::fs::create_dir_all("data").expect("Failed to create data directory");
        let db = sled::open("data/db").expect("Failed to open sled database");
        Self { db: Arc::new(Mutex::new(db)) }
    }

    pub fn add_item(&self, name: String, description: Option<String>) -> Item {
        let item = Item {
            id: Uuid::new_v4(),
            name,
            description,
            completed: false,
            created_at: OffsetDateTime::now_utc().unix_timestamp().to_string(),
        };

        let value = serde_json::to_vec(&item).expect("Failed to serialize item");
        let db = self.db.lock().unwrap();
        db.insert(item.id.to_string(), value)
            .expect("Failed to insert item");
        db.flush().expect("Failed to flush database");
        item
    }

    pub fn list_items(&self, completed: Option<bool>) -> Vec<Item> {
        let db = self.db.lock().unwrap();
        db.iter()
            .values()
            .filter_map(|result| result.ok())
            .filter_map(|bytes| serde_json::from_slice::<Item>(&bytes).ok())
            .filter(|item| completed.map_or(true, |status| item.completed == status))
            .collect()
    }

    pub fn get_item(&self, id: Uuid) -> Option<Item> {
        let db = self.db.lock().unwrap();
        db.get(id.to_string())
            .ok()
            .flatten()
            .and_then(|bytes| serde_json::from_slice::<Item>(&bytes).ok())
    }

    pub fn delete_item(&self, id: Uuid) -> bool {
        let db = self.db.lock().unwrap();
        db.remove(id.to_string()).map(|res| res.is_some()).unwrap_or(false)
    }

    pub fn update_item(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<Option<String>>,
        completed: Option<bool>,
    ) -> Option<Item> {
        let mut item = self.get_item(id)?;
        if let Some(name) = name {
            item.name = name;
        }
        if let Some(description) = description {
            item.description = description;
        }
        if let Some(completed) = completed {
            item.completed = completed;
        }

        let value = serde_json::to_vec(&item).ok()?;
        let db = self.db.lock().unwrap();
        db.insert(id.to_string(), value).ok()?;
        db.flush().ok()?;
        Some(item)
    }
}
