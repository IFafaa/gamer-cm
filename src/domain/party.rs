use chrono::{DateTime, Utc};

use super::team::Team;

pub struct Party {
    pub id: i32,
    pub community_id: i32,
    pub game_name: String,
    pub teams: Vec<Team>,
    pub team_winner_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Party {
    pub fn new(game_name: String, teams: Vec<Team>, community_id: i32) -> Self {
        Party {
            id: 0,
            community_id,
            teams,
            team_winner_id: None,
            game_name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[async_trait::async_trait]
pub trait PartyRepository: Send + Sync {
    async fn insert(&self, party: &Party) -> anyhow::Result<()>;
}
