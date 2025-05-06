use sqlx::postgres::PgPoolOptions;

pub async fn config_database() -> sqlx::Result<sqlx::PgPool> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}
