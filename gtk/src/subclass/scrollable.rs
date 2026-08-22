// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Scrollable`] interface.

use glib::translate::*;

use crate::{Border, Scrollable, ffi, prelude::*, subclass::prelude::*};

pub trait ScrollableImpl: ObjectImpl + ObjectSubclass<Type: IsA<Scrollable>> {
    #[doc(alias = "get_border")]
    fn border(&self) -> Option<Border> {
        self.parent_border()
    }
}

pub trait ScrollableImplExt: ScrollableImpl {
    fn parent_border(&self) -> Option<Border> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Scrollable>()
                as *const ffi::GtkScrollableInterface;

            (*parent_iface).get_border.and_then(|func| {
                let mut border = Border::uninitialized();
                from_glib::<_, bool>(func(
                    self.obj().unsafe_cast_ref::<Scrollable>().to_glib_none().0,
                    border.to_glib_none_mut().0,
                ))
                .then_some(border)
            })
        }
    }
}

impl<T: ScrollableImpl> ScrollableImplExt for T {}

unsafe impl<T: ScrollableImpl> IsImplementable<T> for Scrollable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        iface.get_border = Some(scrollable_get_border::<T>);
    }
}

unsafe extern "C" fn scrollable_get_border<T: ScrollableImpl>(
    scrollableptr: *mut ffi::GtkScrollable,
    borderptr: *mut ffi::GtkBorder,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(scrollableptr as *mut T::Instance);
        let imp = instance.imp();

        if let Some(border) = imp.border() {
            if !borderptr.is_null() {
                *borderptr = *border;
            }
            glib::ffi::GTRUE
        } else {
            glib::ffi::GFALSE
        }
    }
}
