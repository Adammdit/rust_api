use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: String,
}
