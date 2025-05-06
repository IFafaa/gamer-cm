use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddPlayerIntoCommunityDto {
    pub nickname: String,
    pub community_id: i32,
}
