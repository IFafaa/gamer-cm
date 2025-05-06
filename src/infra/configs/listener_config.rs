use tokio::net::TcpListener;

pub async fn listener_config() -> Result<TcpListener, Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener: TcpListener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    Ok(listener)
}
