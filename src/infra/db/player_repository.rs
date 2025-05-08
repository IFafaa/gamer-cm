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
}
