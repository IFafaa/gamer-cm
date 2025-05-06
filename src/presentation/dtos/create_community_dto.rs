use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateCommunityDto {
    pub name: String,
}
