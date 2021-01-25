// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;

bitflags::bitflags! {
    pub struct ParamFlags: u32 {
        const READABLE = 1;
        const WRITABLE = 2;
        const READWRITE = 3;
        const CONSTRUCT = 4;
        const CONSTRUCT_ONLY = 8;
        const LAX_VALIDATION = 16;
        const USER_0 = 128;
        const USER_1 = 256;
        const USER_2 = 1024;
        const USER_3 = 2048;
        const USER_4 = 4096;
        const USER_5 = 8192;
        const USER_6 = 16384;
        const USER_7 = 32768;
        const USER_8 = 65536;
        const EXPLICIT_NOTIFY = 1073741824;
        const DEPRECATED = 2147483648;
    }
}

#[doc(hidden)]
impl ToGlib for ParamFlags {
    type GlibType = gobject_ffi::GParamFlags;

    fn to_glib(&self) -> gobject_ffi::GParamFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gobject_ffi::GParamFlags> for ParamFlags {
    unsafe fn from_glib(value: gobject_ffi::GParamFlags) -> ParamFlags {
        ParamFlags::from_bits_truncate(value)
    }
}
