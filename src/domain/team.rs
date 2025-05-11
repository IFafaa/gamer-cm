use chrono::{DateTime, Utc};

use super::player::Player;

pub struct Team {
    pub id: i32,
    pub name: String,
    pub players: Vec<Player>,
    pub community_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub enabled: bool,
}

impl Team {
    pub fn new(name: String, community_id: i32) -> Self {
        Team {
            id: 0,
            name,
            community_id,
            players: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            enabled: true,
        }
    }
}

#[async_trait::async_trait]

pub trait TeamRepository: Send + Sync {
    async fn insert(&self, team: &Team) -> anyhow::Result<()>;
    async fn exists(&self, name: String, community_id: i32) -> anyhow::Result<bool>;
    // async fn get_by_params(&self) -> anyhow::Result<Vec<Team>>;
    async fn save(&self, team: &Team) -> anyhow::Result<()>;
}
