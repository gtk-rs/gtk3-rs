// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;
use gtk_sys;

use super::bin::BinImpl;
use crate::Bin;
use crate::Widget;
use crate::Window;

pub trait WindowImpl: WindowImplExt + BinImpl {
    fn set_focus(&self, window: &Self::Type, focus: Option<&Widget>) {
        self.parent_set_focus(window, focus)
    }

    fn activate_focus(&self, window: &Self::Type) {
        self.parent_activate_focus(window)
    }

    fn activate_default(&self, window: &Self::Type) {
        self.parent_activate_default(window)
    }

    fn keys_changed(&self, window: &Self::Type) {
        self.parent_keys_changed(window)
    }

    fn enable_debugging(&self, window: &Self::Type, toggle: bool) -> bool {
        self.parent_enable_debugging(window, toggle)
    }
}

pub trait WindowImplExt: ObjectSubclass {
    fn parent_set_focus(&self, window: &Self::Type, focus: Option<&Widget>);
    fn parent_activate_focus(&self, window: &Self::Type);
    fn parent_activate_default(&self, window: &Self::Type);
    fn parent_keys_changed(&self, window: &Self::Type);
    fn parent_enable_debugging(&self, window: &Self::Type, toggle: bool) -> bool;
}

impl<T: WindowImpl> WindowImplExt for T {
    fn parent_set_focus(&self, window: &Self::Type, focus: Option<&Widget>) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            if let Some(f) = (*parent_class).set_focus {
                f(
                    window.unsafe_cast_ref::<Window>().to_glib_none().0,
                    focus.to_glib_none().0,
                )
            }
        }
    }

    fn parent_activate_focus(&self, window: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            if let Some(f) = (*parent_class).activate_focus {
                f(window.unsafe_cast_ref::<Window>().to_glib_none().0)
            }
        }
    }

    fn parent_activate_default(&self, window: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            if let Some(f) = (*parent_class).activate_default {
                f(window.unsafe_cast_ref::<Window>().to_glib_none().0)
            }
        }
    }

    fn parent_keys_changed(&self, window: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            if let Some(f) = (*parent_class).keys_changed {
                f(window.unsafe_cast_ref::<Window>().to_glib_none().0)
            }
        }
    }

    fn parent_enable_debugging(&self, window: &Self::Type, toggle: bool) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            if let Some(f) = (*parent_class).enable_debugging {
                from_glib(f(
                    window.unsafe_cast_ref::<Window>().to_glib_none().0,
                    toggle.to_glib(),
                ))
            } else {
                false
            }
        }
    }
}

unsafe impl<T: WindowImpl> IsSubclassable<T> for Window {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Bin as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.set_focus = Some(window_set_focus::<T>);
        klass.activate_focus = Some(window_activate_focus::<T>);
        klass.activate_default = Some(window_activate_default::<T>);
        klass.keys_changed = Some(window_keys_changed::<T>);
        klass.enable_debugging = Some(window_enable_debugging::<T>);
    }
}

unsafe extern "C" fn window_set_focus<T: WindowImpl>(
    ptr: *mut gtk_sys::GtkWindow,
    widgetptr: *mut gtk_sys::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Window> = from_glib_borrow(ptr);
    let widget: Borrowed<Option<Widget>> = from_glib_borrow(widgetptr);

    imp.set_focus(wrap.unsafe_cast_ref(), widget.as_ref().as_ref())
}

unsafe extern "C" fn window_activate_focus<T: WindowImpl>(ptr: *mut gtk_sys::GtkWindow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Window> = from_glib_borrow(ptr);

    imp.activate_focus(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn window_activate_default<T: WindowImpl>(ptr: *mut gtk_sys::GtkWindow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Window> = from_glib_borrow(ptr);

    imp.activate_default(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn window_keys_changed<T: WindowImpl>(ptr: *mut gtk_sys::GtkWindow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Window> = from_glib_borrow(ptr);

    imp.keys_changed(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn window_enable_debugging<T: WindowImpl>(
    ptr: *mut gtk_sys::GtkWindow,
    toggleptr: glib_sys::gboolean,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Window> = from_glib_borrow(ptr);
    let toggle: bool = from_glib(toggleptr);

    imp.enable_debugging(wrap.unsafe_cast_ref(), toggle)
        .to_glib()
}
