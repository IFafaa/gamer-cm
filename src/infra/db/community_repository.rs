use crate::domain::{
    community::{Community, CommunityRepository},
    player::Player,
};
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

    async fn get_all(&self) -> anyhow::Result<Vec<Community>> {
        let result = sqlx::query!("SELECT id, name, created_at, updated_at FROM communities")
            .fetch_all(&self.pool)
            .await?;

        let mut communities = Vec::new();
        for row in result {
            let players = sqlx::query!(
                "SELECT id, nickname, community_id, created_at, updated_at FROM players WHERE community_id = $1",
                row.id
            )
            .fetch_all(&self.pool)
            .await?;

            let players: Vec<Player> = players
                .into_iter()
                .map(|p| Player {
                    id: p.id,
                    nickname: p.nickname,
                    community_id: p.community_id.unwrap_or(0),
                    created_at: p.created_at,
                    updated_at: p.updated_at,
                })
                .collect();

            communities.push(Community {
                id: row.id,
                name: row.name,
                players,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        Ok(communities)
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
