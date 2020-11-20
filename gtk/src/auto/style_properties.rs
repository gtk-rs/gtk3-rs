// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::StateFlags;
use crate::StyleProvider;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct StyleProperties(Object<ffi::GtkStyleProperties, ffi::GtkStylePropertiesClass>) @implements StyleProvider;

    match fn {
        get_type => || ffi::gtk_style_properties_get_type(),
    }
}

impl StyleProperties {
    #[cfg_attr(feature = "v3_16", deprecated)]
    pub fn new() -> StyleProperties {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_style_properties_new()) }
    }
}

impl Default for StyleProperties {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STYLE_PROPERTIES: Option<&StyleProperties> = None;

pub trait StylePropertiesExt: 'static {
    #[cfg_attr(feature = "v3_16", deprecated)]
    fn clear(&self);

    //#[cfg_attr(feature = "v3_16", deprecated)]
    //fn get(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn get_property(&self, property: &str, state: StateFlags) -> Option<glib::Value>;

    //#[cfg_attr(feature = "v3_16", deprecated)]
    //fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn merge<P: IsA<StyleProperties>>(&self, props_to_merge: &P, replace: bool);

    //#[cfg_attr(feature = "v3_16", deprecated)]
    //fn set(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn set_property(&self, property: &str, state: StateFlags, value: &glib::Value);

    //#[cfg_attr(feature = "v3_16", deprecated)]
    //fn set_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn unset_property(&self, property: &str, state: StateFlags);
}

impl<O: IsA<StyleProperties>> StylePropertiesExt for O {
    fn clear(&self) {
        unsafe {
            ffi::gtk_style_properties_clear(self.as_ref().to_glib_none().0);
        }
    }

    //fn get(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_style_properties_get() }
    //}

    fn get_property(&self, property: &str, state: StateFlags) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret = from_glib(ffi::gtk_style_properties_get_property(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                state.to_glib(),
                value.to_glib_none_mut().0,
            ));
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    //fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gtk_style_properties_get_valist() }
    //}

    fn merge<P: IsA<StyleProperties>>(&self, props_to_merge: &P, replace: bool) {
        unsafe {
            ffi::gtk_style_properties_merge(
                self.as_ref().to_glib_none().0,
                props_to_merge.as_ref().to_glib_none().0,
                replace.to_glib(),
            );
        }
    }

    //fn set(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_style_properties_set() }
    //}

    fn set_property(&self, property: &str, state: StateFlags, value: &glib::Value) {
        unsafe {
            ffi::gtk_style_properties_set_property(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                state.to_glib(),
                value.to_glib_none().0,
            );
        }
    }

    //fn set_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gtk_style_properties_set_valist() }
    //}

    fn unset_property(&self, property: &str, state: StateFlags) {
        unsafe {
            ffi::gtk_style_properties_unset_property(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                state.to_glib(),
            );
        }
    }
}

impl fmt::Display for StyleProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StyleProperties")
    }
}
