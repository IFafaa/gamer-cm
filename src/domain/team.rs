use serde::Serialize;
use time::PrimitiveDateTime;

use crate::shared::date_time::DateTime;

use super::player::Player;

#[derive(Serialize, Clone)]

pub struct Team {
    pub id: i32,
    pub name: String,
    pub players: Vec<Player>,
    pub community_id: i32,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub enabled: bool,
}

impl Team {
    pub fn new(name: String, community_id: i32) -> Self {
        Team {
            id: 0,
            name,
            community_id,
            players: Vec::new(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            enabled: true,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}

#[async_trait::async_trait]

pub trait TeamRepository: Send + Sync {
    async fn insert(&self, team: &Team) -> anyhow::Result<()>;
    async fn exists(&self, name: String, community_id: i32) -> anyhow::Result<bool>;
    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Team>>;
    async fn save(&self, team: &Team) -> anyhow::Result<()>;
}
