use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Constructor)]
pub struct Tunnel {
    agent_edge_id: String,
    proxy_edge_id: Option<String>,
    tunnel_type: TunnelType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TunnelType {
    Tcp,
    Udp,
}
