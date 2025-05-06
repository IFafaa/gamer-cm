use chrono::{DateTime, Utc};

use super::player::Player;

pub struct Community {
    pub id: i32,
    pub name: String,
    pub players: Vec<Player>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Community {
    pub fn new(name: String) -> Self {
        Community {
            id: 0,
            name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            players: Vec::new(),
        }
    }
}

#[async_trait::async_trait]
pub trait CommunityRepository: Send + Sync {
    async fn insert(&self, community: &Community) -> anyhow::Result<()>;
    async fn exists(&self, name: String) -> anyhow::Result<bool>;
}
