use serde::Serialize;
use time::PrimitiveDateTime;

use crate::shared::date_time::DateTime;

use super::{player::Player, team::Team};

#[derive(Serialize)]
pub struct Community {
    pub id: i32,
    pub name: String,
    pub players: Vec<Player>,
    pub teams: Vec<Team>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub enabled: bool,
}

impl Community {
    pub fn new(name: String) -> Self {
        Community {
            id: 0,
            name,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            players: Vec::new(),
            teams: Vec::new(),
            enabled: true,
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.updated_at = DateTime::now();
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[async_trait::async_trait]
pub trait CommunityRepository: Send + Sync {
    async fn insert(&self, community: &Community) -> anyhow::Result<()>;
    async fn exists(&self, name: String) -> anyhow::Result<bool>;
    async fn get_all(&self) -> anyhow::Result<Vec<Community>>;
    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Community>>;
    async fn save(&self, community: &Community) -> anyhow::Result<()>;
}
