// Take a look at the license at the top of the repository in the LICENSE file.

use crate::CellRendererPixbuf;
use crate::IconSize;
use glib::object::{IsA, ObjectExt};
use glib::translate::*;

pub trait CellRendererPixbufExtManual: 'static {
    #[doc(alias = "get_property_stock_size")]
    fn property_stock_size(&self) -> IconSize;

    fn set_property_stock_size(&self, stock_size: IconSize);
}

impl<O: IsA<CellRendererPixbuf> + IsA<glib::object::Object>> CellRendererPixbufExtManual for O {
    fn property_stock_size(&self) -> IconSize {
        unsafe { from_glib(self.property::<u32>("stock-size") as i32) }
    }

    fn set_property_stock_size(&self, stock_size: IconSize) {
        self.set_property("stock-size", stock_size.into_glib() as u32);
    }
}
