use derive_more::{Constructor, Display};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Constructor, Eq, PartialEq, Hash, Display, Clone)]
#[display(fmt = "{}://[{}]<=>[{:?}]", tunnel_type, agent_edge_id, proxy_edge_id)]
pub struct TunnelInfo {
    pub agent_edge_id: String,
    pub proxy_edge_id: Option<String>,
    pub tunnel_type: TunnelType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Display, Clone, Copy)]
pub enum TunnelType {
    #[display(fmt = "tcp")]
    Tcp,
    #[display(fmt = "udp")]
    Udp,
}
