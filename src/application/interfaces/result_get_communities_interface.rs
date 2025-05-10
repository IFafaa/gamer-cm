use serde::Serialize;

use crate::{domain::community::Community, shared::api_response::ApiResponse};

#[derive(Serialize)]

pub struct IResultGetCommunities {
    id: i32,
    name: String,
    created_at: String,
    updated_at: String,
    players: Vec<IResultPlayer>,
}
#[derive(Serialize)]

struct IResultPlayer {
    id: i32,
    nickname: String,
    created_at: String,
    updated_at: String,
}
impl IResultGetCommunities {
    pub fn new(communities: Vec<Community>) -> ApiResponse<Vec<Self>> {
        let data = communities
            .into_iter()
            .map(|community| IResultGetCommunities {
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
            })
            .collect();
        ApiResponse::success(data)
    }
}
