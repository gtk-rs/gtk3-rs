// Take a look at the license at the top of the repository in the LICENSE file.

use crate::CellRendererPixbuf;
use crate::IconSize;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::Value;

pub trait CellRendererPixbufExtManual: 'static {
    #[doc(alias = "get_property_stock_size")]
    fn property_stock_size(&self) -> IconSize;

    /// The GtkIconSize value that specifies the size of the rendered icon.
    fn set_property_stock_size(&self, stock_size: IconSize);
}

impl<O: IsA<CellRendererPixbuf> + IsA<glib::object::Object>> CellRendererPixbufExtManual for O {
    fn property_stock_size(&self) -> IconSize {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut _,
                "stock-size".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            from_glib(
                value
                    .get::<u32>()
                    .expect("Return Value for property `stock_size` getter") as i32,
            )
        }
    }

    fn set_property_stock_size(&self, stock_size: IconSize) {
        unsafe {
            let value = Value::from(&(stock_size.into_glib() as u32));
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut _,
                "stock-size".to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}
