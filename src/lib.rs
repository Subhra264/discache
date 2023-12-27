use network::CacheNetwork;
use std::marker::PhantomData;

pub mod rpc {
    tonic::include_proto!("api");
}

pub mod cache;
pub mod network;
pub mod utils;

pub enum RPCServer {}
pub enum HTTPServer {}

pub trait Server {}
impl Server for RPCServer {}
impl Server for HTTPServer {}

// RPC server for the Cache Cluster
pub struct CacheClusterServer<T = RPCServer>
where
    T: Server,
{
    network: CacheNetwork,
    pd: PhantomData<T>,
}

impl<T> CacheClusterServer<T>
where
    T: Server,
{
    pub fn new() -> Self {
        CacheClusterServer {
            network: CacheNetwork::new(),
            pd: PhantomData,
        }
    }
}

impl CacheClusterServer {
    pub fn run(&self) {
        // TODO: Run this client
    }
}

impl CacheClusterServer<HTTPServer> {}

// RPC server for the Cache
pub struct CacheServer<T = RPCServer>
where
    T: Server,
{
    pd: PhantomData<T>,
}

// impl<K, V> for CacheServer<K, V> {}
