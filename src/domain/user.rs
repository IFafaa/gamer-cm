pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: time::PrimitiveDateTime,
    pub updated_at: time::PrimitiveDateTime,
    pub enabled: bool,
}
