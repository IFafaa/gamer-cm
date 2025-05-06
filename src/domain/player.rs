use serde::Serialize;
use time::PrimitiveDateTime;

use crate::shared::date_time::DateTime;

#[derive(Serialize)]
pub struct Player {
    pub id: i32,
    pub nickname: String,
    pub community_id: i32,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl Player {
    pub fn new(nickname: String, community_id: i32) -> Self {
        Player {
            id: 0,
            nickname,
            community_id,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
