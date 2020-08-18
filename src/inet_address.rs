use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use InetAddress;
use InetAddressExt;
use SocketFamily;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::slice::from_raw_parts;

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
            from_glib_full(gio_sys::g_inet_address_new_from_bytes(
                bytes.to_glib_none().0,
                family.to_glib(),
            ))
        }
    }
}

pub trait InetAddressExtManual {
    fn to_bytes<'a>(&'a self) -> Option<InetAddressBytes<'a>>;
}

impl<O: IsA<InetAddress>> InetAddressExtManual for O {
    /// Returns `None` in case the address has a native size different than 4 and 16.
    fn to_bytes<'a>(&'a self) -> Option<InetAddressBytes<'a>> {
        let size = self.get_native_size();
        unsafe {
            let bytes = gio_sys::g_inet_address_to_bytes(self.as_ref().to_glib_none().0);
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
