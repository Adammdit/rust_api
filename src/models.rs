use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
}
