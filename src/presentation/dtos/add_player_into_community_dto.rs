use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AddPlayerIntoCommunityDto {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Nickname must be between 1 and 50 characters"
    ))]
    pub nickname: String,

    #[validate(range(min = 1, message = "Community ID must be greater than 0"))]
    pub community_id: i32,
}
