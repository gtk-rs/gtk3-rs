// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::StateFlags;
use crate::Widget;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct WidgetPath(Shared<ffi::GtkWidgetPath>);

    match fn {
        ref => |ptr| ffi::gtk_widget_path_ref(ptr),
        unref => |ptr| ffi::gtk_widget_path_unref(ptr),
        get_type => || ffi::gtk_widget_path_get_type(),
    }
}

impl WidgetPath {
    pub fn new() -> WidgetPath {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_widget_path_new()) }
    }

    pub fn append_for_widget<P: IsA<Widget>>(&self, widget: &P) -> i32 {
        unsafe {
            ffi::gtk_widget_path_append_for_widget(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            )
        }
    }

    pub fn append_type(&self, type_: glib::types::Type) -> i32 {
        unsafe { ffi::gtk_widget_path_append_type(self.to_glib_none().0, type_.to_glib()) }
    }

    pub fn append_with_siblings(&self, siblings: &WidgetPath, sibling_index: u32) -> i32 {
        unsafe {
            ffi::gtk_widget_path_append_with_siblings(
                self.to_glib_none().0,
                siblings.to_glib_none().0,
                sibling_index,
            )
        }
    }

    pub fn copy(&self) -> Option<WidgetPath> {
        unsafe { from_glib_full(ffi::gtk_widget_path_copy(self.to_glib_none().0)) }
    }

    pub fn get_object_type(&self) -> glib::types::Type {
        unsafe { from_glib(ffi::gtk_widget_path_get_object_type(self.to_glib_none().0)) }
    }

    pub fn has_parent(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_has_parent(
                self.to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    pub fn is_type(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_is_type(
                self.to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    pub fn iter_add_class(&self, pos: i32, name: &str) {
        unsafe {
            ffi::gtk_widget_path_iter_add_class(self.to_glib_none().0, pos, name.to_glib_none().0);
        }
    }

    pub fn iter_clear_classes(&self, pos: i32) {
        unsafe {
            ffi::gtk_widget_path_iter_clear_classes(self.to_glib_none().0, pos);
        }
    }

    pub fn iter_get_name(&self, pos: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_widget_path_iter_get_name(
                self.to_glib_none().0,
                pos,
            ))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn iter_get_object_name(&self, pos: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_widget_path_iter_get_object_name(
                self.to_glib_none().0,
                pos,
            ))
        }
    }

    pub fn iter_get_object_type(&self, pos: i32) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_get_object_type(
                self.to_glib_none().0,
                pos,
            ))
        }
    }

    pub fn iter_get_sibling_index(&self, pos: i32) -> u32 {
        unsafe { ffi::gtk_widget_path_iter_get_sibling_index(self.to_glib_none().0, pos) }
    }

    pub fn iter_get_siblings(&self, pos: i32) -> Option<WidgetPath> {
        unsafe {
            from_glib_none(ffi::gtk_widget_path_iter_get_siblings(
                self.to_glib_none().0,
                pos,
            ))
        }
    }

    pub fn iter_get_state(&self, pos: i32) -> StateFlags {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_get_state(
                self.to_glib_none().0,
                pos,
            ))
        }
    }

    pub fn iter_has_class(&self, pos: i32, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_has_class(
                self.to_glib_none().0,
                pos,
                name.to_glib_none().0,
            ))
        }
    }

    pub fn iter_has_name(&self, pos: i32, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_has_name(
                self.to_glib_none().0,
                pos,
                name.to_glib_none().0,
            ))
        }
    }

    pub fn iter_has_qclass(&self, pos: i32, qname: glib::Quark) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_has_qclass(
                self.to_glib_none().0,
                pos,
                qname.to_glib(),
            ))
        }
    }

    pub fn iter_has_qname(&self, pos: i32, qname: glib::Quark) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_has_qname(
                self.to_glib_none().0,
                pos,
                qname.to_glib(),
            ))
        }
    }

    pub fn iter_list_classes(&self, pos: i32) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_widget_path_iter_list_classes(
                self.to_glib_none().0,
                pos,
            ))
        }
    }

    pub fn iter_remove_class(&self, pos: i32, name: &str) {
        unsafe {
            ffi::gtk_widget_path_iter_remove_class(
                self.to_glib_none().0,
                pos,
                name.to_glib_none().0,
            );
        }
    }

    pub fn iter_set_name(&self, pos: i32, name: &str) {
        unsafe {
            ffi::gtk_widget_path_iter_set_name(self.to_glib_none().0, pos, name.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn iter_set_object_name(&self, pos: i32, name: Option<&str>) {
        unsafe {
            ffi::gtk_widget_path_iter_set_object_name(
                self.to_glib_none().0,
                pos,
                name.to_glib_none().0,
            );
        }
    }

    pub fn iter_set_object_type(&self, pos: i32, type_: glib::types::Type) {
        unsafe {
            ffi::gtk_widget_path_iter_set_object_type(self.to_glib_none().0, pos, type_.to_glib());
        }
    }

    pub fn iter_set_state(&self, pos: i32, state: StateFlags) {
        unsafe {
            ffi::gtk_widget_path_iter_set_state(self.to_glib_none().0, pos, state.to_glib());
        }
    }

    pub fn length(&self) -> i32 {
        unsafe { ffi::gtk_widget_path_length(self.to_glib_none().0) }
    }

    pub fn prepend_type(&self, type_: glib::types::Type) {
        unsafe {
            ffi::gtk_widget_path_prepend_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    fn to_string(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gtk_widget_path_to_string(self.to_glib_none().0)) }
    }
}

impl Default for WidgetPath {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WidgetPath {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}
