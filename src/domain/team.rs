use chrono::{DateTime, Utc};

use super::player::Player;

pub struct Team {
    pub id: i32,
    pub name: String,
    pub members: Vec<Player>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Team {
    pub fn new(name: String) -> Self {
        Team {
            id: 0,
            name,
            members: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
