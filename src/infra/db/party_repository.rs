use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::party::{Party, PartyRepository};

pub struct PgPartyRepository {
    pub pool: PgPool,
}

impl PgPartyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PartyRepository for PgPartyRepository {
    async fn insert(&self, party: &Party) -> Result<(), anyhow::Error> {
        let rec = sqlx::query!(
            "INSERT INTO parties (community_id, game_name, team_winner_id) 
             VALUES ($1, $2, $3) RETURNING id",
            party.community_id,
            party.game_name,
            party.team_winner_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(anyhow::Error::from)?;

        let party_id = rec.id;

        for team in &party.teams {
            sqlx::query!(
                "INSERT INTO party_teams (party_id, team_id) VALUES ($1, $2)",
                party_id,
                team.id
            )
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;
        }

        Ok(())
    }
}
