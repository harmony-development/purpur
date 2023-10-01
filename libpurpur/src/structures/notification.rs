use serde::{Serialize, Deserialize};

use super::Identifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Identifier<Notification>,
    pub title: String,
    pub body: String,
}
