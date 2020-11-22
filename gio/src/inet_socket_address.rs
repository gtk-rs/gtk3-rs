use crate::InetAddress;
use crate::InetSocketAddress;
use crate::InetSocketAddressExt;

use std::net::SocketAddr;

impl From<SocketAddr> for InetSocketAddress {
    fn from(addr: SocketAddr) -> Self {
        InetSocketAddress::new::<InetAddress>(&addr.ip().into(), addr.port())
    }
}

impl From<InetSocketAddress> for SocketAddr {
    fn from(addr: InetSocketAddress) -> Self {
        SocketAddr::new(addr.get_address().unwrap().into(), addr.get_port())
    }
}
