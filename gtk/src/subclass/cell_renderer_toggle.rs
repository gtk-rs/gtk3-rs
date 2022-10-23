// Take a look at the license at the top of the repository in the LICENSE file.

use libc::c_char;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString};

use super::cell_renderer::CellRendererImpl;

use crate::CellRendererToggle;

pub trait CellRendererToggleImpl: CellRendererToggleImplExt + CellRendererImpl {
    fn toggled(&self, path: &str) {
        self.parent_toggled(path);
    }
}

pub trait CellRendererToggleImplExt: ObjectSubclass {
    fn parent_toggled(&self, path: &str);
}

impl<T: CellRendererToggleImpl> CellRendererToggleImplExt for T {
    fn parent_toggled(&self, path: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererToggleClass;
            if let Some(f) = (*parent_class).toggled {
                f(
                    self.obj()
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
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        let klass = class.as_mut();
        klass.toggled = Some(cell_renderer_toggle_toggled::<T>);
    }
}

unsafe extern "C" fn cell_renderer_toggle_toggled<T: CellRendererToggleImpl>(
    ptr: *mut ffi::GtkCellRendererToggle,
    path: *const c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.toggled(&GString::from_glib_borrow(path))
}
