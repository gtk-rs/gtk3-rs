// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(feature = "v2_66", feature = "dox"))]
use crate::auto::TlsChannelBindingType;
use crate::auto::TlsConnection;
#[cfg(any(feature = "v2_66", feature = "dox"))]
use glib::translate::*;
use glib::IsA;
#[cfg(any(feature = "v2_66", feature = "dox"))]
use std::ptr;

pub trait TlsConnectionManualExt {
    #[cfg(any(feature = "v2_66", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_tls_connection_get_channel_binding_data")]
    fn get_channel_binding_data(
        &self,
        type_: TlsChannelBindingType,
    ) -> Result<glib::ByteArray, glib::Error>;
}

impl<O: IsA<TlsConnection>> TlsConnectionManualExt for O {
    #[cfg(any(feature = "v2_66", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
    fn get_channel_binding_data(
        &self,
        type_: TlsChannelBindingType,
    ) -> Result<glib::ByteArray, glib::Error> {
        unsafe {
            let data = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::g_tls_connection_get_channel_binding_data(
                self.as_ptr() as *mut _,
                type_.to_glib(),
                data,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
