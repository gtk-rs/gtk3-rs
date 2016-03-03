// This file was generated by gir (7dd2bcd) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use PositionType;
use Rectangle;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Popover(Object<ffi::GtkPopover>): Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_popover_get_type(),
    }
}

impl Popover {
    #[cfg(feature = "3.12")]
    pub fn new<T: IsA<Widget>>(relative_to: Option<&T>) -> Popover {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_new(relative_to.to_glib_none().0)).downcast_unchecked()
        }
    }

    //#[cfg(feature = "3.12")]
    //pub fn new_from_model<T: IsA<Widget>, U: IsA</*Ignored*/gio::MenuModel>>(relative_to: Option<&T>, model: &U) -> Popover {
    //    unsafe { TODO: call ffi::gtk_popover_new_from_model() }
    //}
}

pub trait PopoverExt {
    //#[cfg(feature = "3.12")]
    //fn bind_model<T: IsA</*Ignored*/gio::MenuModel>>(&self, model: Option<&T>, action_namespace: Option<&str>);

    #[cfg(feature = "3.12")]
    fn get_modal(&self) -> bool;

    fn get_pointing_to(&self) -> Option<Rectangle>;

    fn get_position(&self) -> PositionType;

    #[cfg(feature = "3.12")]
    fn get_relative_to(&self) -> Option<Widget>;

    #[cfg(feature = "3.16")]
    fn get_transitions_enabled(&self) -> bool;

    #[cfg(feature = "3.12")]
    fn set_modal(&self, modal: bool);

    #[cfg(feature = "3.12")]
    fn set_pointing_to(&self, rect: &Rectangle);

    #[cfg(feature = "3.12")]
    fn set_position(&self, position: PositionType);

    #[cfg(feature = "3.12")]
    fn set_relative_to<T: IsA<Widget>>(&self, relative_to: Option<&T>);

    #[cfg(feature = "3.16")]
    fn set_transitions_enabled(&self, transitions_enabled: bool);
}

impl<O: IsA<Popover>> PopoverExt for O {
    //#[cfg(feature = "3.12")]
    //fn bind_model<T: IsA</*Ignored*/gio::MenuModel>>(&self, model: Option<&T>, action_namespace: Option<&str>) {
    //    unsafe { TODO: call ffi::gtk_popover_bind_model() }
    //}

    #[cfg(feature = "3.12")]
    fn get_modal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_modal(self.to_glib_none().0))
        }
    }

    fn get_pointing_to(&self) -> Option<Rectangle> {
        unsafe {
            let mut rect = Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_popover_get_pointing_to(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    fn get_position(&self) -> PositionType {
        unsafe {
            ffi::gtk_popover_get_position(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "3.12")]
    fn get_relative_to(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_popover_get_relative_to(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "3.16")]
    fn get_transitions_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_transitions_enabled(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "3.12")]
    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_popover_set_modal(self.to_glib_none().0, modal.to_glib());
        }
    }

    #[cfg(feature = "3.12")]
    fn set_pointing_to(&self, rect: &Rectangle) {
        unsafe {
            ffi::gtk_popover_set_pointing_to(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }

    #[cfg(feature = "3.12")]
    fn set_position(&self, position: PositionType) {
        unsafe {
            ffi::gtk_popover_set_position(self.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "3.12")]
    fn set_relative_to<T: IsA<Widget>>(&self, relative_to: Option<&T>) {
        unsafe {
            ffi::gtk_popover_set_relative_to(self.to_glib_none().0, relative_to.to_glib_none().0);
        }
    }

    #[cfg(feature = "3.16")]
    fn set_transitions_enabled(&self, transitions_enabled: bool) {
        unsafe {
            ffi::gtk_popover_set_transitions_enabled(self.to_glib_none().0, transitions_enabled.to_glib());
        }
    }
}
