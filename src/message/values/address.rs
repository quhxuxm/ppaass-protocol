use crate::error::ProtocolError;
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Display)]
pub enum UnifiedAddress {
    #[display("{_0:?}")]
    Ip(SocketAddr),
    #[display("{}:{}", host, port)]
    Domain { host: String, port: u16 },
}

impl PartialEq for UnifiedAddress {
    fn eq(&self, other: &Self) -> bool {
        match self {
            UnifiedAddress::Ip(ip_addr) => match other {
                UnifiedAddress::Ip(other_ip_addr) => ip_addr.eq(other_ip_addr),
                UnifiedAddress::Domain { .. } => false,
            },
            UnifiedAddress::Domain { host, port } => match other {
                UnifiedAddress::Ip(_) => false,
                UnifiedAddress::Domain {
                    host: other_host,
                    port: other_port,
                } => host == other_host && port == other_port,
            },
        }
    }
}

pub struct SocketAddrIter {
    elements: Vec<SocketAddr>,
    index: usize,
}

impl SocketAddrIter {
    pub fn new(elements: Vec<SocketAddr>) -> Self {
        Self { elements, index: 0 }
    }
}

impl Iterator for SocketAddrIter {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.elements.get(self.index);
        self.index += 1;
        result.copied()
    }
}

impl ToSocketAddrs for UnifiedAddress {
    type Iter = SocketAddrIter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        let socket_addr_vec: Vec<SocketAddr> = self
            .clone()
            .try_into()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(SocketAddrIter::new(socket_addr_vec))
    }
}

impl TryFrom<UnifiedAddress> for Vec<SocketAddr> {
    type Error = ProtocolError;

    fn try_from(value: UnifiedAddress) -> Result<Self, Self::Error> {
        match value {
            UnifiedAddress::Ip(socket_addr) => Ok(vec![socket_addr]),
            UnifiedAddress::Domain { host, port } => {
                let address_string = format!("{host}:{port}");
                let addresses = address_string.to_socket_addrs()?;
                let addresses = addresses.collect::<Vec<_>>();
                Ok(addresses)
            }
        }
    }
}

impl From<SocketAddr> for UnifiedAddress {
    fn from(value: SocketAddr) -> Self {
        UnifiedAddress::Ip(value)
    }
}
