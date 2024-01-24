use cache::{network::CacheNetwork, CacheClusterServer};
use clap::{Parser, ValueEnum};
use std::fmt::Display;
use std::vec::Vec;

#[derive(Debug, Clone, ValueEnum)]
enum ServerType {
    Grpc,
}

impl Display for ServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Grpc => f.write_str("grpc"),
        }
    }
}

#[derive(Debug, Parser)]
#[command(author="Subhradeep Chakraborty", version, about, long_about = None)]
/// CLI to configure and manage the cluster of Cache nodes
struct Args {
    #[arg(short, long, default_value_t = ServerType::Grpc)]
    server: ServerType,

    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,

    #[arg(short, long, default_value_t = 7000)]
    port: u16,

    #[arg(short, long)]
    nodes: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut nodes = vec![];
    let addr = format!("{host}:{port}", host = args.host, port = args.port);

    for node in &args.nodes {
        // For now no consideration of node weights
        nodes.push((node.as_str(), 1 as usize));
    }
    let cache_network = CacheNetwork::with_servers(nodes)?;
    CacheClusterServer::new(cache_network).run(&addr).await?;
    Ok(())
}
