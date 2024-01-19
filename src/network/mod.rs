use crate::{
    rpc::{self, cache_client::CacheClient, Key},
    utils::hash::{xxhash_64, xxhash_64_with_seed},
};
use std::net::{SocketAddr, ToSocketAddrs};
use tonic::{transport::Channel, Code, Request};

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

    pub async fn get_value(&mut self, key: String) -> Result<String, Error> {
        let node_index = self.find_node_with_key(key.as_str())?;
        self.nodes[node_index].get(key).await
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

    pub async fn get(&mut self, key: String) -> Result<String, Error> {
        if let Some(conn) = &mut self.client {
            match conn.get(Request::new(Key { key })).await {
                Ok(resp) => {
                    let resp = resp.into_inner();
                    match resp.value {
                        Some(value) => Ok(value.value),
                        None => Err(Error::Unknown),
                    }
                }
                Err(status) => match status.code() {
                    Code::NotFound => Err(Error::EntryNotFound(status.message().to_string())),
                    _ => Err(Error::Reason(status.message().to_string())),
                },
            }
        } else {
            Err(Error::NodeCouldNotBeConnected(self.address()))
        }
    }
}
