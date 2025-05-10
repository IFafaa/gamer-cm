use serde::Serialize;

use crate::{domain::community::Community, shared::api_response::ApiResponse};

#[derive(Serialize, Default)]

pub struct IResultGetCommunityById {
    id: i32,
    name: String,
    created_at: String,
    updated_at: String,
    players: Vec<IResultPlayer>,
}
#[derive(Serialize, Default)]

struct IResultPlayer {
    id: i32,
    nickname: String,
    created_at: String,
    updated_at: String,
}
impl IResultGetCommunityById {
    pub fn new(community: Community) -> ApiResponse<Self> {
        let data = IResultGetCommunityById {
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
        };
        ApiResponse::success(data)
    }
}
