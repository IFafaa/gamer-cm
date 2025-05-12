use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::player::{Player, PlayerRepository};

pub struct PgPlayerRepository {
    pub pool: PgPool,
}

impl PgPlayerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PlayerRepository for PgPlayerRepository {
    async fn insert(&self, player: &Player) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO players (nickname, community_id) VALUES ($1, $2)",
            player.nickname,
            player.community_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn exists(&self, nickname: String, community_id: i32) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM players WHERE nickname = $1 AND community_id = $2 AND enabled = true) AS exists",
            nickname,
            community_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.exists.unwrap_or(false))
    }

    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Player>> {
        let row = sqlx::query!(
            "SELECT id, nickname, community_id, created_at, updated_at, enabled FROM players WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            return Ok(Some(Player {
                id: row.id,
                nickname: row.nickname,
                community_id: row.community_id,
                created_at: row.created_at,
                updated_at: row.updated_at,
                enabled: row.enabled,
            }));
        }

        Ok(None)
    }

    async fn get_by_ids(&self, ids: Vec<i32>) -> anyhow::Result<Vec<Player>> {
        let rows = sqlx::query!(
            "SELECT id, nickname, community_id, created_at, updated_at, enabled FROM players WHERE id = ANY($1)",
            &ids
        )
        .fetch_all(&self.pool)
        .await?;

        let players = rows
            .into_iter()
            .map(|row| Player {
                id: row.id,
                nickname: row.nickname,
                community_id: row.community_id,
                created_at: row.created_at,
                updated_at: row.updated_at,
                enabled: row.enabled,
            })
            .collect();

        Ok(players)
    }

    async fn save(&self, player: &Player) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE players SET nickname = $1, enabled = $2, updated_at = NOW() WHERE id = $3",
            player.nickname,
            player.enabled,
            player.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
