use crate::domain::community::{Community, CommunityRepository};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgCommunityRepository {
    pub pool: PgPool,
}

impl PgCommunityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommunityRepository for PgCommunityRepository {
    async fn insert(&self, community: &Community) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO communities (name) VALUES ($1)", community.name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn exists(&self, name: String) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM communities WHERE name = $1) AS exists",
            name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.exists.unwrap_or(false))
    }
}
