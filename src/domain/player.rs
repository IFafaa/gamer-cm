use serde::Serialize;
use time::PrimitiveDateTime;

use crate::shared::date_time::DateTime;

#[derive(Serialize, Clone)]
pub struct Player {
    pub id: i32,
    pub nickname: String,
    pub community_id: i32,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub enabled: bool,
}

impl Player {
    pub fn new(nickname: String, community_id: i32) -> Self {
        Player {
            id: 0,
            nickname,
            community_id,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            enabled: true,
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[async_trait::async_trait]
pub trait PlayerRepository: Send + Sync {
    async fn insert(&self, player: &Player) -> anyhow::Result<()>;
    async fn exists(&self, name: String, community_id: i32) -> anyhow::Result<bool>;
    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Player>>;
    async fn get_by_ids(&self, ids: Vec<i32>) -> anyhow::Result<Vec<Player>>;
    async fn save(&self, player: &Player) -> anyhow::Result<()>;
}
