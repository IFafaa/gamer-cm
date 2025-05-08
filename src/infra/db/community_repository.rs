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
    async fn save(&self, community: &Community) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE communities SET name = $1, enabled = $2, updated_at = NOW() WHERE id = $3",
            community.name,
            community.enabled,
            community.id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    async fn insert(&self, community: &Community) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO communities (name) VALUES ($1)", community.name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_all(&self) -> anyhow::Result<Vec<Community>> {
        let result = sqlx::query!(
            "SELECT id, name, created_at, updated_at FROM communities WHERE enabled = true"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut communities = Vec::new();
        for row in result {
            let players = sqlx::query!(
                "SELECT id, nickname, community_id, created_at, updated_at FROM players WHERE community_id = $1 AND enabled = true",
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
                    enabled: true,
                })
                .collect();

            communities.push(Community {
                id: row.id,
                name: row.name,
                players,
                created_at: row.created_at,
                updated_at: row.updated_at,
                enabled: true,
            });
        }
        Ok(communities)
    }

    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Community>> {
        let row = sqlx::query!(
            "SELECT id, name, created_at, updated_at, enabled FROM communities WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let players = sqlx::query!(
                "SELECT id, nickname, community_id, created_at, updated_at FROM players WHERE community_id = $1 AND enabled = true",
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
                    enabled: true,
                })
                .collect();

            Ok(Some(Community {
                id: row.id,
                name: row.name,
                players,
                created_at: row.created_at,
                updated_at: row.updated_at,
                enabled: row.enabled,
            }))
        } else {
            Ok(None)
        }
    }

    async fn exists(&self, name: String) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM communities WHERE name = $1 AND enabled = true) AS exists",
            name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.exists.unwrap_or(false))
    }
}
