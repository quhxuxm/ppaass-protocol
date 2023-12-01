use derive_more::{Constructor, Display};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Constructor, Eq, PartialEq, Hash, Display)]
#[display(fmt = "{}://[{}]<=>[{:?}]", tunnel_type, agent_edge_id, proxy_edge_id)]
pub struct Tunnel {
    agent_edge_id: String,
    proxy_edge_id: Option<String>,
    tunnel_type: TunnelType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Display)]
pub enum TunnelType {
    #[display(fmt = "tcp")]
    Tcp,
    #[display(fmt = "udp")]
    Udp,
}
