use serde::Serialize;

use crate::domain::community::Community;

#[derive(Serialize)]
pub struct IResultGetCommunity {
    id: i32,
    name: String,
    created_at: String,
    updated_at: String,
    players: Vec<IResultPlayer>,
    teams: Vec<IResultTeams>,
}

#[derive(Serialize)]
struct IResultTeams {
    id: i32,
    name: String,
    players: Vec<IResultPlayer>,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize)]
struct IResultPlayer {
    id: i32,
    nickname: String,
    created_at: String,
    updated_at: String,
}

impl IResultGetCommunity {
    pub fn new(community: Community) -> Self {
        IResultGetCommunity {
            id: community.id,
            name: community.name,
            created_at: community.created_at.to_string(),
            updated_at: community.updated_at.to_string(),
            players: community
                .players
                .into_iter()
                .map(|player| IResultPlayer {
                    id: player.id,
                    nickname: player.nickname,
                    created_at: player.created_at.to_string(),
                    updated_at: player.updated_at.to_string(),
                })
                .collect(),
            teams: community
                .teams
                .into_iter()
                .map(|team| IResultTeams {
                    id: team.id,
                    name: team.name,
                    players: team
                        .players
                        .into_iter()
                        .map(|player| IResultPlayer {
                            id: player.id,
                            nickname: player.nickname,
                            created_at: player.created_at.to_string(),
                            updated_at: player.updated_at.to_string(),
                        })
                        .collect(),
                    created_at: team.created_at.to_string(),
                    updated_at: team.updated_at.to_string(),
                })
                .collect(),
        }
    }
}
