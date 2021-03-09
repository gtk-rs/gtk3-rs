// Take a look at the license at the top of the repository in the LICENSE file.

use libc::{c_char, c_uint};

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString};

use super::cell_renderer_text::CellRendererTextImpl;
use crate::CellRendererAccel;
use crate::CellRendererText;

pub trait CellRendererAccelImpl: CellRendererAccelImplExt + CellRendererTextImpl {
    fn accel_edited(
        &self,
        renderer: &Self::Type,
        path: &str,
        accel_key: u32,
        accel_mods: gdk::ModifierType,
        hardware_keycode: u32,
    ) {
        self.parent_accel_edited(renderer, path, accel_key, accel_mods, hardware_keycode);
    }

    fn accel_cleared(&self, renderer: &Self::Type, path: &str) {
        self.parent_accel_cleared(renderer, path);
    }
}

pub trait CellRendererAccelImplExt: ObjectSubclass {
    fn parent_accel_edited(
        &self,
        renderer: &Self::Type,
        path: &str,
        accel_key: u32,
        accel_mods: gdk::ModifierType,
        hardware_keycode: u32,
    );
    fn parent_accel_cleared(&self, renderer: &Self::Type, path: &str);
}

impl<T: CellRendererAccelImpl> CellRendererAccelImplExt for T {
    fn parent_accel_edited(
        &self,
        renderer: &Self::Type,
        path: &str,
        accel_key: u32,
        accel_mods: gdk::ModifierType,
        hardware_keycode: u32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut ffi::GtkCellRendererAccelClass;
            if let Some(f) = (*parent_class).accel_edited {
                f(
                    renderer
                        .unsafe_cast_ref::<CellRendererAccel>()
                        .to_glib_none()
                        .0,
                    path.to_glib_none().0,
                    accel_key,
                    accel_mods.to_glib(),
                    hardware_keycode,
                )
            }
        }
    }

    fn parent_accel_cleared(&self, renderer: &Self::Type, path: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut ffi::GtkCellRendererAccelClass;
            if let Some(f) = (*parent_class).accel_cleared {
                f(
                    renderer
                        .unsafe_cast_ref::<CellRendererAccel>()
                        .to_glib_none()
                        .0,
                    path.to_glib_none().0,
                )
            }
        }
    }
}

unsafe impl<T: CellRendererAccelImpl> IsSubclassable<T> for CellRendererAccel {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <CellRendererText as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.accel_edited = Some(cell_renderer_accel_edited::<T>);
        klass.accel_cleared = Some(cell_renderer_accel_cleared::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <CellRendererText as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn cell_renderer_accel_edited<T: CellRendererAccelImpl>(
    ptr: *mut ffi::GtkCellRendererAccel,
    path: *const c_char,
    accel_key: c_uint,
    accel_mods: gdk::ffi::GdkModifierType,
    hardware_keycode: c_uint,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRendererAccel> = from_glib_borrow(ptr);

    imp.accel_edited(
        wrap.unsafe_cast_ref(),
        &GString::from_glib_borrow(path),
        accel_key,
        from_glib(accel_mods),
        hardware_keycode,
    )
}

unsafe extern "C" fn cell_renderer_accel_cleared<T: CellRendererAccelImpl>(
    ptr: *mut ffi::GtkCellRendererAccel,
    path: *const c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRendererAccel> = from_glib_borrow(ptr);

    imp.accel_cleared(wrap.unsafe_cast_ref(), &GString::from_glib_borrow(path))
}
