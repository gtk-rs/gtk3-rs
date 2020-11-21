// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::Adjustment;
use crate::Align;
use crate::Buildable;
use crate::Container;
use crate::ListBoxRow;
use crate::MovementStep;
use crate::ResizeMode;
use crate::SelectionMode;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct ListBox(Object<ffi::GtkListBox, ffi::GtkListBoxClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_list_box_get_type(),
    }
}

impl ListBox {
    pub fn new() -> ListBox {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_list_box_new()).unsafe_cast() }
    }
}

impl Default for ListBox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct ListBoxBuilder {
    activate_on_single_click: Option<bool>,
    selection_mode: Option<SelectionMode>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl ListBoxBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ListBox {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref activate_on_single_click) = self.activate_on_single_click {
            properties.push(("activate-on-single-click", activate_on_single_click));
        }
        if let Some(ref selection_mode) = self.selection_mode {
            properties.push(("selection-mode", selection_mode));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        let ret = glib::Object::new(ListBox::static_type(), &properties)
            .expect("object new")
            .downcast::<ListBox>()
            .expect("downcast");
        ret
    }

    pub fn activate_on_single_click(mut self, activate_on_single_click: bool) -> Self {
        self.activate_on_single_click = Some(activate_on_single_click);
        self
    }

    pub fn selection_mode(mut self, selection_mode: SelectionMode) -> Self {
        self.selection_mode = Some(selection_mode);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_LIST_BOX: Option<&ListBox> = None;

pub trait ListBoxExt: 'static {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn bind_model<P: IsA<gio::ListModel>, Q: Fn(&glib::Object) -> Widget + 'static>(
        &self,
        model: Option<&P>,
        create_widget_func: Q,
    );

    fn drag_highlight_row<P: IsA<ListBoxRow>>(&self, row: &P);

    fn drag_unhighlight_row(&self);

    fn get_activate_on_single_click(&self) -> bool;

    fn get_adjustment(&self) -> Option<Adjustment>;

    fn get_row_at_index(&self, index_: i32) -> Option<ListBoxRow>;

    fn get_row_at_y(&self, y: i32) -> Option<ListBoxRow>;

    fn get_selected_row(&self) -> Option<ListBoxRow>;

    fn get_selected_rows(&self) -> Vec<ListBoxRow>;

    fn get_selection_mode(&self) -> SelectionMode;

    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32);

    fn invalidate_filter(&self);

    fn invalidate_headers(&self);

    fn invalidate_sort(&self);

    fn prepend<P: IsA<Widget>>(&self, child: &P);

    fn select_all(&self);

    fn select_row<P: IsA<ListBoxRow>>(&self, row: Option<&P>);

    fn selected_foreach<P: FnMut(&ListBox, &ListBoxRow)>(&self, func: P);

    fn set_activate_on_single_click(&self, single: bool);

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: Option<&P>);

    fn set_filter_func(&self, filter_func: Option<Box_<dyn Fn(&ListBoxRow) -> bool + 'static>>);

    fn set_header_func(
        &self,
        update_header: Option<Box_<dyn Fn(&ListBoxRow, Option<&ListBoxRow>) + 'static>>,
    );

    fn set_placeholder<P: IsA<Widget>>(&self, placeholder: Option<&P>);

    fn set_selection_mode(&self, mode: SelectionMode);

    fn set_sort_func(
        &self,
        sort_func: Option<Box_<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>,
    );

    fn unselect_all(&self);

    fn unselect_row<P: IsA<ListBoxRow>>(&self, row: &P);

    fn connect_activate_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_cursor_row(&self);

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_move_cursor(&self, object: MovementStep, p0: i32);

    fn connect_row_activated<F: Fn(&Self, &ListBoxRow) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_row_selected<F: Fn(&Self, Option<&ListBoxRow>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_select_all(&self);

    fn connect_selected_rows_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_toggle_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_toggle_cursor_row(&self);

    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_unselect_all(&self);

    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<ListBox>> ListBoxExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn bind_model<P: IsA<gio::ListModel>, Q: Fn(&glib::Object) -> Widget + 'static>(
        &self,
        model: Option<&P>,
        create_widget_func: Q,
    ) {
        let create_widget_func_data: Box_<Q> = Box_::new(create_widget_func);
        unsafe extern "C" fn create_widget_func_func<
            P: IsA<gio::ListModel>,
            Q: Fn(&glib::Object) -> Widget + 'static,
        >(
            item: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
        ) -> *mut ffi::GtkWidget {
            let item = from_glib_borrow(item);
            let callback: &Q = &*(user_data as *mut _);
            let res = (*callback)(&item);
            res.to_glib_full()
        }
        let create_widget_func = Some(create_widget_func_func::<P, Q> as _);
        unsafe extern "C" fn user_data_free_func_func<
            P: IsA<gio::ListModel>,
            Q: Fn(&glib::Object) -> Widget + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<Q> = Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(user_data_free_func_func::<P, Q> as _);
        let super_callback0: Box_<Q> = create_widget_func_data;
        unsafe {
            ffi::gtk_list_box_bind_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
                create_widget_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call4,
            );
        }
    }

    fn drag_highlight_row<P: IsA<ListBoxRow>>(&self, row: &P) {
        unsafe {
            ffi::gtk_list_box_drag_highlight_row(
                self.as_ref().to_glib_none().0,
                row.as_ref().to_glib_none().0,
            );
        }
    }

    fn drag_unhighlight_row(&self) {
        unsafe {
            ffi::gtk_list_box_drag_unhighlight_row(self.as_ref().to_glib_none().0);
        }
    }

    fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_get_activate_on_single_click(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_adjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_row_at_index(&self, index_: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_index(
                self.as_ref().to_glib_none().0,
                index_,
            ))
        }
    }

    fn get_row_at_y(&self, y: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_y(
                self.as_ref().to_glib_none().0,
                y,
            ))
        }
    }

    fn get_selected_row(&self) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_selected_row(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_selected_rows(&self) -> Vec<ListBoxRow> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_list_box_get_selected_rows(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            from_glib(ffi::gtk_list_box_get_selection_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            ffi::gtk_list_box_insert(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    fn invalidate_filter(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_filter(self.as_ref().to_glib_none().0);
        }
    }

    fn invalidate_headers(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_headers(self.as_ref().to_glib_none().0);
        }
    }

    fn invalidate_sort(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_sort(self.as_ref().to_glib_none().0);
        }
    }

    fn prepend<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_list_box_prepend(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_list_box_select_all(self.as_ref().to_glib_none().0);
        }
    }

    fn select_row<P: IsA<ListBoxRow>>(&self, row: Option<&P>) {
        unsafe {
            ffi::gtk_list_box_select_row(
                self.as_ref().to_glib_none().0,
                row.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn selected_foreach<P: FnMut(&ListBox, &ListBoxRow)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&ListBox, &ListBoxRow)>(
            box_: *mut ffi::GtkListBox,
            row: *mut ffi::GtkListBoxRow,
            user_data: glib::ffi::gpointer,
        ) {
            let box_ = from_glib_borrow(box_);
            let row = from_glib_borrow(row);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&box_, &row);
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_list_box_selected_foreach(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_list_box_set_activate_on_single_click(
                self.as_ref().to_glib_none().0,
                single.to_glib(),
            );
        }
    }

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: Option<&P>) {
        unsafe {
            ffi::gtk_list_box_set_adjustment(
                self.as_ref().to_glib_none().0,
                adjustment.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_filter_func(&self, filter_func: Option<Box_<dyn Fn(&ListBoxRow) -> bool + 'static>>) {
        let filter_func_data: Box_<Option<Box_<dyn Fn(&ListBoxRow) -> bool + 'static>>> =
            Box_::new(filter_func);
        unsafe extern "C" fn filter_func_func(
            row: *mut ffi::GtkListBoxRow,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let row = from_glib_borrow(row);
            let callback: &Option<Box_<dyn Fn(&ListBoxRow) -> bool + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&row)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let filter_func = if filter_func_data.is_some() {
            Some(filter_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&ListBoxRow) -> bool + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(&ListBoxRow) -> bool + 'static>>> =
            filter_func_data;
        unsafe {
            ffi::gtk_list_box_set_filter_func(
                self.as_ref().to_glib_none().0,
                filter_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_header_func(
        &self,
        update_header: Option<Box_<dyn Fn(&ListBoxRow, Option<&ListBoxRow>) + 'static>>,
    ) {
        let update_header_data: Box_<
            Option<Box_<dyn Fn(&ListBoxRow, Option<&ListBoxRow>) + 'static>>,
        > = Box_::new(update_header);
        unsafe extern "C" fn update_header_func(
            row: *mut ffi::GtkListBoxRow,
            before: *mut ffi::GtkListBoxRow,
            user_data: glib::ffi::gpointer,
        ) {
            let row = from_glib_borrow(row);
            let before: Borrowed<Option<ListBoxRow>> = from_glib_borrow(before);
            let callback: &Option<Box_<dyn Fn(&ListBoxRow, Option<&ListBoxRow>) + 'static>> =
                &*(user_data as *mut _);
            if let Some(ref callback) = *callback {
                callback(&row, before.as_ref().as_ref())
            } else {
                panic!("cannot get closure...")
            };
        }
        let update_header = if update_header_data.is_some() {
            Some(update_header_func as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&ListBoxRow, Option<&ListBoxRow>) + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<
            Option<Box_<dyn Fn(&ListBoxRow, Option<&ListBoxRow>) + 'static>>,
        > = update_header_data;
        unsafe {
            ffi::gtk_list_box_set_header_func(
                self.as_ref().to_glib_none().0,
                update_header,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_placeholder<P: IsA<Widget>>(&self, placeholder: Option<&P>) {
        unsafe {
            ffi::gtk_list_box_set_placeholder(
                self.as_ref().to_glib_none().0,
                placeholder.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_list_box_set_selection_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    fn set_sort_func(
        &self,
        sort_func: Option<Box_<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>,
    ) {
        let sort_func_data: Box_<Option<Box_<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>> =
            Box_::new(sort_func);
        unsafe extern "C" fn sort_func_func(
            row1: *mut ffi::GtkListBoxRow,
            row2: *mut ffi::GtkListBoxRow,
            user_data: glib::ffi::gpointer,
        ) -> libc::c_int {
            let row1 = from_glib_borrow(row1);
            let row2 = from_glib_borrow(row2);
            let callback: &Option<Box_<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&row1, &row2)
            } else {
                panic!("cannot get closure...")
            };
            res
        }
        let sort_func = if sort_func_data.is_some() {
            Some(sort_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>> =
            sort_func_data;
        unsafe {
            ffi::gtk_list_box_set_sort_func(
                self.as_ref().to_glib_none().0,
                sort_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_list_box_unselect_all(self.as_ref().to_glib_none().0);
        }
    }

    fn unselect_row<P: IsA<ListBoxRow>>(&self, row: &P) {
        unsafe {
            ffi::gtk_list_box_unselect_row(
                self.as_ref().to_glib_none().0,
                row.as_ref().to_glib_none().0,
            );
        }
    }

    fn connect_activate_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_cursor_row_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListBox,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ListBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-cursor-row\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_cursor_row_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate_cursor_row(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("activate-cursor-row", &[])
                .unwrap()
        };
    }

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_cursor_trampoline<P, F: Fn(&P, MovementStep, i32) + 'static>(
            this: *mut ffi::GtkListBox,
            object: ffi::GtkMovementStep,
            p0: libc::c_int,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ListBox>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ListBox::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(object),
                p0,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-cursor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_cursor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_cursor(&self, object: MovementStep, p0: i32) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("move-cursor", &[&object, &p0])
                .unwrap()
        };
    }

    fn connect_row_activated<F: Fn(&Self, &ListBoxRow) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn row_activated_trampoline<P, F: Fn(&P, &ListBoxRow) + 'static>(
            this: *mut ffi::GtkListBox,
            row: *mut ffi::GtkListBoxRow,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ListBox>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ListBox::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(row),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"row-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    row_activated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_row_selected<F: Fn(&Self, Option<&ListBoxRow>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn row_selected_trampoline<P, F: Fn(&P, Option<&ListBoxRow>) + 'static>(
            this: *mut ffi::GtkListBox,
            row: *mut ffi::GtkListBoxRow,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ListBox>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ListBox::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<ListBoxRow>::from_glib_borrow(row)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"row-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    row_selected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn select_all_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListBox,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ListBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"select-all\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    select_all_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_select_all(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("select-all", &[])
                .unwrap()
        };
    }

    fn connect_selected_rows_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn selected_rows_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListBox,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ListBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selected-rows-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    selected_rows_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_toggle_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggle_cursor_row_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListBox,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ListBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggle-cursor-row\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toggle_cursor_row_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_toggle_cursor_row(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("toggle-cursor-row", &[])
                .unwrap()
        };
    }

    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unselect_all_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListBox,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ListBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unselect-all\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    unselect_all_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_unselect_all(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("unselect-all", &[])
                .unwrap()
        };
    }

    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_activate_on_single_click_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ListBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::activate-on-single-click\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activate_on_single_click_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_selection_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ListBox>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selection-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selection_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ListBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListBox")
    }
}
