use derive_more::Constructor;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Constructor)]
pub struct Tunnel {
    tunnel_id: String,
    tunnel_type: TunnelType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TunnelType {
    Tcp,
    Udp,
}
