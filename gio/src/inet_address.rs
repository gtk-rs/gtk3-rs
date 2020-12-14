// Take a look at the license at the top of the repository in the LICENSE file.

use crate::InetAddress;
use crate::InetAddressExt;
use crate::SocketFamily;
use glib::object::IsA;
use glib::translate::*;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
pub enum InetAddressBytes<'a> {
    V4(&'a [u8; 4]),
    V6(&'a [u8; 16]),
}

impl<'a> InetAddressBytes<'a> {
    fn deref(&self) -> &[u8] {
        use self::InetAddressBytes::*;

        match *self {
            V4(bytes) => bytes,
            V6(bytes) => bytes,
        }
    }
}

impl InetAddress {
    pub fn from_bytes(inet_address_bytes: InetAddressBytes) -> Self {
        let bytes = inet_address_bytes.deref();

        let family = match inet_address_bytes {
            InetAddressBytes::V4(_) => SocketFamily::Ipv4,
            InetAddressBytes::V6(_) => SocketFamily::Ipv6,
        };
        unsafe {
            from_glib_full(ffi::g_inet_address_new_from_bytes(
                bytes.to_glib_none().0,
                family.to_glib(),
            ))
        }
    }
}

pub trait InetAddressExtManual {
    fn to_bytes(&self) -> Option<InetAddressBytes<'_>>;
}

impl<O: IsA<InetAddress>> InetAddressExtManual for O {
    /// Returns `None` in case the address has a native size different than 4 and 16.
    fn to_bytes(&self) -> Option<InetAddressBytes<'_>> {
        let size = self.get_native_size();
        unsafe {
            let bytes = ffi::g_inet_address_to_bytes(self.as_ref().to_glib_none().0);
            if size == 4 {
                Some(InetAddressBytes::V4(&*(bytes as *const [u8; 4])))
            } else if size == 16 {
                Some(InetAddressBytes::V6(&*(bytes as *const [u8; 16])))
            } else {
                None
            }
        }
    }
}

impl From<IpAddr> for InetAddress {
    fn from(addr: IpAddr) -> Self {
        match addr {
            IpAddr::V4(v4) => InetAddress::from_bytes(InetAddressBytes::V4(&v4.octets())),
            IpAddr::V6(v6) => InetAddress::from_bytes(InetAddressBytes::V6(&v6.octets())),
        }
    }
}

impl From<InetAddress> for IpAddr {
    fn from(addr: InetAddress) -> IpAddr {
        let size = addr.get_native_size();
        unsafe {
            let bytes = ffi::g_inet_address_to_bytes(addr.to_glib_none().0);
            if size == 4 {
                IpAddr::V4(Ipv4Addr::from(*(bytes as *const [u8; 4])))
            } else if size == 16 {
                IpAddr::V6(Ipv6Addr::from(*(bytes as *const [u16; 8])))
            } else {
                panic!("Unknown IP kind");
            }
        }
    }
}
