// Take a look at the license at the top of the repository in the LICENSE file.

use libc::c_char;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString};

use super::cell_renderer::CellRendererImpl;
use crate::CellRenderer;
use crate::CellRendererToggle;

pub trait CellRendererToggleImpl: CellRendererToggleImplExt + CellRendererImpl {
    fn toggled(&self, renderer: &Self::Type, path: &str) {
        self.parent_toggled(renderer, path);
    }
}

pub trait CellRendererToggleImplExt: ObjectSubclass {
    fn parent_toggled(&self, renderer: &Self::Type, path: &str);
}

impl<T: CellRendererToggleImpl> CellRendererToggleImplExt for T {
    fn parent_toggled(&self, renderer: &Self::Type, path: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut ffi::GtkCellRendererToggleClass;
            if let Some(f) = (*parent_class).toggled {
                f(
                    renderer
                        .unsafe_cast_ref::<CellRendererToggle>()
                        .to_glib_none()
                        .0,
                    path.to_glib_none().0,
                )
            }
        }
    }
}

unsafe impl<T: CellRendererToggleImpl> IsSubclassable<T> for CellRendererToggle {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <CellRenderer as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.toggled = Some(cell_renderer_toggle_toggled::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <CellRenderer as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn cell_renderer_toggle_toggled<T: CellRendererToggleImpl>(
    ptr: *mut ffi::GtkCellRendererToggle,
    path: *const c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRendererToggle> = from_glib_borrow(ptr);

    imp.toggled(wrap.unsafe_cast_ref(), &GString::from_glib_borrow(path))
}
