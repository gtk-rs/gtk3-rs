// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::AccelGroup;
use crate::Orientation;
use crate::PageSetup;
use crate::PositionType;
use crate::PrintSettings;
use crate::SelectionData;
use crate::SpinButton;
use crate::StyleContext;
use crate::TextBuffer;
use crate::TextDirection;
use crate::TreeModel;
use crate::TreePath;
use crate::Widget;
use crate::Window;
use cairo;
use gdk;
use gdk_pixbuf;
use glib;
use glib::object::IsA;
use glib::translate::*;
use pango;
use std::boxed::Box as Box_;
use std::mem;
use std::ptr;

pub fn accel_groups_activate<P: IsA<glib::Object>>(
    object: &P,
    accel_key: u32,
    accel_mods: gdk::ModifierType,
) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_accel_groups_activate(
            object.as_ref().to_glib_none().0,
            accel_key,
            accel_mods.to_glib(),
        ))
    }
}

pub fn accel_groups_from_object<P: IsA<glib::Object>>(object: &P) -> Vec<AccelGroup> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_none(ffi::gtk_accel_groups_from_object(
            object.as_ref().to_glib_none().0,
        ))
    }
}

pub fn accelerator_get_default_mod_mask() -> gdk::ModifierType {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_accelerator_get_default_mod_mask()) }
}

pub fn accelerator_get_label(
    accelerator_key: u32,
    accelerator_mods: gdk::ModifierType,
) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_get_label(
            accelerator_key,
            accelerator_mods.to_glib(),
        ))
    }
}

pub fn accelerator_get_label_with_keycode(
    display: Option<&gdk::Display>,
    accelerator_key: u32,
    keycode: u32,
    accelerator_mods: gdk::ModifierType,
) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_get_label_with_keycode(
            display.to_glib_none().0,
            accelerator_key,
            keycode,
            accelerator_mods.to_glib(),
        ))
    }
}

pub fn accelerator_name(
    accelerator_key: u32,
    accelerator_mods: gdk::ModifierType,
) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_name(
            accelerator_key,
            accelerator_mods.to_glib(),
        ))
    }
}

pub fn accelerator_name_with_keycode(
    display: Option<&gdk::Display>,
    accelerator_key: u32,
    keycode: u32,
    accelerator_mods: gdk::ModifierType,
) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_name_with_keycode(
            display.to_glib_none().0,
            accelerator_key,
            keycode,
            accelerator_mods.to_glib(),
        ))
    }
}

pub fn accelerator_parse(accelerator: &str) -> (u32, gdk::ModifierType) {
    assert_initialized_main_thread!();
    unsafe {
        let mut accelerator_key = mem::MaybeUninit::uninit();
        let mut accelerator_mods = mem::MaybeUninit::uninit();
        ffi::gtk_accelerator_parse(
            accelerator.to_glib_none().0,
            accelerator_key.as_mut_ptr(),
            accelerator_mods.as_mut_ptr(),
        );
        let accelerator_key = accelerator_key.assume_init();
        let accelerator_mods = accelerator_mods.assume_init();
        (accelerator_key, from_glib(accelerator_mods))
    }
}

//pub fn accelerator_parse_with_keycode(accelerator: &str, accelerator_codes: Vec<u32>) -> (u32, gdk::ModifierType) {
//    unsafe { TODO: call ffi:gtk_accelerator_parse_with_keycode() }
//}

pub fn accelerator_set_default_mod_mask(default_mod_mask: gdk::ModifierType) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_accelerator_set_default_mod_mask(default_mod_mask.to_glib());
    }
}

pub fn accelerator_valid(keyval: u32, modifiers: gdk::ModifierType) -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_accelerator_valid(keyval, modifiers.to_glib())) }
}

pub fn bindings_activate<P: IsA<glib::Object>>(
    object: &P,
    keyval: u32,
    modifiers: gdk::ModifierType,
) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_bindings_activate(
            object.as_ref().to_glib_none().0,
            keyval,
            modifiers.to_glib(),
        ))
    }
}

pub fn bindings_activate_event<P: IsA<glib::Object>>(
    object: &P,
    event: &mut gdk::EventKey,
) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_bindings_activate_event(
            object.as_ref().to_glib_none().0,
            event.to_glib_none_mut().0,
        ))
    }
}

pub fn cairo_should_draw_window<P: IsA<gdk::Window>>(cr: &cairo::Context, window: &P) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_cairo_should_draw_window(
            mut_override(cr.to_glib_none().0),
            window.as_ref().to_glib_none().0,
        ))
    }
}

pub fn cairo_transform_to_window<P: IsA<Widget>, Q: IsA<gdk::Window>>(
    cr: &cairo::Context,
    widget: &P,
    window: &Q,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_cairo_transform_to_window(
            mut_override(cr.to_glib_none().0),
            widget.as_ref().to_glib_none().0,
            window.as_ref().to_glib_none().0,
        );
    }
}

pub fn device_grab_add<P: IsA<Widget>>(widget: &P, device: &gdk::Device, block_others: bool) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_device_grab_add(
            widget.as_ref().to_glib_none().0,
            device.to_glib_none().0,
            block_others.to_glib(),
        );
    }
}

pub fn device_grab_remove<P: IsA<Widget>>(widget: &P, device: &gdk::Device) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_device_grab_remove(widget.as_ref().to_glib_none().0, device.to_glib_none().0);
    }
}

pub fn disable_setlocale() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_disable_setlocale();
    }
}

//pub fn distribute_natural_allocation(extra_space: i32, n_requested_sizes: u32, sizes: /*Ignored*/&mut RequestedSize) -> i32 {
//    unsafe { TODO: call ffi:gtk_distribute_natural_allocation() }
//}

pub fn events_pending() -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_events_pending()) }
}

pub fn false_() -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_false()) }
}

pub fn get_current_event() -> Option<gdk::Event> {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(ffi::gtk_get_current_event()) }
}

pub fn get_current_event_device() -> Option<gdk::Device> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gtk_get_current_event_device()) }
}

pub fn get_current_event_state() -> Option<gdk::ModifierType> {
    assert_initialized_main_thread!();
    unsafe {
        let mut state = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::gtk_get_current_event_state(state.as_mut_ptr()));
        let state = state.assume_init();
        if ret {
            Some(from_glib(state))
        } else {
            None
        }
    }
}

pub fn get_current_event_time() -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gtk_get_current_event_time() }
}

pub fn get_debug_flags() -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gtk_get_debug_flags() }
}

pub fn get_default_language() -> Option<pango::Language> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gtk_get_default_language()) }
}

pub fn get_event_widget(event: &mut gdk::Event) -> Option<Widget> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gtk_get_event_widget(event.to_glib_none_mut().0)) }
}

pub fn get_locale_direction() -> TextDirection {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_get_locale_direction()) }
}

//pub fn get_option_group(open_default_display: bool) -> /*Ignored*/Option<glib::OptionGroup> {
//    unsafe { TODO: call ffi:gtk_get_option_group() }
//}

pub fn grab_get_current() -> Option<Widget> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gtk_grab_get_current()) }
}

//pub fn init_check(argv: /*Unimplemented*/Vec<glib::GString>) -> bool {
//    unsafe { TODO: call ffi:gtk_init_check() }
//}

//pub fn init_with_args(argv: /*Unimplemented*/Vec<glib::GString>, parameter_string: Option<&str>, entries: /*Ignored*/&[&glib::OptionEntry], translation_domain: Option<&str>) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:gtk_init_with_args() }
//}

pub fn main() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_main();
    }
}

pub fn main_do_event(event: &mut gdk::Event) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_main_do_event(event.to_glib_none_mut().0);
    }
}

pub fn main_iteration() -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_main_iteration()) }
}

pub fn main_iteration_do(blocking: bool) -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_main_iteration_do(blocking.to_glib())) }
}

pub fn main_level() -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gtk_main_level() }
}

//pub fn parse_args(argv: /*Unimplemented*/Vec<glib::GString>) -> bool {
//    unsafe { TODO: call ffi:gtk_parse_args() }
//}

pub fn print_run_page_setup_dialog<P: IsA<Window>>(
    parent: Option<&P>,
    page_setup: Option<&PageSetup>,
    settings: &PrintSettings,
) -> Option<PageSetup> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gtk_print_run_page_setup_dialog(
            parent.map(|p| p.as_ref()).to_glib_none().0,
            page_setup.to_glib_none().0,
            settings.to_glib_none().0,
        ))
    }
}

pub fn print_run_page_setup_dialog_async<
    P: IsA<Window>,
    Q: FnOnce(&PageSetup) + Send + Sync + 'static,
>(
    parent: Option<&P>,
    page_setup: Option<&PageSetup>,
    settings: &PrintSettings,
    done_cb: Q,
) {
    skip_assert_initialized!();
    let done_cb_data: Box_<Q> = Box_::new(done_cb);
    unsafe extern "C" fn done_cb_func<
        P: IsA<Window>,
        Q: FnOnce(&PageSetup) + Send + Sync + 'static,
    >(
        page_setup: *mut ffi::GtkPageSetup,
        data: glib::ffi::gpointer,
    ) {
        let page_setup = from_glib_borrow(page_setup);
        let callback: Box_<Q> = Box_::from_raw(data as *mut _);
        (*callback)(&page_setup);
    }
    let done_cb = Some(done_cb_func::<P, Q> as _);
    let super_callback0: Box_<Q> = done_cb_data;
    unsafe {
        ffi::gtk_print_run_page_setup_dialog_async(
            parent.map(|p| p.as_ref()).to_glib_none().0,
            page_setup.to_glib_none().0,
            settings.to_glib_none().0,
            done_cb,
            Box_::into_raw(super_callback0) as *mut _,
        );
    }
}

pub fn propagate_event<P: IsA<Widget>>(widget: &P, event: &mut gdk::Event) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_propagate_event(widget.as_ref().to_glib_none().0, event.to_glib_none_mut().0);
    }
}

pub fn render_activity<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_activity(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

pub fn render_arrow<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    angle: f64,
    x: f64,
    y: f64,
    size: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_arrow(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            angle,
            x,
            y,
            size,
        );
    }
}

pub fn render_background<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_background(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
pub fn render_background_get_clip<P: IsA<StyleContext>>(
    context: &P,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> gdk::Rectangle {
    skip_assert_initialized!();
    unsafe {
        let mut out_clip = gdk::Rectangle::uninitialized();
        ffi::gtk_render_background_get_clip(
            context.as_ref().to_glib_none().0,
            x,
            y,
            width,
            height,
            out_clip.to_glib_none_mut().0,
        );
        out_clip
    }
}

pub fn render_check<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_check(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

pub fn render_expander<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_expander(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

pub fn render_extension<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    gap_side: PositionType,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_extension(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
            gap_side.to_glib(),
        );
    }
}

pub fn render_focus<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_focus(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

pub fn render_frame<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_frame(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[cfg_attr(feature = "v3_24", deprecated)]
pub fn render_frame_gap<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    gap_side: PositionType,
    xy0_gap: f64,
    xy1_gap: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_frame_gap(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
            gap_side.to_glib(),
            xy0_gap,
            xy1_gap,
        );
    }
}

pub fn render_handle<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_handle(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

pub fn render_icon<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    pixbuf: &gdk_pixbuf::Pixbuf,
    x: f64,
    y: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_icon(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            pixbuf.to_glib_none().0,
            x,
            y,
        );
    }
}

pub fn render_icon_surface<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    surface: &cairo::Surface,
    x: f64,
    y: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_icon_surface(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            mut_override(surface.to_glib_none().0),
            x,
            y,
        );
    }
}

pub fn render_insertion_cursor<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    layout: &pango::Layout,
    index: i32,
    direction: pango::Direction,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_insertion_cursor(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            layout.to_glib_none().0,
            index,
            direction.to_glib(),
        );
    }
}

pub fn render_layout<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    layout: &pango::Layout,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_layout(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            layout.to_glib_none().0,
        );
    }
}

pub fn render_line<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_line(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x0,
            y0,
            x1,
            y1,
        );
    }
}

pub fn render_option<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_option(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

pub fn render_slider<P: IsA<StyleContext>>(
    context: &P,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    orientation: Orientation,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_slider(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
            orientation.to_glib(),
        );
    }
}

pub fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    assert_initialized_main_thread!();
    unsafe {
        let mut h = mem::MaybeUninit::uninit();
        let mut s = mem::MaybeUninit::uninit();
        let mut v = mem::MaybeUninit::uninit();
        ffi::gtk_rgb_to_hsv(r, g, b, h.as_mut_ptr(), s.as_mut_ptr(), v.as_mut_ptr());
        let h = h.assume_init();
        let s = s.assume_init();
        let v = v.assume_init();
        (h, s, v)
    }
}

pub fn selection_add_target<P: IsA<Widget>>(
    widget: &P,
    selection: &gdk::Atom,
    target: &gdk::Atom,
    info: u32,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_selection_add_target(
            widget.as_ref().to_glib_none().0,
            selection.to_glib_none().0,
            target.to_glib_none().0,
            info,
        );
    }
}

pub fn selection_clear_targets<P: IsA<Widget>>(widget: &P, selection: &gdk::Atom) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_selection_clear_targets(
            widget.as_ref().to_glib_none().0,
            selection.to_glib_none().0,
        );
    }
}

pub fn selection_convert<P: IsA<Widget>>(
    widget: &P,
    selection: &gdk::Atom,
    target: &gdk::Atom,
    time_: u32,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_selection_convert(
            widget.as_ref().to_glib_none().0,
            selection.to_glib_none().0,
            target.to_glib_none().0,
            time_,
        ))
    }
}

pub fn selection_owner_set<P: IsA<Widget>>(
    widget: Option<&P>,
    selection: &gdk::Atom,
    time_: u32,
) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_selection_owner_set(
            widget.map(|p| p.as_ref()).to_glib_none().0,
            selection.to_glib_none().0,
            time_,
        ))
    }
}

pub fn selection_owner_set_for_display<P: IsA<Widget>>(
    display: &gdk::Display,
    widget: Option<&P>,
    selection: &gdk::Atom,
    time_: u32,
) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_selection_owner_set_for_display(
            display.to_glib_none().0,
            widget.map(|p| p.as_ref()).to_glib_none().0,
            selection.to_glib_none().0,
            time_,
        ))
    }
}

pub fn selection_remove_all<P: IsA<Widget>>(widget: &P) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_selection_remove_all(widget.as_ref().to_glib_none().0);
    }
}

pub fn set_debug_flags(flags: u32) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_set_debug_flags(flags);
    }
}

//pub fn show_about_dialog<P: IsA<Window>>(parent: Option<&P>, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:gtk_show_about_dialog() }
//}

pub fn show_uri(
    screen: Option<&gdk::Screen>,
    uri: &str,
    timestamp: u32,
) -> Result<(), glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::gtk_show_uri(
            screen.to_glib_none().0,
            uri.to_glib_none().0,
            timestamp,
            &mut error,
        );
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
pub fn show_uri_on_window<P: IsA<Window>>(
    parent: Option<&P>,
    uri: &str,
    timestamp: u32,
) -> Result<(), glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::gtk_show_uri_on_window(
            parent.map(|p| p.as_ref()).to_glib_none().0,
            uri.to_glib_none().0,
            timestamp,
            &mut error,
        );
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn targets_include_image(targets: &[&gdk::Atom], writable: bool) -> bool {
    assert_initialized_main_thread!();
    let n_targets = targets.len() as i32;
    unsafe {
        from_glib(ffi::gtk_targets_include_image(
            targets.to_glib_none().0,
            n_targets,
            writable.to_glib(),
        ))
    }
}

pub fn targets_include_rich_text<P: IsA<TextBuffer>>(targets: &[&gdk::Atom], buffer: &P) -> bool {
    skip_assert_initialized!();
    let n_targets = targets.len() as i32;
    unsafe {
        from_glib(ffi::gtk_targets_include_rich_text(
            targets.to_glib_none().0,
            n_targets,
            buffer.as_ref().to_glib_none().0,
        ))
    }
}

pub fn targets_include_text(targets: &[&gdk::Atom]) -> bool {
    assert_initialized_main_thread!();
    let n_targets = targets.len() as i32;
    unsafe {
        from_glib(ffi::gtk_targets_include_text(
            targets.to_glib_none().0,
            n_targets,
        ))
    }
}

pub fn targets_include_uri(targets: &[&gdk::Atom]) -> bool {
    assert_initialized_main_thread!();
    let n_targets = targets.len() as i32;
    unsafe {
        from_glib(ffi::gtk_targets_include_uri(
            targets.to_glib_none().0,
            n_targets,
        ))
    }
}

#[cfg_attr(feature = "v3_20", deprecated)]
pub fn test_create_simple_window(window_title: &str, dialog_text: &str) -> Option<Widget> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gtk_test_create_simple_window(
            window_title.to_glib_none().0,
            dialog_text.to_glib_none().0,
        ))
    }
}

//#[cfg_attr(feature = "v3_20", deprecated)]
//pub fn test_create_widget(widget_type: glib::types::Type, first_property_name: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Widget> {
//    unsafe { TODO: call ffi:gtk_test_create_widget() }
//}

//#[cfg_attr(feature = "v3_20", deprecated)]
//pub fn test_display_button_window(window_title: &str, dialog_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Widget> {
//    unsafe { TODO: call ffi:gtk_test_display_button_window() }
//}

pub fn test_find_label<P: IsA<Widget>>(widget: &P, label_pattern: &str) -> Option<Widget> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gtk_test_find_label(
            widget.as_ref().to_glib_none().0,
            label_pattern.to_glib_none().0,
        ))
    }
}

pub fn test_find_sibling<P: IsA<Widget>>(
    base_widget: &P,
    widget_type: glib::types::Type,
) -> Option<Widget> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gtk_test_find_sibling(
            base_widget.as_ref().to_glib_none().0,
            widget_type.to_glib(),
        ))
    }
}

pub fn test_find_widget<P: IsA<Widget>>(
    widget: &P,
    label_pattern: &str,
    widget_type: glib::types::Type,
) -> Option<Widget> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gtk_test_find_widget(
            widget.as_ref().to_glib_none().0,
            label_pattern.to_glib_none().0,
            widget_type.to_glib(),
        ))
    }
}

//pub fn test_init(argvp: /*Unimplemented*/Vec<glib::GString>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:gtk_test_init() }
//}

//pub fn test_list_all_types() -> /*Unimplemented*/CArray TypeId { ns_id: 0, id: 30 } {
//    unsafe { TODO: call ffi:gtk_test_list_all_types() }
//}

pub fn test_register_all_types() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_test_register_all_types();
    }
}

#[cfg_attr(feature = "v3_20", deprecated)]
pub fn test_slider_get_value<P: IsA<Widget>>(widget: &P) -> f64 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_test_slider_get_value(widget.as_ref().to_glib_none().0) }
}

#[cfg_attr(feature = "v3_20", deprecated)]
pub fn test_slider_set_perc<P: IsA<Widget>>(widget: &P, percentage: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_test_slider_set_perc(widget.as_ref().to_glib_none().0, percentage);
    }
}

#[cfg_attr(feature = "v3_20", deprecated)]
pub fn test_spin_button_click<P: IsA<SpinButton>>(spinner: &P, button: u32, upwards: bool) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_spin_button_click(
            spinner.as_ref().to_glib_none().0,
            button,
            upwards.to_glib(),
        ))
    }
}

#[cfg_attr(feature = "v3_20", deprecated)]
pub fn test_text_get<P: IsA<Widget>>(widget: &P) -> Option<glib::GString> {
    skip_assert_initialized!();
    unsafe { from_glib_full(ffi::gtk_test_text_get(widget.as_ref().to_glib_none().0)) }
}

#[cfg_attr(feature = "v3_20", deprecated)]
pub fn test_text_set<P: IsA<Widget>>(widget: &P, string: &str) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_test_text_set(widget.as_ref().to_glib_none().0, string.to_glib_none().0);
    }
}

#[cfg_attr(feature = "v3_20", deprecated)]
pub fn test_widget_click<P: IsA<Widget>>(
    widget: &P,
    button: u32,
    modifiers: gdk::ModifierType,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_widget_click(
            widget.as_ref().to_glib_none().0,
            button,
            modifiers.to_glib(),
        ))
    }
}

pub fn test_widget_send_key<P: IsA<Widget>>(
    widget: &P,
    keyval: u32,
    modifiers: gdk::ModifierType,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_widget_send_key(
            widget.as_ref().to_glib_none().0,
            keyval,
            modifiers.to_glib(),
        ))
    }
}

pub fn test_widget_wait_for_draw<P: IsA<Widget>>(widget: &P) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_test_widget_wait_for_draw(widget.as_ref().to_glib_none().0);
    }
}

pub fn tree_get_row_drag_data(
    selection_data: &SelectionData,
) -> Option<(Option<TreeModel>, Option<TreePath>)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut tree_model = ptr::null_mut();
        let mut path = ptr::null_mut();
        let ret = from_glib(ffi::gtk_tree_get_row_drag_data(
            mut_override(selection_data.to_glib_none().0),
            &mut tree_model,
            &mut path,
        ));
        if ret {
            Some((from_glib_none(tree_model), from_glib_full(path)))
        } else {
            None
        }
    }
}

pub fn tree_set_row_drag_data<P: IsA<TreeModel>>(
    selection_data: &SelectionData,
    tree_model: &P,
    path: &mut TreePath,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_tree_set_row_drag_data(
            mut_override(selection_data.to_glib_none().0),
            tree_model.as_ref().to_glib_none().0,
            path.to_glib_none_mut().0,
        ))
    }
}

pub fn true_() -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_true()) }
}
