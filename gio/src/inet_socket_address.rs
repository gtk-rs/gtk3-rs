// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::InetAddress;
use crate::InetSocketAddress;

use std::net::SocketAddr;

impl From<SocketAddr> for InetSocketAddress {
    fn from(addr: SocketAddr) -> Self {
        Self::new::<InetAddress>(&addr.ip().into(), addr.port())
    }
}

impl From<InetSocketAddress> for SocketAddr {
    fn from(addr: InetSocketAddress) -> Self {
        Self::new(addr.address().into(), addr.port())
    }
}
