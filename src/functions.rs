// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cairo;
use gdk_sys;
use glib;
use glib::translate::*;
use pango;

#[repr(packed)]
pub struct GRange(pub i32, pub i32);

pub fn pango_layout_line_get_clip_region(
    line: &pango::LayoutLine,
    x_origin: i32,
    y_origin: i32,
    index_ranges: &[GRange],
) -> Option<cairo::Region> {
    assert_initialized_main_thread!();

    let ptr: *const i32 = index_ranges.as_ptr() as _;
    unsafe {
        from_glib_full(gdk_sys::gdk_pango_layout_line_get_clip_region(
            line.to_glib_none().0,
            x_origin,
            y_origin,
            mut_override(ptr),
            index_ranges.len() as i32,
        ))
    }
}

pub fn pango_layout_get_clip_region(
    layout: &pango::Layout,
    x_origin: i32,
    y_origin: i32,
    index_ranges: &[GRange],
) -> Option<cairo::Region> {
    assert_initialized_main_thread!();

    let ptr: *const i32 = index_ranges.as_ptr() as _;
    unsafe {
        from_glib_full(gdk_sys::gdk_pango_layout_get_clip_region(
            layout.to_glib_none().0,
            x_origin,
            y_origin,
            ptr,
            index_ranges.len() as i32,
        ))
    }
}

pub fn setting_get(name: &str) -> Option<glib::Value> {
    assert_initialized_main_thread!();
    unsafe {
        let mut value = glib::Value::uninitialized();
        let done: bool = from_glib(gdk_sys::gdk_setting_get(
            name.to_glib_none().0,
            value.to_glib_none_mut().0,
        ));
        if done == true {
            Some(value)
        } else {
            None
        }
    }
}

pub fn property_change(
    window: &super::Window,
    property: &super::Atom,
    type_: &super::Atom,
    format: i32,
    mode: super::PropMode,
    data: super::ChangeData,
) {
    skip_assert_initialized!();
    let nelements = data.len();
    unsafe {
        gdk_sys::gdk_property_change(
            window.to_glib_none().0,
            property.to_glib_none().0,
            type_.to_glib_none().0,
            format,
            mode.to_glib(),
            data.to_glib(),
            nelements as i32,
        );
    }
}
