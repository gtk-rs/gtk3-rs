// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{CellRenderer, CellRendererMode};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkCellRendererPixbuf")]
    pub struct CellRendererPixbuf(Object<ffi::GtkCellRendererPixbuf, ffi::GtkCellRendererPixbufClass>) @extends CellRenderer;

    match fn {
        type_ => || ffi::gtk_cell_renderer_pixbuf_get_type(),
    }
}

impl CellRendererPixbuf {
    pub const NONE: Option<&'static CellRendererPixbuf> = None;

    #[doc(alias = "gtk_cell_renderer_pixbuf_new")]
    pub fn new() -> CellRendererPixbuf {
        assert_initialized_main_thread!();
        unsafe { CellRenderer::from_glib_none(ffi::gtk_cell_renderer_pixbuf_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellRendererPixbuf`] objects.
    ///
    /// This method returns an instance of [`CellRendererPixbufBuilder`](crate::builders::CellRendererPixbufBuilder) which can be used to create [`CellRendererPixbuf`] objects.
    pub fn builder() -> CellRendererPixbufBuilder {
        CellRendererPixbufBuilder::new()
    }
}

impl Default for CellRendererPixbuf {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellRendererPixbuf`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CellRendererPixbufBuilder {
    builder: glib::object::ObjectBuilder<'static, CellRendererPixbuf>,
}

impl CellRendererPixbufBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn gicon(self, gicon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self.builder.property("gicon", gicon.clone().upcast()),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn pixbuf(self, pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self.builder.property("pixbuf", pixbuf.clone()),
        }
    }

    pub fn pixbuf_expander_closed(self, pixbuf_expander_closed: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self
                .builder
                .property("pixbuf-expander-closed", pixbuf_expander_closed.clone()),
        }
    }

    pub fn pixbuf_expander_open(self, pixbuf_expander_open: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self
                .builder
                .property("pixbuf-expander-open", pixbuf_expander_open.clone()),
        }
    }

    pub fn stock_detail(self, stock_detail: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("stock-detail", stock_detail.into()),
        }
    }

    pub fn stock_size(self, stock_size: u32) -> Self {
        Self {
            builder: self.builder.property("stock-size", stock_size),
        }
    }

    pub fn surface(self, surface: &cairo::Surface) -> Self {
        Self {
            builder: self.builder.property("surface", surface),
        }
    }

    pub fn cell_background(self, cell_background: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background", cell_background.into()),
        }
    }

    pub fn cell_background_rgba(self, cell_background_rgba: &gdk::RGBA) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-rgba", cell_background_rgba),
        }
    }

    pub fn cell_background_set(self, cell_background_set: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-set", cell_background_set),
        }
    }

    pub fn height(self, height: i32) -> Self {
        Self {
            builder: self.builder.property("height", height),
        }
    }

    pub fn is_expanded(self, is_expanded: bool) -> Self {
        Self {
            builder: self.builder.property("is-expanded", is_expanded),
        }
    }

    pub fn is_expander(self, is_expander: bool) -> Self {
        Self {
            builder: self.builder.property("is-expander", is_expander),
        }
    }

    pub fn mode(self, mode: CellRendererMode) -> Self {
        Self {
            builder: self.builder.property("mode", mode),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width(self, width: i32) -> Self {
        Self {
            builder: self.builder.property("width", width),
        }
    }

    pub fn xalign(self, xalign: f32) -> Self {
        Self {
            builder: self.builder.property("xalign", xalign),
        }
    }

    pub fn xpad(self, xpad: u32) -> Self {
        Self {
            builder: self.builder.property("xpad", xpad),
        }
    }

    pub fn yalign(self, yalign: f32) -> Self {
        Self {
            builder: self.builder.property("yalign", yalign),
        }
    }

    pub fn ypad(self, ypad: u32) -> Self {
        Self {
            builder: self.builder.property("ypad", ypad),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellRendererPixbuf`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CellRendererPixbuf {
        self.builder.build()
    }
}

pub trait CellRendererPixbufExt: 'static {
    fn gicon(&self) -> Option<gio::Icon>;

    fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: Option<&P>);

    #[doc(alias = "icon-name")]
    fn icon_name(&self) -> Option<glib::GString>;

    #[doc(alias = "icon-name")]
    fn set_icon_name(&self, icon_name: Option<&str>);

    fn pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    #[doc(alias = "pixbuf-expander-closed")]
    fn pixbuf_expander_closed(&self) -> Option<gdk_pixbuf::Pixbuf>;

    #[doc(alias = "pixbuf-expander-closed")]
    fn set_pixbuf_expander_closed(&self, pixbuf_expander_closed: Option<&gdk_pixbuf::Pixbuf>);

    #[doc(alias = "pixbuf-expander-open")]
    fn pixbuf_expander_open(&self) -> Option<gdk_pixbuf::Pixbuf>;

    #[doc(alias = "pixbuf-expander-open")]
    fn set_pixbuf_expander_open(&self, pixbuf_expander_open: Option<&gdk_pixbuf::Pixbuf>);

    #[doc(alias = "stock-detail")]
    fn stock_detail(&self) -> Option<glib::GString>;

    #[doc(alias = "stock-detail")]
    fn set_stock_detail(&self, stock_detail: Option<&str>);

    fn surface(&self) -> Option<cairo::Surface>;

    fn set_surface(&self, surface: Option<&cairo::Surface>);

    #[doc(alias = "gicon")]
    fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "icon-name")]
    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "pixbuf")]
    fn connect_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "pixbuf-expander-closed")]
    fn connect_pixbuf_expander_closed_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "pixbuf-expander-open")]
    fn connect_pixbuf_expander_open_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "stock-detail")]
    fn connect_stock_detail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "stock-size")]
    fn connect_stock_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "surface")]
    fn connect_surface_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererPixbuf>> CellRendererPixbufExt for O {
    fn gicon(&self) -> Option<gio::Icon> {
        glib::ObjectExt::property(self.as_ref(), "gicon")
    }

    fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: Option<&P>) {
        glib::ObjectExt::set_property(self.as_ref(), "gicon", gicon)
    }

    fn icon_name(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "icon-name")
    }

    fn set_icon_name(&self, icon_name: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(), "icon-name", icon_name)
    }

    fn pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        glib::ObjectExt::property(self.as_ref(), "pixbuf")
    }

    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        glib::ObjectExt::set_property(self.as_ref(), "pixbuf", pixbuf)
    }

    fn pixbuf_expander_closed(&self) -> Option<gdk_pixbuf::Pixbuf> {
        glib::ObjectExt::property(self.as_ref(), "pixbuf-expander-closed")
    }

    fn set_pixbuf_expander_closed(&self, pixbuf_expander_closed: Option<&gdk_pixbuf::Pixbuf>) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "pixbuf-expander-closed",
            pixbuf_expander_closed,
        )
    }

    fn pixbuf_expander_open(&self) -> Option<gdk_pixbuf::Pixbuf> {
        glib::ObjectExt::property(self.as_ref(), "pixbuf-expander-open")
    }

    fn set_pixbuf_expander_open(&self, pixbuf_expander_open: Option<&gdk_pixbuf::Pixbuf>) {
        glib::ObjectExt::set_property(self.as_ref(), "pixbuf-expander-open", pixbuf_expander_open)
    }

    fn stock_detail(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "stock-detail")
    }

    fn set_stock_detail(&self, stock_detail: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(), "stock-detail", stock_detail)
    }

    fn surface(&self) -> Option<cairo::Surface> {
        glib::ObjectExt::property(self.as_ref(), "surface")
    }

    fn set_surface(&self, surface: Option<&cairo::Surface>) {
        glib::ObjectExt::set_property(self.as_ref(), "surface", surface)
    }

    fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<
            P: IsA<CellRendererPixbuf>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_gicon_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<
            P: IsA<CellRendererPixbuf>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_trampoline<
            P: IsA<CellRendererPixbuf>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixbuf\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_pixbuf_expander_closed_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_expander_closed_trampoline<
            P: IsA<CellRendererPixbuf>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixbuf-expander-closed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_expander_closed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_pixbuf_expander_open_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_expander_open_trampoline<
            P: IsA<CellRendererPixbuf>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixbuf-expander-open\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_expander_open_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_stock_detail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stock_detail_trampoline<
            P: IsA<CellRendererPixbuf>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stock-detail\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stock_detail_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_stock_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stock_size_trampoline<
            P: IsA<CellRendererPixbuf>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stock-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stock_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_surface_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_surface_trampoline<
            P: IsA<CellRendererPixbuf>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::surface\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_surface_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CellRendererPixbuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellRendererPixbuf")
    }
}
