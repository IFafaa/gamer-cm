use crate::domain::team::{Team, TeamRepository};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgTeamRepository {
    pub pool: PgPool,
}

impl PgTeamRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TeamRepository for PgTeamRepository {
    async fn insert(&self, team: &Team) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO teams (name, community_id) VALUES ($1, $2)",
            team.name,
            team.community_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn exists(&self, name: String, community_id: i32) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM teams WHERE name = $1 AND community_id = $2)",
            name,
            community_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.exists.unwrap_or(false))
    }

    async fn save(&self, team: &Team) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE teams SET name = $1, enabled = $2 WHERE id = $3",
            team.name,
            team.enabled,
            team.id
        )
        .execute(&self.pool)
        .await?;

        for player in &team.players {
            sqlx::query!(
                "INSERT INTO team_players (team_id, player_id, created_at, updated_at, enabled)
             SELECT $1, $2, NOW(), NOW(), $3
             WHERE NOT EXISTS (
             SELECT 1 FROM team_players WHERE team_id = $1 AND player_id = $2
             )",
                team.id,
                player.id,
                player.enabled
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}
