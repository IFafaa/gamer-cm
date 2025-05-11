use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AddTeamIntoCommunityDto {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Name must be between 1 and 50 characters"
    ))]
    pub nickname: String,

    #[validate(range(min = 1, message = "Community ID must be a positive integer"))]
    pub community_id: i32,
}
