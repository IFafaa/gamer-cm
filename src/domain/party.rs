use chrono::{DateTime, Utc};

use super::{player::Player, team::Team};

pub struct Party {
    pub id: i32,
    pub game_name: String,
    pub players: Vec<Player>,
    pub teams: Vec<Team>,
    pub winner: Option<Team>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl Party {
    pub fn new(game_name: String) -> Self {
        Party {
            id: 0,
            players: Vec::new(),
            teams: Vec::new(),
            winner: None,
            game_name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            finished_at: None,
        }
    }
}
