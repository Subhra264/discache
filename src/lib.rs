use cache::Cache;
use network::CacheNetwork;
use rpc::{Entry, GetResponse, Key, PutResponse};
use std::marker::PhantomData;
use tokio::sync::Mutex;
use tonic::{Request, Response, Result, Status};

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
pub struct CacheServer<C, T = RPCServer>
where
    C: Cache<String, String>,
    T: Server,
{
    cache: Mutex<C>,
    pd: PhantomData<T>,
}

impl<C, T> CacheServer<C, T>
where
    C: Cache<String, String> + Send + 'static,
    T: Server,
{
    pub fn new(cache: C) -> Self {
        CacheServer::<C, T> {
            cache: Mutex::new(cache),
            pd: PhantomData,
        }
    }
}

#[tonic::async_trait]
impl<C> rpc::cache_server::Cache for CacheServer<C>
where
    C: Cache<String, String> + Send + 'static,
{
    async fn get(&self, request: Request<Key>) -> Result<Response<GetResponse>> {
        let key = request.into_inner().key;

        let mut cache = self.cache.lock().await;
        match cache.get(&key) {
            Some(value) => Ok(Response::new(GetResponse {
                code: 0,
                value: value.clone(),
                message: "success".to_string(),
            })),
            None => Err(Status::not_found("key not found")),
        }
    }

    async fn put(&self, request: Request<Entry>) -> Result<Response<PutResponse>> {
        let Entry { key, value } = request.into_inner();
        let mut cache = self.cache.lock().await;
        match cache.put(key, value) {
            Ok(()) => Ok(Response::new(PutResponse {
                code: 0,
                message: "success".to_string(),
            })),
            Err(msg) => Err(Status::internal(msg)),
        }
    }
}
