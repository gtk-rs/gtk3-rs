// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CellRenderer;
use crate::CellRendererMode;
use crate::TreePath;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkCellRendererToggle")]
    pub struct CellRendererToggle(Object<ffi::GtkCellRendererToggle, ffi::GtkCellRendererToggleClass>) @extends CellRenderer;

    match fn {
        type_ => || ffi::gtk_cell_renderer_toggle_get_type(),
    }
}

impl CellRendererToggle {
    pub const NONE: Option<&'static CellRendererToggle> = None;

    #[doc(alias = "gtk_cell_renderer_toggle_new")]
    pub fn new() -> CellRendererToggle {
        assert_initialized_main_thread!();
        unsafe { CellRenderer::from_glib_none(ffi::gtk_cell_renderer_toggle_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellRendererToggle`] objects.
    ///
    /// This method returns an instance of [`CellRendererToggleBuilder`](crate::builders::CellRendererToggleBuilder) which can be used to create [`CellRendererToggle`] objects.
    pub fn builder() -> CellRendererToggleBuilder {
        CellRendererToggleBuilder::default()
    }
}

impl Default for CellRendererToggle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellRendererToggle`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CellRendererToggleBuilder {
    activatable: Option<bool>,
    active: Option<bool>,
    inconsistent: Option<bool>,
    indicator_size: Option<i32>,
    radio: Option<bool>,
    cell_background: Option<String>,
    cell_background_rgba: Option<gdk::RGBA>,
    cell_background_set: Option<bool>,
    height: Option<i32>,
    is_expanded: Option<bool>,
    is_expander: Option<bool>,
    mode: Option<CellRendererMode>,
    sensitive: Option<bool>,
    visible: Option<bool>,
    width: Option<i32>,
    xalign: Option<f32>,
    xpad: Option<u32>,
    yalign: Option<f32>,
    ypad: Option<u32>,
}

impl CellRendererToggleBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`CellRendererToggleBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellRendererToggle`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CellRendererToggle {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref activatable) = self.activatable {
            properties.push(("activatable", activatable));
        }
        if let Some(ref active) = self.active {
            properties.push(("active", active));
        }
        if let Some(ref inconsistent) = self.inconsistent {
            properties.push(("inconsistent", inconsistent));
        }
        if let Some(ref indicator_size) = self.indicator_size {
            properties.push(("indicator-size", indicator_size));
        }
        if let Some(ref radio) = self.radio {
            properties.push(("radio", radio));
        }
        if let Some(ref cell_background) = self.cell_background {
            properties.push(("cell-background", cell_background));
        }
        if let Some(ref cell_background_rgba) = self.cell_background_rgba {
            properties.push(("cell-background-rgba", cell_background_rgba));
        }
        if let Some(ref cell_background_set) = self.cell_background_set {
            properties.push(("cell-background-set", cell_background_set));
        }
        if let Some(ref height) = self.height {
            properties.push(("height", height));
        }
        if let Some(ref is_expanded) = self.is_expanded {
            properties.push(("is-expanded", is_expanded));
        }
        if let Some(ref is_expander) = self.is_expander {
            properties.push(("is-expander", is_expander));
        }
        if let Some(ref mode) = self.mode {
            properties.push(("mode", mode));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width) = self.width {
            properties.push(("width", width));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref xpad) = self.xpad {
            properties.push(("xpad", xpad));
        }
        if let Some(ref yalign) = self.yalign {
            properties.push(("yalign", yalign));
        }
        if let Some(ref ypad) = self.ypad {
            properties.push(("ypad", ypad));
        }
        glib::Object::new::<CellRendererToggle>(&properties)
            .expect("Failed to create an instance of CellRendererToggle")
    }

    pub fn activatable(mut self, activatable: bool) -> Self {
        self.activatable = Some(activatable);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn inconsistent(mut self, inconsistent: bool) -> Self {
        self.inconsistent = Some(inconsistent);
        self
    }

    pub fn indicator_size(mut self, indicator_size: i32) -> Self {
        self.indicator_size = Some(indicator_size);
        self
    }

    pub fn radio(mut self, radio: bool) -> Self {
        self.radio = Some(radio);
        self
    }

    pub fn cell_background(mut self, cell_background: &str) -> Self {
        self.cell_background = Some(cell_background.to_string());
        self
    }

    pub fn cell_background_rgba(mut self, cell_background_rgba: &gdk::RGBA) -> Self {
        self.cell_background_rgba = Some(cell_background_rgba.clone());
        self
    }

    pub fn cell_background_set(mut self, cell_background_set: bool) -> Self {
        self.cell_background_set = Some(cell_background_set);
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn is_expanded(mut self, is_expanded: bool) -> Self {
        self.is_expanded = Some(is_expanded);
        self
    }

    pub fn is_expander(mut self, is_expander: bool) -> Self {
        self.is_expander = Some(is_expander);
        self
    }

    pub fn mode(mut self, mode: CellRendererMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn xpad(mut self, xpad: u32) -> Self {
        self.xpad = Some(xpad);
        self
    }

    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
        self
    }

    pub fn ypad(mut self, ypad: u32) -> Self {
        self.ypad = Some(ypad);
        self
    }
}

pub trait CellRendererToggleExt: 'static {
    #[doc(alias = "gtk_cell_renderer_toggle_get_activatable")]
    #[doc(alias = "get_activatable")]
    fn is_activatable(&self) -> bool;

    #[doc(alias = "gtk_cell_renderer_toggle_get_active")]
    #[doc(alias = "get_active")]
    fn is_active(&self) -> bool;

    #[doc(alias = "gtk_cell_renderer_toggle_get_radio")]
    #[doc(alias = "get_radio")]
    fn is_radio(&self) -> bool;

    #[doc(alias = "gtk_cell_renderer_toggle_set_activatable")]
    fn set_activatable(&self, setting: bool);

    #[doc(alias = "gtk_cell_renderer_toggle_set_active")]
    fn set_active(&self, setting: bool);

    #[doc(alias = "gtk_cell_renderer_toggle_set_radio")]
    fn set_radio(&self, radio: bool);

    fn is_inconsistent(&self) -> bool;

    fn set_inconsistent(&self, inconsistent: bool);

    #[doc(alias = "indicator-size")]
    fn indicator_size(&self) -> i32;

    #[doc(alias = "indicator-size")]
    fn set_indicator_size(&self, indicator_size: i32);

    #[doc(alias = "toggled")]
    fn connect_toggled<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "activatable")]
    fn connect_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "active")]
    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "inconsistent")]
    fn connect_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "indicator-size")]
    fn connect_indicator_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "radio")]
    fn connect_radio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererToggle>> CellRendererToggleExt for O {
    fn is_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_activatable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_radio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_radio(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_activatable(&self, setting: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_activatable(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    fn set_active(&self, setting: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_active(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    fn set_radio(&self, radio: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_radio(
                self.as_ref().to_glib_none().0,
                radio.into_glib(),
            );
        }
    }

    fn is_inconsistent(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "inconsistent")
    }

    fn set_inconsistent(&self, inconsistent: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "inconsistent", &inconsistent)
    }

    fn indicator_size(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "indicator-size")
    }

    fn set_indicator_size(&self, indicator_size: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "indicator-size", &indicator_size)
    }

    fn connect_toggled<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggled_trampoline<
            P: IsA<CellRendererToggle>,
            F: Fn(&P, TreePath) + 'static,
        >(
            this: *mut ffi::GtkCellRendererToggle,
            path: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(crate::ffi::gtk_tree_path_new_from_string(path));
            f(
                CellRendererToggle::from_glib_borrow(this).unsafe_cast_ref(),
                path,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toggled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_activatable_trampoline<
            P: IsA<CellRendererToggle>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererToggle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererToggle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::activatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activatable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<
            P: IsA<CellRendererToggle>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererToggle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererToggle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inconsistent_trampoline<
            P: IsA<CellRendererToggle>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererToggle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererToggle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inconsistent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inconsistent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_indicator_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_indicator_size_trampoline<
            P: IsA<CellRendererToggle>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererToggle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererToggle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::indicator-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_indicator_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_radio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_radio_trampoline<
            P: IsA<CellRendererToggle>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererToggle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererToggle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::radio\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_radio_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CellRendererToggle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellRendererToggle")
    }
}
