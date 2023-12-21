use cache::Cache;
use network::CacheNetwork;

pub mod cache;
pub mod network;
pub mod utils;

// HTTP Client for the Cache Cluster
pub struct CacheClusterClient {
    network: CacheNetwork,
}

impl CacheClusterClient {
    pub fn new() -> Self {
        CacheClusterClient {
            network: CacheNetwork::new(),
        }
    }

    pub fn run(&self) {
        // TODO: Run this client
    }
}

// HTTP Client for the single Cache Server
pub struct CacheClient {}
