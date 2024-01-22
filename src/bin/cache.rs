use cache::{
    cache::{lru::LRUCache, Cache},
    CacheServer,
};
use clap::{Parser, ValueEnum};
use std::fmt::Display;

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

#[derive(Debug, Clone, ValueEnum)]
enum CacheType {
    Lru,
}

impl Display for CacheType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lru => f.write_str("lru"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author="Subhradeep Chakraborty", version, about, long_about = None)]
/// Fast, asynchronous cache server
struct Args {
    #[arg(short, long, default_value_t = ServerType::Grpc)]
    server: ServerType,

    #[arg(short, long, default_value_t = 100)]
    capacity: usize,

    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,

    #[arg(short, long, default_value_t = 8000)]
    port: u16,

    #[arg(long, default_value_t = CacheType::Lru)]
    cache: CacheType,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.server {
        ServerType::Grpc => {
            let cache = LRUCache::<String, String>::new(args.capacity);
            let addr = format!("{host}:{port}", host = args.host, port = args.port);
            CacheServer::run(addr.as_str(), cache).await?;
        }
    }

    Ok(())
}
