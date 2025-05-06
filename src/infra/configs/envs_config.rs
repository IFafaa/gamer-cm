pub fn config_env() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    Ok(())
}
