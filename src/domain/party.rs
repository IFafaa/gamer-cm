use time::PrimitiveDateTime;

use crate::shared::date_time::DateTime;

use super::team::Team;

pub struct Party {
    pub id: i32,
    pub community_id: i32,
    pub game_name: String,
    pub teams: Vec<Team>,
    pub team_winner_id: Option<i32>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl Party {
    pub fn new(game_name: String, teams: Vec<Team>, community_id: i32) -> Self {
        Party {
            id: 0,
            community_id,
            teams,
            team_winner_id: None,
            game_name,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

#[async_trait::async_trait]
pub trait PartyRepository: Send + Sync {
    async fn insert(&self, party: &Party) -> anyhow::Result<()>;
    async fn get_by_params(
        &self,
        params: IGetPartiesByParams,
    ) -> anyhow::Result<Vec<Party>>;
}

pub struct IGetPartiesByParams {
    pub community_id: Option<i32>,
    pub game_name: Option<String>,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
    pub teams_ids: Option<Vec<i32>>,
    pub team_winner_ids: Option<Vec<i32>>,
}
