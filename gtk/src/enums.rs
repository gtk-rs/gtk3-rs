use glib::translate::{from_glib, ToGlib};
use gtk_sys;
use IconSize;
use ResponseType;

impl IconSize {
    pub fn unscaled() -> IconSize {
        skip_assert_initialized!();
        IconSize::__Unknown(-1)
    }
}

impl From<IconSize> for i32 {
    fn from(val: IconSize) -> i32 {
        skip_assert_initialized!();
        val.to_glib() as i32
    }
}

impl From<i32> for IconSize {
    fn from(val: i32) -> Self {
        skip_assert_initialized!();
        from_glib(val as gtk_sys::GtkIconSize)
    }
}

impl From<ResponseType> for i32 {
    fn from(val: ResponseType) -> i32 {
        skip_assert_initialized!();
        val.to_glib() as i32
    }
}

impl From<i32> for ResponseType {
    fn from(val: i32) -> Self {
        skip_assert_initialized!();
        from_glib(val as gtk_sys::GtkResponseType)
    }
}
