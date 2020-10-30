use gtk_sys;

use libc::c_char;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::GString;

use super::cell_renderer::CellRendererImpl;
use CellRenderer;
use CellRendererToggle;

pub trait CellRendererToggleImpl: CellRendererToggleImplExt + CellRendererImpl {
    fn toggled(&self, renderer: &CellRendererToggle, path: &str) {
        self.parent_toggled(renderer, path);
    }
}

pub trait CellRendererToggleImplExt {
    fn parent_toggled(&self, renderer: &CellRendererToggle, path: &str);
}

impl<T: CellRendererToggleImpl> CellRendererToggleImplExt for T {
    fn parent_toggled(&self, renderer: &CellRendererToggle, path: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererToggleClass;
            if let Some(f) = (*parent_class).toggled {
                f(renderer.to_glib_none().0, path.to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: CellRendererToggleImpl> IsSubclassable<T> for CellRendererToggle {
    fn override_vfuncs(class: &mut ::glib::object::Class<Self>) {
        <CellRenderer as IsSubclassable<T>>::override_vfuncs(class);
        unsafe {
            let klass = &mut *(class as *mut _ as *mut gtk_sys::GtkCellRendererToggleClass);
            klass.toggled = Some(cell_renderer_toggle_toggled::<T>);
        }
    }
}

unsafe extern "C" fn cell_renderer_toggle_toggled<T: CellRendererToggleImpl>(
    ptr: *mut gtk_sys::GtkCellRendererToggle,
    path: *const c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.toggled(&wrap, &GString::from_glib_borrow(path))
}
