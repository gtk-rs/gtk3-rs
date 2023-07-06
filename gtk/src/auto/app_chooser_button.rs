// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, AppChooser, Bin, Buildable, CellArea, CellEditable, CellLayout, ComboBox, Container,
    ResizeMode, SensitivityType, TreeModel, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkAppChooserButton")]
    pub struct AppChooserButton(Object<ffi::GtkAppChooserButton, ffi::GtkAppChooserButtonClass>) @extends ComboBox, Bin, Container, Widget, @implements Buildable, CellEditable, CellLayout, AppChooser;

    match fn {
        type_ => || ffi::gtk_app_chooser_button_get_type(),
    }
}

impl AppChooserButton {
    pub const NONE: Option<&'static AppChooserButton> = None;

    #[doc(alias = "gtk_app_chooser_button_new")]
    pub fn new(content_type: &str) -> AppChooserButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_button_new(
                content_type.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AppChooserButton`] objects.
    ///
    /// This method returns an instance of [`AppChooserButtonBuilder`](crate::builders::AppChooserButtonBuilder) which can be used to create [`AppChooserButton`] objects.
    pub fn builder() -> AppChooserButtonBuilder {
        AppChooserButtonBuilder::new()
    }
}

impl Default for AppChooserButton {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AppChooserButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AppChooserButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, AppChooserButton>,
}

impl AppChooserButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn heading(self, heading: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("heading", heading.into()),
        }
    }

    pub fn show_default_item(self, show_default_item: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-default-item", show_default_item),
        }
    }

    pub fn show_dialog_item(self, show_dialog_item: bool) -> Self {
        Self {
            builder: self.builder.property("show-dialog-item", show_dialog_item),
        }
    }

    pub fn active(self, active: i32) -> Self {
        Self {
            builder: self.builder.property("active", active),
        }
    }

    pub fn active_id(self, active_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("active-id", active_id.into()),
        }
    }

    pub fn button_sensitivity(self, button_sensitivity: SensitivityType) -> Self {
        Self {
            builder: self
                .builder
                .property("button-sensitivity", button_sensitivity),
        }
    }

    pub fn cell_area(self, cell_area: &impl IsA<CellArea>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-area", cell_area.clone().upcast()),
        }
    }

    pub fn column_span_column(self, column_span_column: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("column-span-column", column_span_column),
        }
    }

    pub fn entry_text_column(self, entry_text_column: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("entry-text-column", entry_text_column),
        }
    }

    pub fn has_entry(self, has_entry: bool) -> Self {
        Self {
            builder: self.builder.property("has-entry", has_entry),
        }
    }

    pub fn has_frame(self, has_frame: bool) -> Self {
        Self {
            builder: self.builder.property("has-frame", has_frame),
        }
    }

    pub fn id_column(self, id_column: i32) -> Self {
        Self {
            builder: self.builder.property("id-column", id_column),
        }
    }

    pub fn model(self, model: &impl IsA<TreeModel>) -> Self {
        Self {
            builder: self.builder.property("model", model.clone().upcast()),
        }
    }

    pub fn popup_fixed_width(self, popup_fixed_width: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("popup-fixed-width", popup_fixed_width),
        }
    }

    pub fn row_span_column(self, row_span_column: i32) -> Self {
        Self {
            builder: self.builder.property("row-span-column", row_span_column),
        }
    }

    pub fn wrap_width(self, wrap_width: i32) -> Self {
        Self {
            builder: self.builder.property("wrap-width", wrap_width),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn editing_canceled(self, editing_canceled: bool) -> Self {
        Self {
            builder: self.builder.property("editing-canceled", editing_canceled),
        }
    }

    pub fn content_type(self, content_type: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("content-type", content_type.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AppChooserButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> AppChooserButton {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AppChooserButton>> Sealed for T {}
}

pub trait AppChooserButtonExt: IsA<AppChooserButton> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_app_chooser_button_append_custom_item")]
    fn append_custom_item(&self, name: &str, label: &str, icon: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::gtk_app_chooser_button_append_custom_item(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                label.to_glib_none().0,
                icon.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_app_chooser_button_append_separator")]
    fn append_separator(&self) {
        unsafe {
            ffi::gtk_app_chooser_button_append_separator(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_app_chooser_button_get_heading")]
    #[doc(alias = "get_heading")]
    fn heading(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_button_get_heading(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_app_chooser_button_get_show_default_item")]
    #[doc(alias = "get_show_default_item")]
    fn shows_default_item(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_button_get_show_default_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_app_chooser_button_get_show_dialog_item")]
    #[doc(alias = "get_show_dialog_item")]
    fn shows_dialog_item(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_button_get_show_dialog_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_app_chooser_button_set_active_custom_item")]
    fn set_active_custom_item(&self, name: &str) {
        unsafe {
            ffi::gtk_app_chooser_button_set_active_custom_item(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_app_chooser_button_set_heading")]
    fn set_heading(&self, heading: &str) {
        unsafe {
            ffi::gtk_app_chooser_button_set_heading(
                self.as_ref().to_glib_none().0,
                heading.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_app_chooser_button_set_show_default_item")]
    fn set_show_default_item(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_button_set_show_default_item(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_app_chooser_button_set_show_dialog_item")]
    fn set_show_dialog_item(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_button_set_show_dialog_item(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    #[doc(alias = "custom-item-activated")]
    fn connect_custom_item_activated<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn custom_item_activated_trampoline<
            P: IsA<AppChooserButton>,
            F: Fn(&P, &str) + 'static,
        >(
            this: *mut ffi::GtkAppChooserButton,
            item_name: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                AppChooserButton::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(item_name),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name =
                detail.map(|name| format!("custom-item-activated::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"custom-item-activated\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    custom_item_activated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "heading")]
    fn connect_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_heading_trampoline<
            P: IsA<AppChooserButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAppChooserButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppChooserButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::heading\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_heading_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-default-item")]
    fn connect_show_default_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_default_item_trampoline<
            P: IsA<AppChooserButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAppChooserButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppChooserButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-default-item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_default_item_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-dialog-item")]
    fn connect_show_dialog_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_dialog_item_trampoline<
            P: IsA<AppChooserButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAppChooserButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppChooserButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-dialog-item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_dialog_item_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<AppChooserButton>> AppChooserButtonExt for O {}

impl fmt::Display for AppChooserButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AppChooserButton")
    }
}
