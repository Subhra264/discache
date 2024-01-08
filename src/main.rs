use cache::{network::CacheNetwork, CacheClusterServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache_network = CacheNetwork::with_servers(vec![("localhost:4001", 1)])?;
    CacheClusterServer::new(cache_network)
        .run("[::1:4000]")
        .await?;
    Ok(())
}
