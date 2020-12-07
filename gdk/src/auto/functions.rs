// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Atom;
use crate::Display;
use crate::Event;
use crate::EventType;
use crate::ModifierType;
use crate::Screen;
use crate::Visual;
use crate::Window;
use crate::WindowState;
use glib::translate::*;
use std::mem;
use std::ptr;

//#[cfg_attr(feature = "v3_16", deprecated)]
//#[doc(alias = "gdk_add_option_entries_libgtk_only")]
//pub fn add_option_entries_libgtk_only(group: /*Ignored*/&glib::OptionGroup) {
//    unsafe { TODO: call ffi:gdk_add_option_entries_libgtk_only() }
//}

#[doc(alias = "gdk_beep")]
pub fn beep() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_beep();
    }
}

#[doc(alias = "gdk_error_trap_pop")]
pub fn error_trap_pop() -> i32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_error_trap_pop() }
}

#[doc(alias = "gdk_error_trap_pop_ignored")]
pub fn error_trap_pop_ignored() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_error_trap_pop_ignored();
    }
}

#[doc(alias = "gdk_error_trap_push")]
pub fn error_trap_push() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_error_trap_push();
    }
}

#[doc(alias = "gdk_events_get_angle")]
pub fn events_get_angle(event1: &mut Event, event2: &mut Event) -> Option<f64> {
    assert_initialized_main_thread!();
    unsafe {
        let mut angle = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::gdk_events_get_angle(
            event1.to_glib_none_mut().0,
            event2.to_glib_none_mut().0,
            angle.as_mut_ptr(),
        ));
        let angle = angle.assume_init();
        if ret {
            Some(angle)
        } else {
            None
        }
    }
}

#[doc(alias = "gdk_events_get_center")]
pub fn events_get_center(event1: &mut Event, event2: &mut Event) -> Option<(f64, f64)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut x = mem::MaybeUninit::uninit();
        let mut y = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::gdk_events_get_center(
            event1.to_glib_none_mut().0,
            event2.to_glib_none_mut().0,
            x.as_mut_ptr(),
            y.as_mut_ptr(),
        ));
        let x = x.assume_init();
        let y = y.assume_init();
        if ret {
            Some((x, y))
        } else {
            None
        }
    }
}

#[doc(alias = "gdk_events_get_distance")]
pub fn events_get_distance(event1: &mut Event, event2: &mut Event) -> Option<f64> {
    assert_initialized_main_thread!();
    unsafe {
        let mut distance = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::gdk_events_get_distance(
            event1.to_glib_none_mut().0,
            event2.to_glib_none_mut().0,
            distance.as_mut_ptr(),
        ));
        let distance = distance.assume_init();
        if ret {
            Some(distance)
        } else {
            None
        }
    }
}

#[doc(alias = "gdk_events_pending")]
pub fn events_pending() -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gdk_events_pending()) }
}

#[doc(alias = "gdk_flush")]
pub fn flush() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_flush();
    }
}

#[doc(alias = "gdk_get_display_arg_name")]
pub fn get_display_arg_name() -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gdk_get_display_arg_name()) }
}

#[doc(alias = "gdk_get_program_class")]
pub fn get_program_class() -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gdk_get_program_class()) }
}

#[doc(alias = "gdk_get_show_events")]
pub fn get_show_events() -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gdk_get_show_events()) }
}

//#[doc(alias = "gdk_init_check")]
//pub fn init_check(argv: /*Unimplemented*/Vec<glib::GString>) -> bool {
//    unsafe { TODO: call ffi:gdk_init_check() }
//}

#[doc(alias = "gdk_keyval_convert_case")]
pub fn keyval_convert_case(symbol: u32) -> (u32, u32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut lower = mem::MaybeUninit::uninit();
        let mut upper = mem::MaybeUninit::uninit();
        ffi::gdk_keyval_convert_case(symbol, lower.as_mut_ptr(), upper.as_mut_ptr());
        let lower = lower.assume_init();
        let upper = upper.assume_init();
        (lower, upper)
    }
}

#[doc(alias = "gdk_keyval_from_name")]
pub fn keyval_from_name(keyval_name: &str) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_keyval_from_name(keyval_name.to_glib_none().0) }
}

#[doc(alias = "gdk_keyval_is_lower")]
pub fn keyval_is_lower(keyval: u32) -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gdk_keyval_is_lower(keyval)) }
}

#[doc(alias = "gdk_keyval_is_upper")]
pub fn keyval_is_upper(keyval: u32) -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gdk_keyval_is_upper(keyval)) }
}

#[doc(alias = "gdk_keyval_to_lower")]
pub fn keyval_to_lower(keyval: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_keyval_to_lower(keyval) }
}

#[doc(alias = "gdk_keyval_to_upper")]
pub fn keyval_to_upper(keyval: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_keyval_to_upper(keyval) }
}

#[cfg_attr(feature = "v3_22", deprecated)]
#[doc(alias = "gdk_list_visuals")]
pub fn list_visuals() -> Vec<Visual> {
    assert_initialized_main_thread!();
    unsafe { FromGlibPtrContainer::from_glib_container(ffi::gdk_list_visuals()) }
}

#[doc(alias = "gdk_notify_startup_complete")]
pub fn notify_startup_complete() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_notify_startup_complete();
    }
}

#[doc(alias = "gdk_notify_startup_complete_with_id")]
pub fn notify_startup_complete_with_id(startup_id: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_notify_startup_complete_with_id(startup_id.to_glib_none().0);
    }
}

#[doc(alias = "gdk_pango_context_get")]
pub fn pango_context_get() -> Option<pango::Context> {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(ffi::gdk_pango_context_get()) }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
#[doc(alias = "gdk_pango_context_get_for_display")]
pub fn pango_context_get_for_display(display: &Display) -> Option<pango::Context> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gdk_pango_context_get_for_display(
            display.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gdk_pango_context_get_for_screen")]
pub fn pango_context_get_for_screen(screen: &Screen) -> Option<pango::Context> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gdk_pango_context_get_for_screen(
            screen.to_glib_none().0,
        ))
    }
}

//#[doc(alias = "gdk_pango_layout_line_get_clip_region")]
//pub fn pango_layout_line_get_clip_region(line: &pango::LayoutLine, x_origin: i32, y_origin: i32, index_ranges: &[i32], n_ranges: i32) -> Option<cairo::Region> {
//    unsafe { TODO: call ffi:gdk_pango_layout_line_get_clip_region() }
//}

//#[doc(alias = "gdk_parse_args")]
//pub fn parse_args(argv: /*Unimplemented*/Vec<glib::GString>) {
//    unsafe { TODO: call ffi:gdk_parse_args() }
//}

#[doc(alias = "gdk_pixbuf_get_from_surface")]
pub fn pixbuf_get_from_surface(
    surface: &cairo::Surface,
    src_x: i32,
    src_y: i32,
    width: i32,
    height: i32,
) -> Option<gdk_pixbuf::Pixbuf> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gdk_pixbuf_get_from_surface(
            mut_override(surface.to_glib_none().0),
            src_x,
            src_y,
            width,
            height,
        ))
    }
}

#[cfg_attr(feature = "v3_16", deprecated)]
#[doc(alias = "gdk_pre_parse_libgtk_only")]
pub fn pre_parse_libgtk_only() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_pre_parse_libgtk_only();
    }
}

#[doc(alias = "gdk_property_delete")]
pub fn property_delete(window: &Window, property: &Atom) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_property_delete(window.to_glib_none().0, property.to_glib_none().0);
    }
}

#[doc(alias = "gdk_property_get")]
pub fn property_get(
    window: &Window,
    property: &Atom,
    type_: &Atom,
    offset: libc::c_ulong,
    length: libc::c_ulong,
    pdelete: i32,
) -> Option<(Atom, i32, Vec<u8>)> {
    skip_assert_initialized!();
    unsafe {
        let mut actual_property_type = Atom::uninitialized();
        let mut actual_format = mem::MaybeUninit::uninit();
        let mut actual_length = mem::MaybeUninit::uninit();
        let mut data = ptr::null_mut();
        let ret = from_glib(ffi::gdk_property_get(
            window.to_glib_none().0,
            property.to_glib_none().0,
            type_.to_glib_none().0,
            offset,
            length,
            pdelete,
            actual_property_type.to_glib_none_mut().0,
            actual_format.as_mut_ptr(),
            actual_length.as_mut_ptr(),
            &mut data,
        ));
        let actual_format = actual_format.assume_init();
        if ret {
            Some((
                actual_property_type,
                actual_format,
                FromGlibContainer::from_glib_full_num(data, actual_length.assume_init() as usize),
            ))
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "v3_22", deprecated)]
#[doc(alias = "gdk_query_depths")]
pub fn query_depths() -> Vec<i32> {
    assert_initialized_main_thread!();
    unsafe {
        let mut depths = ptr::null_mut();
        let mut count = mem::MaybeUninit::uninit();
        ffi::gdk_query_depths(&mut depths, count.as_mut_ptr());
        FromGlibContainer::from_glib_none_num(depths, count.assume_init() as usize)
    }
}

//#[cfg_attr(feature = "v3_22", deprecated)]
//#[doc(alias = "gdk_query_visual_types")]
//pub fn query_visual_types(visual_types: /*Unimplemented*/CArray TypeId { ns_id: 1, id: 99 }) -> i32 {
//    unsafe { TODO: call ffi:gdk_query_visual_types() }
//}

#[doc(alias = "gdk_selection_convert")]
pub fn selection_convert(requestor: &Window, selection: &Atom, target: &Atom, time_: u32) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_selection_convert(
            requestor.to_glib_none().0,
            selection.to_glib_none().0,
            target.to_glib_none().0,
            time_,
        );
    }
}

#[doc(alias = "gdk_selection_owner_get")]
pub fn selection_owner_get(selection: &Atom) -> Option<Window> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gdk_selection_owner_get(selection.to_glib_none().0)) }
}

#[doc(alias = "gdk_selection_owner_get_for_display")]
pub fn selection_owner_get_for_display(display: &Display, selection: &Atom) -> Option<Window> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gdk_selection_owner_get_for_display(
            display.to_glib_none().0,
            selection.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gdk_selection_owner_set")]
pub fn selection_owner_set(
    owner: Option<&Window>,
    selection: &Atom,
    time_: u32,
    send_event: bool,
) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gdk_selection_owner_set(
            owner.to_glib_none().0,
            selection.to_glib_none().0,
            time_,
            send_event.to_glib(),
        ))
    }
}

#[doc(alias = "gdk_selection_owner_set_for_display")]
pub fn selection_owner_set_for_display(
    display: &Display,
    owner: Option<&Window>,
    selection: &Atom,
    time_: u32,
    send_event: bool,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gdk_selection_owner_set_for_display(
            display.to_glib_none().0,
            owner.to_glib_none().0,
            selection.to_glib_none().0,
            time_,
            send_event.to_glib(),
        ))
    }
}

#[doc(alias = "gdk_selection_send_notify")]
pub fn selection_send_notify(
    requestor: &Window,
    selection: &Atom,
    target: &Atom,
    property: &Atom,
    time_: u32,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_selection_send_notify(
            requestor.to_glib_none().0,
            selection.to_glib_none().0,
            target.to_glib_none().0,
            property.to_glib_none().0,
            time_,
        );
    }
}

#[doc(alias = "gdk_selection_send_notify_for_display")]
pub fn selection_send_notify_for_display(
    display: &Display,
    requestor: &Window,
    selection: &Atom,
    target: &Atom,
    property: &Atom,
    time_: u32,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_selection_send_notify_for_display(
            display.to_glib_none().0,
            requestor.to_glib_none().0,
            selection.to_glib_none().0,
            target.to_glib_none().0,
            property.to_glib_none().0,
            time_,
        );
    }
}

#[doc(alias = "gdk_set_allowed_backends")]
pub fn set_allowed_backends(backends: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_set_allowed_backends(backends.to_glib_none().0);
    }
}

#[doc(alias = "gdk_set_double_click_time")]
pub fn set_double_click_time(msec: u32) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_set_double_click_time(msec);
    }
}

#[doc(alias = "gdk_set_program_class")]
pub fn set_program_class(program_class: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_set_program_class(program_class.to_glib_none().0);
    }
}

#[doc(alias = "gdk_set_show_events")]
pub fn set_show_events(show_events: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_set_show_events(show_events.to_glib());
    }
}

#[doc(alias = "gdk_synthesize_window_state")]
pub fn synthesize_window_state(window: &Window, unset_flags: WindowState, set_flags: WindowState) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_synthesize_window_state(
            window.to_glib_none().0,
            unset_flags.to_glib(),
            set_flags.to_glib(),
        );
    }
}

#[doc(alias = "gdk_test_render_sync")]
pub fn test_render_sync(window: &Window) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_test_render_sync(window.to_glib_none().0);
    }
}

#[doc(alias = "gdk_test_simulate_button")]
pub fn test_simulate_button(
    window: &Window,
    x: i32,
    y: i32,
    button: u32,
    modifiers: ModifierType,
    button_pressrelease: EventType,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gdk_test_simulate_button(
            window.to_glib_none().0,
            x,
            y,
            button,
            modifiers.to_glib(),
            button_pressrelease.to_glib(),
        ))
    }
}

#[doc(alias = "gdk_test_simulate_key")]
pub fn test_simulate_key(
    window: &Window,
    x: i32,
    y: i32,
    keyval: u32,
    modifiers: ModifierType,
    key_pressrelease: EventType,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gdk_test_simulate_key(
            window.to_glib_none().0,
            x,
            y,
            keyval,
            modifiers.to_glib(),
            key_pressrelease.to_glib(),
        ))
    }
}

#[doc(alias = "gdk_text_property_to_utf8_list_for_display")]
pub fn text_property_to_utf8_list_for_display(
    display: &Display,
    encoding: &Atom,
    format: i32,
    text: &[u8],
) -> (i32, Vec<glib::GString>) {
    skip_assert_initialized!();
    let length = text.len() as i32;
    unsafe {
        let mut list = ptr::null_mut();
        let ret = ffi::gdk_text_property_to_utf8_list_for_display(
            display.to_glib_none().0,
            encoding.to_glib_none().0,
            format,
            text.to_glib_none().0,
            length,
            &mut list,
        );
        (ret, FromGlibPtrContainer::from_glib_full(list))
    }
}

#[doc(alias = "gdk_unicode_to_keyval")]
pub fn unicode_to_keyval(wc: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_unicode_to_keyval(wc) }
}

#[doc(alias = "gdk_utf8_to_string_target")]
pub fn utf8_to_string_target(str: &str) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(ffi::gdk_utf8_to_string_target(str.to_glib_none().0)) }
}
