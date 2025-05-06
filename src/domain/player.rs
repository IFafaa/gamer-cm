use chrono::{DateTime, Utc};

pub struct Player {
    pub id: i32,
    pub nickname: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Player {
    pub fn new(nickname: String) -> Self {
        Player {
            id: 0,
            nickname,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
