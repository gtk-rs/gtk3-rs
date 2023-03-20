// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Adjustment, Align, Buildable, CellEditable, Container, Editable, Entry, EntryBuffer,
    EntryCompletion, InputHints, InputPurpose, Orientable, Orientation, SpinButtonUpdatePolicy,
    SpinType, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkSpinButton")]
    pub struct SpinButton(Object<ffi::GtkSpinButton, ffi::GtkSpinButtonClass>) @extends Entry, Widget, @implements Buildable, CellEditable, Editable, Orientable;

    match fn {
        type_ => || ffi::gtk_spin_button_get_type(),
    }
}

impl SpinButton {
    pub const NONE: Option<&'static SpinButton> = None;

    #[doc(alias = "gtk_spin_button_new")]
    pub fn new(
        adjustment: Option<&impl IsA<Adjustment>>,
        climb_rate: f64,
        digits: u32,
    ) -> SpinButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_spin_button_new(
                adjustment.map(|p| p.as_ref()).to_glib_none().0,
                climb_rate,
                digits,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_spin_button_new_with_range")]
    #[doc(alias = "new_with_range")]
    pub fn with_range(min: f64, max: f64, step: f64) -> SpinButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_spin_button_new_with_range(min, max, step))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SpinButton`] objects.
    ///
    /// This method returns an instance of [`SpinButtonBuilder`](crate::builders::SpinButtonBuilder) which can be used to create [`SpinButton`] objects.
    pub fn builder() -> SpinButtonBuilder {
        SpinButtonBuilder::new()
    }
}

impl Default for SpinButton {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SpinButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SpinButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, SpinButton>,
}

impl SpinButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn adjustment(self, adjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("adjustment", adjustment.clone().upcast()),
        }
    }

    pub fn climb_rate(self, climb_rate: f64) -> Self {
        Self {
            builder: self.builder.property("climb-rate", climb_rate),
        }
    }

    pub fn digits(self, digits: u32) -> Self {
        Self {
            builder: self.builder.property("digits", digits),
        }
    }

    pub fn numeric(self, numeric: bool) -> Self {
        Self {
            builder: self.builder.property("numeric", numeric),
        }
    }

    pub fn snap_to_ticks(self, snap_to_ticks: bool) -> Self {
        Self {
            builder: self.builder.property("snap-to-ticks", snap_to_ticks),
        }
    }

    pub fn update_policy(self, update_policy: SpinButtonUpdatePolicy) -> Self {
        Self {
            builder: self.builder.property("update-policy", update_policy),
        }
    }

    pub fn value(self, value: f64) -> Self {
        Self {
            builder: self.builder.property("value", value),
        }
    }

    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            builder: self.builder.property("wrap", wrap),
        }
    }

    pub fn activates_default(self, activates_default: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("activates-default", activates_default),
        }
    }

    pub fn attributes(self, attributes: &pango::AttrList) -> Self {
        Self {
            builder: self.builder.property("attributes", attributes.clone()),
        }
    }

    pub fn buffer(self, buffer: &impl IsA<EntryBuffer>) -> Self {
        Self {
            builder: self.builder.property("buffer", buffer.clone().upcast()),
        }
    }

    pub fn caps_lock_warning(self, caps_lock_warning: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("caps-lock-warning", caps_lock_warning),
        }
    }

    pub fn completion(self, completion: &impl IsA<EntryCompletion>) -> Self {
        Self {
            builder: self
                .builder
                .property("completion", completion.clone().upcast()),
        }
    }

    pub fn editable(self, editable: bool) -> Self {
        Self {
            builder: self.builder.property("editable", editable),
        }
    }

    pub fn enable_emoji_completion(self, enable_emoji_completion: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("enable-emoji-completion", enable_emoji_completion),
        }
    }

    pub fn has_frame(self, has_frame: bool) -> Self {
        Self {
            builder: self.builder.property("has-frame", has_frame),
        }
    }

    pub fn im_module(self, im_module: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("im-module", im_module.into()),
        }
    }

    pub fn input_hints(self, input_hints: InputHints) -> Self {
        Self {
            builder: self.builder.property("input-hints", input_hints),
        }
    }

    pub fn input_purpose(self, input_purpose: InputPurpose) -> Self {
        Self {
            builder: self.builder.property("input-purpose", input_purpose),
        }
    }

    pub fn invisible_char(self, invisible_char: u32) -> Self {
        Self {
            builder: self.builder.property("invisible-char", invisible_char),
        }
    }

    pub fn invisible_char_set(self, invisible_char_set: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("invisible-char-set", invisible_char_set),
        }
    }

    pub fn max_length(self, max_length: i32) -> Self {
        Self {
            builder: self.builder.property("max-length", max_length),
        }
    }

    pub fn max_width_chars(self, max_width_chars: i32) -> Self {
        Self {
            builder: self.builder.property("max-width-chars", max_width_chars),
        }
    }

    pub fn overwrite_mode(self, overwrite_mode: bool) -> Self {
        Self {
            builder: self.builder.property("overwrite-mode", overwrite_mode),
        }
    }

    pub fn placeholder_text(self, placeholder_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("placeholder-text", placeholder_text.into()),
        }
    }

    pub fn populate_all(self, populate_all: bool) -> Self {
        Self {
            builder: self.builder.property("populate-all", populate_all),
        }
    }

    pub fn primary_icon_activatable(self, primary_icon_activatable: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-icon-activatable", primary_icon_activatable),
        }
    }

    pub fn primary_icon_gicon(self, primary_icon_gicon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-icon-gicon", primary_icon_gicon.clone().upcast()),
        }
    }

    pub fn primary_icon_name(self, primary_icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-icon-name", primary_icon_name.into()),
        }
    }

    pub fn primary_icon_pixbuf(self, primary_icon_pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-icon-pixbuf", primary_icon_pixbuf.clone()),
        }
    }

    pub fn primary_icon_sensitive(self, primary_icon_sensitive: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-icon-sensitive", primary_icon_sensitive),
        }
    }

    pub fn primary_icon_tooltip_markup(
        self,
        primary_icon_tooltip_markup: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "primary-icon-tooltip-markup",
                primary_icon_tooltip_markup.into(),
            ),
        }
    }

    pub fn primary_icon_tooltip_text(
        self,
        primary_icon_tooltip_text: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "primary-icon-tooltip-text",
                primary_icon_tooltip_text.into(),
            ),
        }
    }

    pub fn progress_fraction(self, progress_fraction: f64) -> Self {
        Self {
            builder: self
                .builder
                .property("progress-fraction", progress_fraction),
        }
    }

    pub fn progress_pulse_step(self, progress_pulse_step: f64) -> Self {
        Self {
            builder: self
                .builder
                .property("progress-pulse-step", progress_pulse_step),
        }
    }

    pub fn secondary_icon_activatable(self, secondary_icon_activatable: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("secondary-icon-activatable", secondary_icon_activatable),
        }
    }

    pub fn secondary_icon_gicon(self, secondary_icon_gicon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self.builder.property(
                "secondary-icon-gicon",
                secondary_icon_gicon.clone().upcast(),
            ),
        }
    }

    pub fn secondary_icon_name(self, secondary_icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("secondary-icon-name", secondary_icon_name.into()),
        }
    }

    pub fn secondary_icon_pixbuf(self, secondary_icon_pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self
                .builder
                .property("secondary-icon-pixbuf", secondary_icon_pixbuf.clone()),
        }
    }

    pub fn secondary_icon_sensitive(self, secondary_icon_sensitive: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("secondary-icon-sensitive", secondary_icon_sensitive),
        }
    }

    pub fn secondary_icon_tooltip_markup(
        self,
        secondary_icon_tooltip_markup: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "secondary-icon-tooltip-markup",
                secondary_icon_tooltip_markup.into(),
            ),
        }
    }

    pub fn secondary_icon_tooltip_text(
        self,
        secondary_icon_tooltip_text: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "secondary-icon-tooltip-text",
                secondary_icon_tooltip_text.into(),
            ),
        }
    }

    pub fn show_emoji_icon(self, show_emoji_icon: bool) -> Self {
        Self {
            builder: self.builder.property("show-emoji-icon", show_emoji_icon),
        }
    }

    pub fn tabs(self, tabs: &pango::TabArray) -> Self {
        Self {
            builder: self.builder.property("tabs", tabs),
        }
    }

    pub fn text(self, text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("text", text.into()),
        }
    }

    pub fn truncate_multiline(self, truncate_multiline: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("truncate-multiline", truncate_multiline),
        }
    }

    pub fn visibility(self, visibility: bool) -> Self {
        Self {
            builder: self.builder.property("visibility", visibility),
        }
    }

    pub fn width_chars(self, width_chars: i32) -> Self {
        Self {
            builder: self.builder.property("width-chars", width_chars),
        }
    }

    pub fn xalign(self, xalign: f32) -> Self {
        Self {
            builder: self.builder.property("xalign", xalign),
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

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SpinButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SpinButton {
        self.builder.build()
    }
}

pub trait SpinButtonExt: 'static {
    #[doc(alias = "gtk_spin_button_configure")]
    fn configure(&self, adjustment: Option<&impl IsA<Adjustment>>, climb_rate: f64, digits: u32);

    #[doc(alias = "gtk_spin_button_get_adjustment")]
    #[doc(alias = "get_adjustment")]
    fn adjustment(&self) -> Adjustment;

    #[doc(alias = "gtk_spin_button_get_digits")]
    #[doc(alias = "get_digits")]
    fn digits(&self) -> u32;

    #[doc(alias = "gtk_spin_button_get_increments")]
    #[doc(alias = "get_increments")]
    fn increments(&self) -> (f64, f64);

    #[doc(alias = "gtk_spin_button_get_numeric")]
    #[doc(alias = "get_numeric")]
    fn is_numeric(&self) -> bool;

    #[doc(alias = "gtk_spin_button_get_range")]
    #[doc(alias = "get_range")]
    fn range(&self) -> (f64, f64);

    #[doc(alias = "gtk_spin_button_get_snap_to_ticks")]
    #[doc(alias = "get_snap_to_ticks")]
    fn snaps_to_ticks(&self) -> bool;

    #[doc(alias = "gtk_spin_button_get_update_policy")]
    #[doc(alias = "get_update_policy")]
    fn update_policy(&self) -> SpinButtonUpdatePolicy;

    #[doc(alias = "gtk_spin_button_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self) -> f64;

    #[doc(alias = "gtk_spin_button_get_value_as_int")]
    #[doc(alias = "get_value_as_int")]
    fn value_as_int(&self) -> i32;

    #[doc(alias = "gtk_spin_button_get_wrap")]
    #[doc(alias = "get_wrap")]
    fn wraps(&self) -> bool;

    #[doc(alias = "gtk_spin_button_set_adjustment")]
    fn set_adjustment(&self, adjustment: &impl IsA<Adjustment>);

    #[doc(alias = "gtk_spin_button_set_digits")]
    fn set_digits(&self, digits: u32);

    #[doc(alias = "gtk_spin_button_set_increments")]
    fn set_increments(&self, step: f64, page: f64);

    #[doc(alias = "gtk_spin_button_set_numeric")]
    fn set_numeric(&self, numeric: bool);

    #[doc(alias = "gtk_spin_button_set_range")]
    fn set_range(&self, min: f64, max: f64);

    #[doc(alias = "gtk_spin_button_set_snap_to_ticks")]
    fn set_snap_to_ticks(&self, snap_to_ticks: bool);

    #[doc(alias = "gtk_spin_button_set_update_policy")]
    fn set_update_policy(&self, policy: SpinButtonUpdatePolicy);

    #[doc(alias = "gtk_spin_button_set_value")]
    fn set_value(&self, value: f64);

    #[doc(alias = "gtk_spin_button_set_wrap")]
    fn set_wrap(&self, wrap: bool);

    #[doc(alias = "gtk_spin_button_spin")]
    fn spin(&self, direction: SpinType, increment: f64);

    #[doc(alias = "gtk_spin_button_update")]
    fn update(&self);

    #[doc(alias = "climb-rate")]
    fn climb_rate(&self) -> f64;

    #[doc(alias = "climb-rate")]
    fn set_climb_rate(&self, climb_rate: f64);

    #[doc(alias = "adjustment")]
    fn connect_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "climb-rate")]
    fn connect_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "digits")]
    fn connect_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "numeric")]
    fn connect_numeric_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "snap-to-ticks")]
    fn connect_snap_to_ticks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "update-policy")]
    fn connect_update_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "value")]
    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "wrap")]
    fn connect_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SpinButton>> SpinButtonExt for O {
    fn configure(&self, adjustment: Option<&impl IsA<Adjustment>>, climb_rate: f64, digits: u32) {
        unsafe {
            ffi::gtk_spin_button_configure(
                self.as_ref().to_glib_none().0,
                adjustment.map(|p| p.as_ref()).to_glib_none().0,
                climb_rate,
                digits,
            );
        }
    }

    fn adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_spin_button_get_adjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn digits(&self) -> u32 {
        unsafe { ffi::gtk_spin_button_get_digits(self.as_ref().to_glib_none().0) }
    }

    fn increments(&self) -> (f64, f64) {
        unsafe {
            let mut step = mem::MaybeUninit::uninit();
            let mut page = mem::MaybeUninit::uninit();
            ffi::gtk_spin_button_get_increments(
                self.as_ref().to_glib_none().0,
                step.as_mut_ptr(),
                page.as_mut_ptr(),
            );
            (step.assume_init(), page.assume_init())
        }
    }

    fn is_numeric(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_numeric(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn range(&self) -> (f64, f64) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            ffi::gtk_spin_button_get_range(
                self.as_ref().to_glib_none().0,
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            (min.assume_init(), max.assume_init())
        }
    }

    fn snaps_to_ticks(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_snap_to_ticks(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn update_policy(&self) -> SpinButtonUpdatePolicy {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_update_policy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn value(&self) -> f64 {
        unsafe { ffi::gtk_spin_button_get_value(self.as_ref().to_glib_none().0) }
    }

    fn value_as_int(&self) -> i32 {
        unsafe { ffi::gtk_spin_button_get_value_as_int(self.as_ref().to_glib_none().0) }
    }

    fn wraps(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_wrap(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_adjustment(&self, adjustment: &impl IsA<Adjustment>) {
        unsafe {
            ffi::gtk_spin_button_set_adjustment(
                self.as_ref().to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_digits(&self, digits: u32) {
        unsafe {
            ffi::gtk_spin_button_set_digits(self.as_ref().to_glib_none().0, digits);
        }
    }

    fn set_increments(&self, step: f64, page: f64) {
        unsafe {
            ffi::gtk_spin_button_set_increments(self.as_ref().to_glib_none().0, step, page);
        }
    }

    fn set_numeric(&self, numeric: bool) {
        unsafe {
            ffi::gtk_spin_button_set_numeric(self.as_ref().to_glib_none().0, numeric.into_glib());
        }
    }

    fn set_range(&self, min: f64, max: f64) {
        unsafe {
            ffi::gtk_spin_button_set_range(self.as_ref().to_glib_none().0, min, max);
        }
    }

    fn set_snap_to_ticks(&self, snap_to_ticks: bool) {
        unsafe {
            ffi::gtk_spin_button_set_snap_to_ticks(
                self.as_ref().to_glib_none().0,
                snap_to_ticks.into_glib(),
            );
        }
    }

    fn set_update_policy(&self, policy: SpinButtonUpdatePolicy) {
        unsafe {
            ffi::gtk_spin_button_set_update_policy(
                self.as_ref().to_glib_none().0,
                policy.into_glib(),
            );
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_spin_button_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_wrap(&self, wrap: bool) {
        unsafe {
            ffi::gtk_spin_button_set_wrap(self.as_ref().to_glib_none().0, wrap.into_glib());
        }
    }

    fn spin(&self, direction: SpinType, increment: f64) {
        unsafe {
            ffi::gtk_spin_button_spin(
                self.as_ref().to_glib_none().0,
                direction.into_glib(),
                increment,
            );
        }
    }

    fn update(&self) {
        unsafe {
            ffi::gtk_spin_button_update(self.as_ref().to_glib_none().0);
        }
    }

    fn climb_rate(&self) -> f64 {
        glib::ObjectExt::property(self.as_ref(), "climb-rate")
    }

    fn set_climb_rate(&self, climb_rate: f64) {
        glib::ObjectExt::set_property(self.as_ref(), "climb-rate", climb_rate)
    }

    fn connect_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adjustment_trampoline<
            P: IsA<SpinButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SpinButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_adjustment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_climb_rate_trampoline<
            P: IsA<SpinButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SpinButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::climb-rate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_climb_rate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_digits_trampoline<P: IsA<SpinButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SpinButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::digits\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_digits_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_numeric_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_numeric_trampoline<P: IsA<SpinButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SpinButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::numeric\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_numeric_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_snap_to_ticks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_snap_to_ticks_trampoline<
            P: IsA<SpinButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SpinButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::snap-to-ticks\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_snap_to_ticks_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_update_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_update_policy_trampoline<
            P: IsA<SpinButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SpinButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::update-policy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_update_policy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P: IsA<SpinButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SpinButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_trampoline<P: IsA<SpinButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SpinButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wrap\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wrap_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SpinButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SpinButton")
    }
}
