use crate::{
    rpc::{self, cache_client::CacheClient, Entry, GetResponse, Key, PutResponse},
    utils::hash::{xxhash_64, xxhash_64_with_seed},
};
use std::net::{SocketAddr, ToSocketAddrs};
use tonic::{transport::Channel, Request, Response, Status};

#[derive(Debug)]
pub enum Error {
    NotValidAddress,
    CouldNotConnectNodes(Vec<String>),
    NoNodesRegistered,
    NodeCouldNotBeConnected(String),
    Unknown,
    EntryNotFound(String),
    Reason(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotValidAddress => f.write_str("given address not valid"),
            Error::CouldNotConnectNodes(ids) => {
                f.write_fmt(format_args!("Couldn't connect with {:?} servers", ids))
            }
            Error::NoNodesRegistered => f.write_str("no cache nodes are connected currently"),
            Error::NodeCouldNotBeConnected(addr) => {
                f.write_fmt(format_args!("Server {} could not be connected", addr))
            }
            Error::Unknown => f.write_str("failed due to unknown reason"),
            Error::EntryNotFound(msg) => f.write_str(&msg),
            Error::Reason(msg) => f.write_str(&msg),
        }
    }
}

impl std::error::Error for Error {}

pub struct CacheNetwork {
    nodes: Vec<ServerNode>,
}

impl CacheNetwork {
    pub fn new() -> Self {
        CacheNetwork { nodes: vec![] }
    }

    pub fn with_nodes(nodes: Vec<ServerNode>) -> Self {
        CacheNetwork { nodes }
    }

    pub fn with_servers(servers: Vec<(&str, usize)>) -> Result<Self, Error> {
        let mut network = Self::new();
        for server in servers {
            network.add_server(server.0, server.1)?;
        }
        Ok(network)
    }

    pub fn add_node(&mut self, node: ServerNode) {
        self.nodes.push(node);
    }

    pub fn add_server(&mut self, addr: &str, weight: usize) -> Result<(), Error> {
        let node = ServerNode::parse(addr, weight).unwrap();
        self.add_node(node);
        Ok(())
    }

    pub async fn connect_nodes(&mut self) -> Result<(), Error> {
        let mut error_ids = vec![];
        for node in &mut self.nodes {
            if node.connect().await.is_err() {
                error_ids.push(node.address());
            }
        }
        // TODO: Return errors for nodes that couldn't be connected
        Ok(())
    }

    pub fn find_node_with_key(&self, key: &str) -> Result<usize, Error> {
        if !self.nodes.is_empty() {
            let mut node_index = 0;
            let mut hash: u64 = 0;
            for (pos, node) in self.nodes.iter().enumerate() {
                if node.active {
                    let rendez_hash = xxhash_64_with_seed(key, node.id());
                    if rendez_hash > hash {
                        hash = rendez_hash;
                        node_index = pos;
                    }
                }
            }
            return Ok(node_index);
        }
        Err(Error::NoNodesRegistered)
    }

    pub async fn get_value(&mut self, key: Key) -> tonic::Result<Response<GetResponse>> {
        match self.find_node_with_key(&key.key) {
            Ok(node_index) => self.nodes[node_index].get(key).await,
            Err(err) => match err {
                Error::NoNodesRegistered => Err(Status::failed_precondition(
                    "no cache nodes are connected recently",
                )),
                _ => Err(Status::unknown("failed due to unknown reason")),
            },
        }
    }

    pub async fn put_entry(&mut self, entry: Entry) -> tonic::Result<Response<PutResponse>> {
        if let Some(key) = &entry.key {
            match self.find_node_with_key(&key.key) {
                Ok(node_index) => self.nodes[node_index].put(entry).await,
                Err(err) => match err {
                    Error::NoNodesRegistered => Err(Status::failed_precondition(
                        "no cache nodes are connected recently",
                    )),
                    _ => Err(Status::unknown("failed due to unknown reason")),
                },
            }
        } else {
            Err(Status::invalid_argument("key not given"))
        }
    }
}

pub struct ServerNode {
    id: u64,
    host: String,
    port: u16,
    weight: usize,
    active: bool,
    client: Option<rpc::cache_client::CacheClient<Channel>>,
}

impl ServerNode {
    fn address_from(mut host: String, port: u16) -> String {
        host.push_str(&port.to_string());
        host
    }

    pub fn new(host: String, port: u16, weight: usize) -> Self {
        let address = Self::address_from(host.clone(), port);
        ServerNode {
            id: xxhash_64(address.as_str()),
            host,
            port,
            weight,
            active: false,
            client: None,
        }
    }

    pub fn parse(addr: &str, weight: usize) -> Result<Self, Error> {
        if let Ok(socket_addr) = addr.to_socket_addrs() {
            match socket_addr.collect::<Vec<_>>().first() {
                Some(SocketAddr::V4(v4)) => {
                    Ok(ServerNode::new(v4.ip().to_string(), v4.port(), weight))
                }
                Some(SocketAddr::V6(v6)) => {
                    Ok(ServerNode::new(v6.ip().to_string(), v6.port(), weight))
                }
                None => Err(Error::NotValidAddress),
            }
        } else {
            Err(Error::NotValidAddress)
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn address(&self) -> String {
        Self::address_from(self.host(), self.port)
    }

    pub fn weight(&self) -> usize {
        self.weight
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.client.is_none() {
            self.client = Some(CacheClient::connect(self.address()).await?);
            self.active = true;
        }
        Ok(())
    }

    pub async fn get(&mut self, key: Key) -> tonic::Result<Response<GetResponse>> {
        if let Some(conn) = &mut self.client {
            conn.get(Request::new(key)).await
        } else {
            Err(Status::failed_precondition(format!(
                "node {} couldn't not be connected",
                self.address()
            )))
        }
    }

    pub async fn put(&mut self, entry: Entry) -> tonic::Result<Response<PutResponse>> {
        if let Some(conn) = &mut self.client {
            conn.put(Request::new(entry)).await
        } else {
            Err(Status::failed_precondition(format!(
                "node {} couldn't not be connected",
                self.address()
            )))
        }
    }
}
