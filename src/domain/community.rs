use serde::Serialize;
use time::PrimitiveDateTime;

use crate::shared::date_time::DateTime;

use super::player::Player;

#[derive(Serialize)]
pub struct Community {
    pub id: i32,
    pub name: String,
    pub players: Vec<Player>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl Community {
    pub fn new(name: String) -> Self {
        Community {
            id: 0,
            name,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            players: Vec::new(),
        }
    }
}

#[async_trait::async_trait]
pub trait CommunityRepository: Send + Sync {
    async fn insert(&self, community: &Community) -> anyhow::Result<()>;
    async fn exists(&self, name: String) -> anyhow::Result<bool>;
    async fn get_all(&self) -> anyhow::Result<Vec<Community>>;
}
