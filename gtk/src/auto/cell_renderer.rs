// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    CellEditable, CellRendererMode, CellRendererState, Requisition, SizeRequestMode, StateFlags,
    TreePath, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkCellRenderer")]
    pub struct CellRenderer(Object<ffi::GtkCellRenderer, ffi::GtkCellRendererClass>);

    match fn {
        type_ => || ffi::gtk_cell_renderer_get_type(),
    }
}

impl CellRenderer {
    pub const NONE: Option<&'static CellRenderer> = None;
}

pub trait CellRendererExt: 'static {
    #[doc(alias = "gtk_cell_renderer_activate")]
    fn activate(
        &self,
        event: &gdk::Event,
        widget: &impl IsA<Widget>,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool;

    #[doc(alias = "gtk_cell_renderer_get_aligned_area")]
    #[doc(alias = "get_aligned_area")]
    fn aligned_area(
        &self,
        widget: &impl IsA<Widget>,
        flags: CellRendererState,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle;

    #[doc(alias = "gtk_cell_renderer_get_alignment")]
    #[doc(alias = "get_alignment")]
    fn alignment(&self) -> (f32, f32);

    #[doc(alias = "gtk_cell_renderer_get_fixed_size")]
    #[doc(alias = "get_fixed_size")]
    fn fixed_size(&self) -> (i32, i32);

    #[doc(alias = "gtk_cell_renderer_get_padding")]
    #[doc(alias = "get_padding")]
    fn padding(&self) -> (i32, i32);

    #[doc(alias = "gtk_cell_renderer_get_preferred_height")]
    #[doc(alias = "get_preferred_height")]
    fn preferred_height(&self, widget: &impl IsA<Widget>) -> (i32, i32);

    #[doc(alias = "gtk_cell_renderer_get_preferred_height_for_width")]
    #[doc(alias = "get_preferred_height_for_width")]
    fn preferred_height_for_width(&self, widget: &impl IsA<Widget>, width: i32) -> (i32, i32);

    #[doc(alias = "gtk_cell_renderer_get_preferred_size")]
    #[doc(alias = "get_preferred_size")]
    fn preferred_size(&self, widget: &impl IsA<Widget>) -> (Requisition, Requisition);

    #[doc(alias = "gtk_cell_renderer_get_preferred_width")]
    #[doc(alias = "get_preferred_width")]
    fn preferred_width(&self, widget: &impl IsA<Widget>) -> (i32, i32);

    #[doc(alias = "gtk_cell_renderer_get_preferred_width_for_height")]
    #[doc(alias = "get_preferred_width_for_height")]
    fn preferred_width_for_height(&self, widget: &impl IsA<Widget>, height: i32) -> (i32, i32);

    #[doc(alias = "gtk_cell_renderer_get_request_mode")]
    #[doc(alias = "get_request_mode")]
    fn request_mode(&self) -> SizeRequestMode;

    #[doc(alias = "gtk_cell_renderer_get_sensitive")]
    #[doc(alias = "get_sensitive")]
    fn is_sensitive(&self) -> bool;

    #[doc(alias = "gtk_cell_renderer_get_state")]
    #[doc(alias = "get_state")]
    fn state(&self, widget: Option<&impl IsA<Widget>>, cell_state: CellRendererState)
        -> StateFlags;

    #[doc(alias = "gtk_cell_renderer_get_visible")]
    #[doc(alias = "get_visible")]
    fn is_visible(&self) -> bool;

    #[doc(alias = "gtk_cell_renderer_is_activatable")]
    fn is_activatable(&self) -> bool;

    #[doc(alias = "gtk_cell_renderer_render")]
    fn render(
        &self,
        cr: &cairo::Context,
        widget: &impl IsA<Widget>,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    );

    #[doc(alias = "gtk_cell_renderer_set_alignment")]
    fn set_alignment(&self, xalign: f32, yalign: f32);

    #[doc(alias = "gtk_cell_renderer_set_fixed_size")]
    fn set_fixed_size(&self, width: i32, height: i32);

    #[doc(alias = "gtk_cell_renderer_set_padding")]
    fn set_padding(&self, xpad: i32, ypad: i32);

    #[doc(alias = "gtk_cell_renderer_set_sensitive")]
    fn set_sensitive(&self, sensitive: bool);

    #[doc(alias = "gtk_cell_renderer_set_visible")]
    fn set_visible(&self, visible: bool);

    #[doc(alias = "gtk_cell_renderer_start_editing")]
    fn start_editing(
        &self,
        event: Option<&gdk::Event>,
        widget: &impl IsA<Widget>,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> Option<CellEditable>;

    #[doc(alias = "gtk_cell_renderer_stop_editing")]
    fn stop_editing(&self, canceled: bool);

    #[doc(alias = "cell-background")]
    fn set_cell_background(&self, cell_background: Option<&str>);

    #[doc(alias = "cell-background-rgba")]
    fn cell_background_rgba(&self) -> Option<gdk::RGBA>;

    #[doc(alias = "cell-background-rgba")]
    fn set_cell_background_rgba(&self, cell_background_rgba: Option<&gdk::RGBA>);

    #[doc(alias = "cell-background-set")]
    fn is_cell_background_set(&self) -> bool;

    #[doc(alias = "cell-background-set")]
    fn set_cell_background_set(&self, cell_background_set: bool);

    fn is_editing(&self) -> bool;

    fn height(&self) -> i32;

    fn set_height(&self, height: i32);

    #[doc(alias = "is-expanded")]
    fn is_expanded(&self) -> bool;

    #[doc(alias = "is-expanded")]
    fn set_is_expanded(&self, is_expanded: bool);

    #[doc(alias = "is-expander")]
    fn is_expander(&self) -> bool;

    #[doc(alias = "is-expander")]
    fn set_is_expander(&self, is_expander: bool);

    fn mode(&self) -> CellRendererMode;

    fn set_mode(&self, mode: CellRendererMode);

    fn width(&self) -> i32;

    fn set_width(&self, width: i32);

    fn xalign(&self) -> f32;

    fn set_xalign(&self, xalign: f32);

    fn xpad(&self) -> u32;

    fn set_xpad(&self, xpad: u32);

    fn yalign(&self) -> f32;

    fn set_yalign(&self, yalign: f32);

    fn ypad(&self) -> u32;

    fn set_ypad(&self, ypad: u32);

    #[doc(alias = "editing-canceled")]
    fn connect_editing_canceled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "editing-started")]
    fn connect_editing_started<F: Fn(&Self, &CellEditable, TreePath) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "cell-background")]
    fn connect_cell_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "cell-background-rgba")]
    fn connect_cell_background_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "cell-background-set")]
    fn connect_cell_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "editing")]
    fn connect_editing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "height")]
    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "is-expanded")]
    fn connect_is_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "is-expander")]
    fn connect_is_expander_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "mode")]
    fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "sensitive")]
    fn connect_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible")]
    fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "width")]
    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "xalign")]
    fn connect_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "xpad")]
    fn connect_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "yalign")]
    fn connect_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "ypad")]
    fn connect_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRenderer>> CellRendererExt for O {
    fn activate(
        &self,
        event: &gdk::Event,
        widget: &impl IsA<Widget>,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_activate(
                self.as_ref().to_glib_none().0,
                mut_override(event.to_glib_none().0),
                widget.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                background_area.to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    fn aligned_area(
        &self,
        widget: &impl IsA<Widget>,
        flags: CellRendererState,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle {
        unsafe {
            let mut aligned_area = gdk::Rectangle::uninitialized();
            ffi::gtk_cell_renderer_get_aligned_area(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                flags.into_glib(),
                cell_area.to_glib_none().0,
                aligned_area.to_glib_none_mut().0,
            );
            aligned_area
        }
    }

    fn alignment(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::MaybeUninit::uninit();
            let mut yalign = mem::MaybeUninit::uninit();
            ffi::gtk_cell_renderer_get_alignment(
                self.as_ref().to_glib_none().0,
                xalign.as_mut_ptr(),
                yalign.as_mut_ptr(),
            );
            (xalign.assume_init(), yalign.assume_init())
        }
    }

    fn fixed_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::gtk_cell_renderer_get_fixed_size(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    fn padding(&self) -> (i32, i32) {
        unsafe {
            let mut xpad = mem::MaybeUninit::uninit();
            let mut ypad = mem::MaybeUninit::uninit();
            ffi::gtk_cell_renderer_get_padding(
                self.as_ref().to_glib_none().0,
                xpad.as_mut_ptr(),
                ypad.as_mut_ptr(),
            );
            (xpad.assume_init(), ypad.assume_init())
        }
    }

    fn preferred_height(&self, widget: &impl IsA<Widget>) -> (i32, i32) {
        unsafe {
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            ffi::gtk_cell_renderer_get_preferred_height(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn preferred_height_for_width(&self, widget: &impl IsA<Widget>, width: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::MaybeUninit::uninit();
            let mut natural_height = mem::MaybeUninit::uninit();
            ffi::gtk_cell_renderer_get_preferred_height_for_width(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                width,
                minimum_height.as_mut_ptr(),
                natural_height.as_mut_ptr(),
            );
            (minimum_height.assume_init(), natural_height.assume_init())
        }
    }

    fn preferred_size(&self, widget: &impl IsA<Widget>) -> (Requisition, Requisition) {
        unsafe {
            let mut minimum_size = Requisition::uninitialized();
            let mut natural_size = Requisition::uninitialized();
            ffi::gtk_cell_renderer_get_preferred_size(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_size.to_glib_none_mut().0,
                natural_size.to_glib_none_mut().0,
            );
            (minimum_size, natural_size)
        }
    }

    fn preferred_width(&self, widget: &impl IsA<Widget>) -> (i32, i32) {
        unsafe {
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            ffi::gtk_cell_renderer_get_preferred_width(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn preferred_width_for_height(&self, widget: &impl IsA<Widget>, height: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::MaybeUninit::uninit();
            let mut natural_width = mem::MaybeUninit::uninit();
            ffi::gtk_cell_renderer_get_preferred_width_for_height(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                height,
                minimum_width.as_mut_ptr(),
                natural_width.as_mut_ptr(),
            );
            (minimum_width.assume_init(), natural_width.assume_init())
        }
    }

    fn request_mode(&self) -> SizeRequestMode {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_get_request_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_sensitive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_get_sensitive(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn state(
        &self,
        widget: Option<&impl IsA<Widget>>,
        cell_state: CellRendererState,
    ) -> StateFlags {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_get_state(
                self.as_ref().to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
                cell_state.into_glib(),
            ))
        }
    }

    fn is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_get_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_is_activatable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn render(
        &self,
        cr: &cairo::Context,
        widget: &impl IsA<Widget>,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) {
        unsafe {
            ffi::gtk_cell_renderer_render(
                self.as_ref().to_glib_none().0,
                mut_override(cr.to_glib_none().0),
                widget.as_ref().to_glib_none().0,
                background_area.to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.into_glib(),
            );
        }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_cell_renderer_set_alignment(self.as_ref().to_glib_none().0, xalign, yalign);
        }
    }

    fn set_fixed_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_cell_renderer_set_fixed_size(self.as_ref().to_glib_none().0, width, height);
        }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe {
            ffi::gtk_cell_renderer_set_padding(self.as_ref().to_glib_none().0, xpad, ypad);
        }
    }

    fn set_sensitive(&self, sensitive: bool) {
        unsafe {
            ffi::gtk_cell_renderer_set_sensitive(
                self.as_ref().to_glib_none().0,
                sensitive.into_glib(),
            );
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_cell_renderer_set_visible(self.as_ref().to_glib_none().0, visible.into_glib());
        }
    }

    fn start_editing(
        &self,
        event: Option<&gdk::Event>,
        widget: &impl IsA<Widget>,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> Option<CellEditable> {
        unsafe {
            from_glib_none(ffi::gtk_cell_renderer_start_editing(
                self.as_ref().to_glib_none().0,
                mut_override(event.to_glib_none().0),
                widget.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                background_area.to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    fn stop_editing(&self, canceled: bool) {
        unsafe {
            ffi::gtk_cell_renderer_stop_editing(
                self.as_ref().to_glib_none().0,
                canceled.into_glib(),
            );
        }
    }

    fn set_cell_background(&self, cell_background: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(), "cell-background", cell_background)
    }

    fn cell_background_rgba(&self) -> Option<gdk::RGBA> {
        glib::ObjectExt::property(self.as_ref(), "cell-background-rgba")
    }

    fn set_cell_background_rgba(&self, cell_background_rgba: Option<&gdk::RGBA>) {
        glib::ObjectExt::set_property(self.as_ref(), "cell-background-rgba", cell_background_rgba)
    }

    fn is_cell_background_set(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "cell-background-set")
    }

    fn set_cell_background_set(&self, cell_background_set: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "cell-background-set", cell_background_set)
    }

    fn is_editing(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "editing")
    }

    fn height(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "height")
    }

    fn set_height(&self, height: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "height", height)
    }

    fn is_expanded(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "is-expanded")
    }

    fn set_is_expanded(&self, is_expanded: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "is-expanded", is_expanded)
    }

    fn is_expander(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "is-expander")
    }

    fn set_is_expander(&self, is_expander: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "is-expander", is_expander)
    }

    fn mode(&self) -> CellRendererMode {
        glib::ObjectExt::property(self.as_ref(), "mode")
    }

    fn set_mode(&self, mode: CellRendererMode) {
        glib::ObjectExt::set_property(self.as_ref(), "mode", mode)
    }

    fn width(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "width")
    }

    fn set_width(&self, width: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "width", width)
    }

    fn xalign(&self) -> f32 {
        glib::ObjectExt::property(self.as_ref(), "xalign")
    }

    fn set_xalign(&self, xalign: f32) {
        glib::ObjectExt::set_property(self.as_ref(), "xalign", xalign)
    }

    fn xpad(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "xpad")
    }

    fn set_xpad(&self, xpad: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "xpad", xpad)
    }

    fn yalign(&self) -> f32 {
        glib::ObjectExt::property(self.as_ref(), "yalign")
    }

    fn set_yalign(&self, yalign: f32) {
        glib::ObjectExt::set_property(self.as_ref(), "yalign", yalign)
    }

    fn ypad(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "ypad")
    }

    fn set_ypad(&self, ypad: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "ypad", ypad)
    }

    fn connect_editing_canceled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn editing_canceled_trampoline<
            P: IsA<CellRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRenderer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"editing-canceled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    editing_canceled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_editing_started<F: Fn(&Self, &CellEditable, TreePath) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn editing_started_trampoline<
            P: IsA<CellRenderer>,
            F: Fn(&P, &CellEditable, TreePath) + 'static,
        >(
            this: *mut ffi::GtkCellRenderer,
            editable: *mut ffi::GtkCellEditable,
            path: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(crate::ffi::gtk_tree_path_new_from_string(path));
            f(
                CellRenderer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(editable),
                path,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"editing-started\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    editing_started_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_cell_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cell_background_trampoline<
            P: IsA<CellRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::cell-background\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cell_background_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_cell_background_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cell_background_rgba_trampoline<
            P: IsA<CellRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::cell-background-rgba\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cell_background_rgba_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_cell_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cell_background_set_trampoline<
            P: IsA<CellRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::cell-background-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cell_background_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_editing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_editing_trampoline<
            P: IsA<CellRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::editing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_editing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P: IsA<CellRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_expanded_trampoline<
            P: IsA<CellRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-expanded\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_expanded_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_expander_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_expander_trampoline<
            P: IsA<CellRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-expander\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_expander_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<P: IsA<CellRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sensitive_trampoline<
            P: IsA<CellRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::sensitive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sensitive_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<
            P: IsA<CellRenderer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P: IsA<CellRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xalign_trampoline<P: IsA<CellRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_xalign_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xpad_trampoline<P: IsA<CellRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xpad\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_xpad_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_yalign_trampoline<P: IsA<CellRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::yalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_yalign_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ypad_trampoline<P: IsA<CellRenderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRenderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ypad\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ypad_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CellRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellRenderer")
    }
}
