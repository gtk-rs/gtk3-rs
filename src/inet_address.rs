use gio_sys;
use glib::translate::*;
use InetAddress;
use SocketFamily;

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
    pub fn new_from_bytes(inet_address_bytes: InetAddressBytes) -> Self {
        use self::InetAddressBytes::*;

        let bytes = inet_address_bytes.deref();

        let family = match inet_address_bytes {
            V4(_) => SocketFamily::Ipv4,
            V6(_) => SocketFamily::Ipv6,
        };
        unsafe {
            from_glib_full(gio_sys::g_inet_address_new_from_bytes(
                bytes.to_glib_none().0,
                family.to_glib(),
            ))
        }
    }
}
