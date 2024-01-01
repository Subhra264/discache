use std::net::{SocketAddr, ToSocketAddrs};

#[derive(Debug)]
pub enum Error {
    NotValidAddress,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotValidAddress => f.write_str("given address not valid"),
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

    pub fn add_node(&mut self, node: ServerNode) {
        self.nodes.push(node);
    }
}

pub struct ServerNode {
    host: String,
    port: u16,
    weight: usize,
}

impl ServerNode {
    pub fn new(host: String, port: u16, weight: usize) -> Self {
        ServerNode { host, port, weight }
    }

    pub fn parse(addr: &str, weight: usize) -> Result<Self, Error> {
        if let Ok(socket_addr) = addr.to_socket_addrs() {
            match socket_addr.collect::<Vec<_>>().first() {
                Some(SocketAddr::V4(v4)) => Ok(ServerNode {
                    host: v4.ip().to_string(),
                    port: v4.port(),
                    weight,
                }),
                Some(SocketAddr::V6(v6)) => Ok(ServerNode {
                    host: v6.ip().to_string(),
                    port: v6.port(),
                    weight,
                }),
                None => Err(Error::NotValidAddress),
            }
        } else {
            Err(Error::NotValidAddress)
        }
    }

    pub fn get_host(&self) -> String {
        self.host.clone()
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_weight(&self) -> usize {
        self.weight
    }
}
