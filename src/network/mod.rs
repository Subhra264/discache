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
