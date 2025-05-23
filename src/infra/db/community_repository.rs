use crate::domain::{
    community::{Community, CommunityRepository},
    player::Player,
    team::Team,
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

    async fn fetch_players_by_community(&self, community_id: i32) -> anyhow::Result<Vec<Player>> {
        let rows = sqlx::query!(
            "SELECT id, nickname, community_id, created_at, updated_at FROM players 
             WHERE community_id = $1 AND enabled = true",
            community_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|p| Player {
                id: p.id,
                nickname: p.nickname,
                community_id: p.community_id,
                created_at: p.created_at,
                updated_at: p.updated_at,
                enabled: true,
            })
            .collect())
    }

    async fn fetch_team_players(
        &self,
        community_id: i32,
        team_id: i32,
    ) -> anyhow::Result<Vec<Player>> {
        let rows = sqlx::query!(
            "SELECT id, nickname, community_id, created_at, updated_at FROM players 
             WHERE community_id = $1 AND enabled = true 
             AND id IN (SELECT player_id FROM team_players WHERE team_id = $2)",
            community_id,
            team_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|p| Player {
                id: p.id,
                nickname: p.nickname,
                community_id: p.community_id,
                created_at: p.created_at,
                updated_at: p.updated_at,
                enabled: true,
            })
            .collect())
    }

    async fn fetch_teams_by_community(&self, community_id: i32) -> anyhow::Result<Vec<Team>> {
        let team_rows = sqlx::query!(
            "SELECT id, name, community_id, created_at, updated_at 
             FROM teams WHERE community_id = $1 AND enabled = true",
            community_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut teams = Vec::new();

        for t in team_rows {
            let players = self.fetch_team_players(community_id, t.id).await?;

            teams.push(Team {
                id: t.id,
                name: t.name,
                community_id: t.community_id,
                players,
                created_at: t.created_at,
                updated_at: t.updated_at,
                enabled: true,
            });
        }

        Ok(teams)
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
        let rows = sqlx::query!(
            "SELECT id, name, created_at, updated_at, enabled 
             FROM communities WHERE enabled = true"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut communities = Vec::new();

        for row in rows {
            let players = self.fetch_players_by_community(row.id).await?;
            let teams = self.fetch_teams_by_community(row.id).await?;

            communities.push(Community {
                id: row.id,
                name: row.name,
                players,
                teams,
                created_at: row.created_at,
                updated_at: row.updated_at,
                enabled: row.enabled,
            });
        }

        Ok(communities)
    }

    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<Community>> {
        let row = sqlx::query!(
            "SELECT id, name, created_at, updated_at, enabled 
             FROM communities WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let players = self.fetch_players_by_community(row.id).await?;
            let teams = self.fetch_teams_by_community(row.id).await?;

            return Ok(Some(Community {
                id: row.id,
                name: row.name,
                players,
                teams,
                created_at: row.created_at,
                updated_at: row.updated_at,
                enabled: row.enabled,
            }));
        }

        Ok(None)
    }

    async fn exists(&self, name: String) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(
                SELECT 1 FROM communities WHERE name = $1 AND enabled = true
            ) AS exists",
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }
}
