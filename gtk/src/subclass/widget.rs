// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::SignalId;
use libc::c_int;
use std::mem;
use std::num::NonZeroU32;

use glib::Propagation;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use crate::DirectionType;
use crate::DragResult;
use crate::Orientation;
use crate::SelectionData;
use crate::SizeRequestMode;
use crate::StateFlags;
use crate::TextDirection;
use crate::Tooltip;
use crate::Widget;
use crate::WidgetHelpType;
use crate::prelude::*;
use crate::{Allocation, ffi};

pub trait WidgetImpl: ObjectImpl + ObjectSubclass<Type: IsA<Widget>> {
    // Do not provide an override for this, as widget implementors should just use
    // WidgetClassSubclassExt::set_accessible_type().  The base gtk_widget_real_get_accessible()
    // does caching (because this vfunc is transfer-none) and initialization, and it's probably
    // best that we don't encourage people to take on the burden of getting that right.
    //fn get_accessible(&self) -> atk::Accessible;

    // Deprecated, use state_flags_changed instead.
    //fn state_changed(&self, previous_state: StateType);

    // Deprecated, use style_updated instead.
    //fn style_set(&self, previous_style: Style);

    fn adjust_baseline_allocation(&self, baseline: &mut i32) {
        self.parent_adjust_baseline_allocation(baseline)
    }

    fn adjust_baseline_request(&self, minimum_baseline: &mut i32, natural_baseline: &mut i32) {
        self.parent_adjust_baseline_request(minimum_baseline, natural_baseline)
    }

    #[doc(alias = "get_preferred_height_and_baseline_for_width")]
    fn preferred_height_and_baseline_for_width(&self, width: i32) -> (i32, i32, i32, i32) {
        self.parent_preferred_height_and_baseline_for_width(width)
    }

    fn adjust_size_allocation(
        &self,

        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
        allocated_pos: &mut i32,
        allocated_size: &mut i32,
    ) {
        self.parent_adjust_size_allocation(
            orientation,
            minimum_size,
            natural_size,
            allocated_pos,
            allocated_size,
        )
    }

    fn adjust_size_request(
        &self,

        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
    ) {
        self.parent_adjust_size_request(orientation, minimum_size, natural_size)
    }

    fn button_press_event(&self, event: &gdk::EventButton) -> Propagation {
        self.parent_button_press_event(event)
    }

    fn button_release_event(&self, event: &gdk::EventButton) -> Propagation {
        self.parent_button_release_event(event)
    }

    fn child_notify(&self, child_property: &glib::ParamSpec) {
        self.parent_child_notify(child_property)
    }

    fn composited_changed(&self) {
        self.parent_composited_changed()
    }

    fn compute_expand(&self, hexpand: &mut bool, vexpand: &mut bool) {
        self.parent_compute_expand(hexpand, vexpand)
    }

    fn configure_event(&self, event: &gdk::EventConfigure) -> Propagation {
        self.parent_configure_event(event)
    }

    fn window_state_event(&self, event: &gdk::EventWindowState) -> Propagation {
        self.parent_window_state_event(event)
    }

    fn damage_event(&self, event: &gdk::EventExpose) -> Propagation {
        self.parent_damage_event(event)
    }

    fn delete_event(&self, event: &gdk::Event) -> Propagation {
        self.parent_delete_event(event)
    }

    fn destroy(&self) {
        self.parent_destroy()
    }

    fn destroy_event(&self, event: &gdk::Event) -> Propagation {
        self.parent_destroy_event(event)
    }

    fn direction_changed(&self, previous_direction: TextDirection) {
        self.parent_direction_changed(previous_direction)
    }

    fn dispatch_child_properties_changed(&self, pspecs: &[glib::ParamSpec]) {
        self.parent_dispatch_child_properties_changed(pspecs)
    }

    fn drag_begin(&self, context: &gdk::DragContext) {
        self.parent_drag_begin(context)
    }

    fn drag_data_delete(&self, context: &gdk::DragContext) {
        self.parent_drag_data_delete(context)
    }

    fn drag_data_get(
        &self,

        context: &gdk::DragContext,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        self.parent_drag_data_get(context, selection_data, info, time)
    }

    fn drag_data_received(
        &self,

        context: &gdk::DragContext,
        x: i32,
        y: i32,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        self.parent_drag_data_received(context, x, y, selection_data, info, time)
    }

    fn drag_drop(&self, context: &gdk::DragContext, x: i32, y: i32, time: u32) -> Propagation {
        self.parent_drag_drop(context, x, y, time)
    }

    fn drag_end(&self, context: &gdk::DragContext) {
        self.parent_drag_end(context)
    }

    fn drag_failed(&self, context: &gdk::DragContext, result: DragResult) -> Propagation {
        self.parent_drag_failed(context, result)
    }

    fn drag_leave(&self, context: &gdk::DragContext, time: u32) {
        self.parent_drag_leave(context, time)
    }

    fn drag_motion(&self, context: &gdk::DragContext, x: i32, y: i32, time: u32) -> Propagation {
        self.parent_drag_motion(context, x, y, time)
    }

    fn draw(&self, cr: &cairo::Context) -> Propagation {
        self.parent_draw(cr)
    }

    fn can_activate_accel(&self, signal_id: SignalId) -> bool {
        self.parent_can_activate_accel(signal_id)
    }

    fn request_mode(&self) -> SizeRequestMode {
        self.parent_request_mode()
    }

    fn preferred_width(&self) -> (i32, i32) {
        self.parent_preferred_width()
    }

    #[doc(alias = "get_preferred_width_for_height")]
    fn preferred_width_for_height(&self, height: i32) -> (i32, i32) {
        self.parent_preferred_width_for_height(height)
    }

    fn preferred_height(&self) -> (i32, i32) {
        self.parent_preferred_height()
    }

    #[doc(alias = "get_preferred_height_for_width")]
    fn preferred_height_for_width(&self, width: i32) -> (i32, i32) {
        self.parent_preferred_height_for_width(width)
    }

    fn size_allocate(&self, allocation: &Allocation) {
        self.parent_size_allocate(allocation)
    }

    fn realize(&self) {
        self.parent_realize();
    }

    fn unrealize(&self) {
        self.parent_unrealize();
    }
    fn map(&self) {
        self.parent_map();
    }

    fn unmap(&self) {
        self.parent_unmap();
    }

    fn show(&self) {
        self.parent_show();
    }

    fn hide(&self) {
        self.parent_hide();
    }

    fn grab_focus(&self) {
        self.parent_grab_focus();
    }

    fn focus(&self, direction: DirectionType) -> bool {
        self.parent_focus(direction)
    }

    fn motion_notify_event(&self, event: &gdk::EventMotion) -> Propagation {
        self.parent_motion_notify_event(event)
    }

    fn scroll_event(&self, event: &gdk::EventScroll) -> Propagation {
        self.parent_scroll_event(event)
    }

    fn enter_notify_event(&self, event: &gdk::EventCrossing) -> Propagation {
        self.parent_enter_notify_event(event)
    }

    fn leave_notify_event(&self, event: &gdk::EventCrossing) -> Propagation {
        self.parent_leave_notify_event(event)
    }

    fn focus_in_event(&self, event: &gdk::EventFocus) -> Propagation {
        self.parent_focus_in_event(event)
    }

    fn focus_out_event(&self, event: &gdk::EventFocus) -> Propagation {
        self.parent_focus_out_event(event)
    }

    fn key_press_event(&self, event: &gdk::EventKey) -> Propagation {
        self.parent_key_press_event(event)
    }

    fn key_release_event(&self, event: &gdk::EventKey) -> Propagation {
        self.parent_key_release_event(event)
    }

    fn map_event(&self, event: &gdk::Event) -> Propagation {
        self.parent_map_event(event)
    }

    fn unmap_event(&self, event: &gdk::Event) -> Propagation {
        self.parent_unmap_event(event)
    }

    fn show_all(&self) {
        self.parent_show_all();
    }

    fn state_flags_changed(&self, previous_state_flags: StateFlags) {
        self.parent_state_flags_changed(previous_state_flags);
    }

    fn parent_set(&self, previous_parent: Option<&Widget>) {
        self.parent_parent_set(previous_parent);
    }

    fn hierarchy_changed(&self, previous_toplevel: Option<&Widget>) {
        self.parent_hierarchy_changed(previous_toplevel);
    }

    fn grab_notify(&self, was_grabbed: bool) {
        self.parent_grab_notify(was_grabbed);
    }

    fn mnemonic_activate(&self, group_cycling: bool) -> bool {
        self.parent_mnemonic_activate(group_cycling)
    }

    fn move_focus(&self, direction: DirectionType) {
        self.parent_move_focus(direction);
    }

    fn keynav_failed(&self, direction: DirectionType) -> bool {
        self.parent_keynav_failed(direction)
    }

    fn event(&self, event: &gdk::Event) -> Propagation {
        self.parent_event(event)
    }

    fn property_notify_event(&self, event: &gdk::EventProperty) -> Propagation {
        self.parent_property_notify_event(event)
    }

    fn selection_clear_event(&self, event: &gdk::EventSelection) -> Propagation {
        self.parent_selection_clear_event(event)
    }

    fn selection_request_event(&self, event: &gdk::EventSelection) -> Propagation {
        self.parent_selection_request_event(event)
    }

    fn selection_notify_event(&self, event: &gdk::EventSelection) -> Propagation {
        self.parent_selection_notify_event(event)
    }

    fn proximity_in_event(&self, event: &gdk::EventProximity) -> Propagation {
        self.parent_proximity_in_event(event)
    }

    fn proximity_out_event(&self, event: &gdk::EventProximity) -> Propagation {
        self.parent_proximity_out_event(event)
    }

    fn visibility_notify_event(&self, event: &gdk::EventVisibility) -> Propagation {
        self.parent_visibility_notify_event(event)
    }

    fn grab_broken_event(&self, event: &gdk::EventGrabBroken) -> Propagation {
        self.parent_grab_broken_event(event)
    }

    fn touch_event(&self, event: &gdk::EventTouch) -> Propagation {
        self.parent_touch_event(event)
    }

    fn selection_get(&self, selection_data: &SelectionData, info: u32, time: u32) {
        self.parent_selection_get(selection_data, info, time);
    }

    fn selection_received(&self, selection_data: &SelectionData, time: u32) {
        self.parent_selection_received(selection_data, time);
    }

    fn popup_menu(&self) -> bool {
        self.parent_popup_menu()
    }

    fn show_help(&self, help_type: WidgetHelpType) -> bool {
        self.parent_show_help(help_type)
    }

    fn screen_changed(&self, previous_screen: Option<&gdk::Screen>) {
        self.parent_screen_changed(previous_screen);
    }

    fn query_tooltip(&self, x: i32, y: i32, keyboard_tooltip: bool, tooltip: &Tooltip) -> bool {
        self.parent_query_tooltip(x, y, keyboard_tooltip, tooltip)
    }

    fn style_updated(&self) {
        self.parent_style_updated();
    }

    fn queue_draw_region(&self, region: &cairo::Region) {
        self.parent_queue_draw_region(region);
    }
}

pub trait WidgetImplExt: WidgetImpl {
    fn parent_adjust_baseline_allocation(&self, baseline: &mut i32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_baseline_allocation
                .expect("No parent class impl for \"adjust_baseline_allocation\"");
            f(
                self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                baseline,
            )
        }
    }
    fn parent_adjust_baseline_request(
        &self,

        minimum_baseline: &mut i32,
        natural_baseline: &mut i32,
    ) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_baseline_request
                .expect("No parent class impl for \"adjust_baseline_request\"");
            f(
                self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                minimum_baseline,
                natural_baseline,
            )
        }
    }

    fn parent_preferred_height_and_baseline_for_width(&self, width: i32) -> (i32, i32, i32, i32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).get_preferred_height_and_baseline_for_width {
                let mut minimum_height = mem::MaybeUninit::uninit();
                let mut natural_height = mem::MaybeUninit::uninit();
                let mut minimum_baseline = -1;
                let mut natural_baseline = -1;
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    width,
                    minimum_height.as_mut_ptr(),
                    natural_height.as_mut_ptr(),
                    &mut minimum_baseline,
                    &mut natural_baseline,
                );
                (
                    minimum_height.assume_init(),
                    natural_height.assume_init(),
                    minimum_baseline,
                    natural_baseline,
                )
            } else {
                // The parent hasn't wired up baseline support, so do the fallback calculation
                // exactly how GTK does it when there's no baseline support. This has to go through
                // `self` and not `parent_*`, because we installed the baseline vfunc, which makes
                // GTK route *every* height query here instead of calling get_preferred_height or
                // get_preferred_height_for_width on our impl, so this is the only way to get the
                // correct result.
                let (minimum, natural) = if width < 0 {
                    self.preferred_height()
                } else {
                    self.preferred_height_for_width(width)
                };
                (minimum, natural, -1, -1)
            }
        }
    }

    fn parent_adjust_size_allocation(
        &self,

        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
        allocated_pos: &mut i32,
        allocated_size: &mut i32,
    ) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_size_allocation
                .expect("No parent class impl for \"adjust_size_allocation\"");
            f(
                self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                orientation.into_glib(),
                minimum_size,
                natural_size,
                allocated_pos,
                allocated_size,
            )
        }
    }
    fn parent_adjust_size_request(
        &self,

        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
    ) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_size_request
                .expect("No parent class impl for \"adjust_size_request\"");
            f(
                self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                orientation.into_glib(),
                minimum_size as *mut i32,
                natural_size as *mut i32,
            )
        }
    }
    fn parent_button_press_event(&self, event: &gdk::EventButton) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).button_press_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_button_release_event(&self, event: &gdk::EventButton) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).button_release_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_can_activate_accel(&self, signal_id: SignalId) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).can_activate_accel {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    signal_id.into_glib(),
                ))
            } else {
                false
            }
        }
    }
    fn parent_child_notify(&self, child_property: &glib::ParamSpec) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).child_notify {
                let pspec_glib = glib::translate::mut_override(child_property.to_glib_none().0);
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    pspec_glib,
                )
            }
        }
    }
    fn parent_composited_changed(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).composited_changed {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }
    fn parent_compute_expand(&self, hexpand: &mut bool, vexpand: &mut bool) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let widget = self.obj();
            let widget = widget.unsafe_cast_ref::<Widget>();
            if let Some(f) = (*parent_class).compute_expand {
                let mut hexpand_glib = hexpand.into_glib();
                let mut vexpand_glib = vexpand.into_glib();
                f(
                    widget.to_glib_none().0,
                    &mut hexpand_glib,
                    &mut vexpand_glib,
                );
                *hexpand = from_glib(hexpand_glib);
                *vexpand = from_glib(vexpand_glib);
            }
        }
    }
    fn parent_configure_event(&self, event: &gdk::EventConfigure) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).configure_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_window_state_event(&self, event: &gdk::EventWindowState) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).window_state_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_damage_event(&self, event: &gdk::EventExpose) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).damage_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_delete_event(&self, event: &gdk::Event) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).delete_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_destroy(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).destroy {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }
    fn parent_destroy_event(&self, event: &gdk::Event) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).destroy_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_direction_changed(&self, previous_direction: TextDirection) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).direction_changed {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    previous_direction.into_glib(),
                )
            }
        }
    }
    fn parent_dispatch_child_properties_changed(&self, pspecs: &[glib::ParamSpec]) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).dispatch_child_properties_changed {
                let mut pspecs_array = pspecs
                    .iter()
                    .map(|p| p.to_glib_none().0)
                    .collect::<Vec<_>>();
                let pspecs_ptr = pspecs_array.as_mut_ptr();
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    pspecs.len() as u32,
                    pspecs_ptr,
                )
            }
        }
    }
    fn parent_drag_begin(&self, context: &gdk::DragContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_begin {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                )
            }
        }
    }
    fn parent_drag_data_delete(&self, context: &gdk::DragContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_data_delete {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                )
            }
        }
    }
    fn parent_drag_data_get(
        &self,

        context: &gdk::DragContext,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_data_get {
                let selection_mut = glib::translate::mut_override(selection_data.to_glib_none().0);
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    selection_mut,
                    info,
                    time,
                )
            }
        }
    }
    fn parent_drag_data_received(
        &self,

        context: &gdk::DragContext,
        x: i32,
        y: i32,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_data_received {
                let selection_mut = glib::translate::mut_override(selection_data.to_glib_none().0);
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    x,
                    y,
                    selection_mut,
                    info,
                    time,
                )
            }
        }
    }
    fn parent_drag_drop(
        &self,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_drop {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    x,
                    y,
                    time,
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_drag_end(&self, context: &gdk::DragContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_end {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                )
            }
        }
    }
    fn parent_drag_failed(&self, context: &gdk::DragContext, result: DragResult) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_failed {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    result.into_glib(),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_drag_leave(&self, context: &gdk::DragContext, time: u32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_leave {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    time,
                )
            }
        }
    }
    fn parent_drag_motion(
        &self,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_motion {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    x,
                    y,
                    time,
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_draw(&self, cr: &cairo::Context) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).draw {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    cr.to_glib_none().0,
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_request_mode(&self) -> SizeRequestMode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class).get_request_mode.unwrap();
            from_glib(f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0))
        }
    }
    fn parent_preferred_width(&self) -> (i32, i32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class).get_preferred_width.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_preferred_width_for_height(&self, height: i32) -> (i32, i32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class).get_preferred_width_for_height.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                height,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_preferred_height(&self) -> (i32, i32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class).get_preferred_height.unwrap();
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_preferred_height_for_width(&self, width: i32) -> (i32, i32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class).get_preferred_height_for_width.unwrap();
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                width,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_size_allocate(&self, allocation: &Allocation) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            let f = (*parent_class)
                .size_allocate
                .expect("No parent class impl for \"size_allocate\"");
            f(
                self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                mut_override(allocation.to_glib_none().0),
            );
        }
    }
    fn parent_realize(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).realize {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }
    fn parent_unrealize(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).unrealize {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }
    fn parent_map(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).map {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }
    fn parent_unmap(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).unmap {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }

    fn parent_show(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).show {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }

    fn parent_hide(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).hide {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }

    fn parent_grab_focus(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).grab_focus {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }

    fn parent_focus(&self, direction: DirectionType) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).focus {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_motion_notify_event(&self, event: &gdk::EventMotion) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).motion_notify_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_scroll_event(&self, event: &gdk::EventScroll) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).scroll_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_enter_notify_event(&self, event: &gdk::EventCrossing) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).enter_notify_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }
    fn parent_leave_notify_event(&self, event: &gdk::EventCrossing) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).leave_notify_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_focus_in_event(&self, event: &gdk::EventFocus) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).focus_in_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_focus_out_event(&self, event: &gdk::EventFocus) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).focus_out_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_key_press_event(&self, event: &gdk::EventKey) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).key_press_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_key_release_event(&self, event: &gdk::EventKey) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).key_release_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_map_event(&self, event: &gdk::Event) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).map_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_unmap_event(&self, event: &gdk::Event) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).unmap_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_show_all(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).show_all {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }

    fn parent_state_flags_changed(&self, previous_state_flags: StateFlags) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).state_flags_changed {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    previous_state_flags.into_glib(),
                );
            }
        }
    }

    fn parent_parent_set(&self, previous_parent: Option<&Widget>) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).parent_set {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    previous_parent.to_glib_none().0,
                );
            }
        }
    }

    fn parent_hierarchy_changed(&self, previous_toplevel: Option<&Widget>) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).hierarchy_changed {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    previous_toplevel.to_glib_none().0,
                );
            }
        }
    }

    fn parent_grab_notify(&self, was_grabbed: bool) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).grab_notify {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    was_grabbed.into_glib(),
                );
            }
        }
    }

    fn parent_mnemonic_activate(&self, group_cycling: bool) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).mnemonic_activate {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    group_cycling.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_move_focus(&self, direction: DirectionType) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).move_focus {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction.into_glib(),
                );
            }
        }
    }

    fn parent_keynav_failed(&self, direction: DirectionType) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).keynav_failed {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    direction.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_event(&self, event: &gdk::Event) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_property_notify_event(&self, event: &gdk::EventProperty) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).property_notify_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_selection_clear_event(&self, event: &gdk::EventSelection) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).selection_clear_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_selection_request_event(&self, event: &gdk::EventSelection) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).selection_request_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_selection_notify_event(&self, event: &gdk::EventSelection) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).selection_notify_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_proximity_in_event(&self, event: &gdk::EventProximity) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).proximity_in_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_proximity_out_event(&self, event: &gdk::EventProximity) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).proximity_out_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_visibility_notify_event(&self, event: &gdk::EventVisibility) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).visibility_notify_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_grab_broken_event(&self, event: &gdk::EventGrabBroken) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).grab_broken_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_touch_event(&self, event: &gdk::EventTouch) -> Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).touch_event {
                Propagation::from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                ))
            } else {
                Propagation::Proceed
            }
        }
    }

    fn parent_selection_get(&self, selection_data: &SelectionData, info: u32, time: u32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).selection_get {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(selection_data.to_glib_none().0),
                    info,
                    time,
                );
            }
        }
    }

    fn parent_selection_received(&self, selection_data: &SelectionData, time: u32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).selection_received {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(selection_data.to_glib_none().0),
                    time,
                );
            }
        }
    }

    fn parent_popup_menu(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).popup_menu {
                from_glib(f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_show_help(&self, help_type: WidgetHelpType) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).show_help {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    help_type.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_screen_changed(&self, previous_screen: Option<&gdk::Screen>) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).screen_changed {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    previous_screen.to_glib_none().0,
                );
            }
        }
    }

    fn parent_query_tooltip(
        &self,
        x: i32,
        y: i32,
        keyboard_tooltip: bool,
        tooltip: &Tooltip,
    ) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).query_tooltip {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    x,
                    y,
                    keyboard_tooltip.into_glib(),
                    tooltip.to_glib_none().0,
                ))
            } else {
                false
            }
        }
    }

    fn parent_style_updated(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).style_updated {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }

    fn parent_queue_draw_region(&self, region: &cairo::Region) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).queue_draw_region {
                f(
                    self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0,
                    region.to_glib_none().0,
                );
            }
        }
    }
}

impl<T: WidgetImpl> WidgetImplExt for T {}

unsafe impl<T: WidgetImpl> IsSubclassable<T> for Widget {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        let klass = class.as_mut();
        klass.adjust_baseline_allocation = Some(widget_adjust_baseline_allocation::<T>);
        klass.adjust_baseline_request = Some(widget_adjust_baseline_request::<T>);
        klass.adjust_size_allocation = Some(widget_adjust_size_allocation::<T>);
        klass.adjust_size_request = Some(widget_adjust_size_request::<T>);
        klass.button_press_event = Some(widget_button_press_event::<T>);
        klass.button_release_event = Some(widget_button_release_event::<T>);
        klass.can_activate_accel = Some(widget_can_activate_accel::<T>);
        klass.child_notify = Some(widget_child_notify::<T>);
        klass.composited_changed = Some(widget_composited_changed::<T>);
        klass.compute_expand = Some(widget_compute_expand::<T>);
        klass.configure_event = Some(widget_configure_event::<T>);
        klass.damage_event = Some(widget_damage_event::<T>);
        klass.delete_event = Some(widget_delete_event::<T>);
        klass.destroy = Some(widget_destroy::<T>);
        klass.destroy_event = Some(widget_destroy_event::<T>);
        klass.direction_changed = Some(widget_direction_changed::<T>);
        klass.dispatch_child_properties_changed =
            Some(widget_dispatch_child_properties_changed::<T>);
        klass.drag_begin = Some(widget_drag_begin::<T>);
        klass.drag_data_delete = Some(widget_drag_data_delete::<T>);
        klass.drag_data_get = Some(widget_drag_data_get::<T>);
        klass.drag_data_received = Some(widget_drag_data_received::<T>);
        klass.drag_drop = Some(widget_drag_drop::<T>);
        klass.drag_end = Some(widget_drag_end::<T>);
        klass.drag_failed = Some(widget_drag_failed::<T>);
        klass.drag_leave = Some(widget_drag_leave::<T>);
        klass.drag_motion = Some(widget_drag_motion::<T>);
        klass.draw = Some(widget_draw::<T>);
        klass.enter_notify_event = Some(widget_enter_notify_event::<T>);
        klass.event = Some(widget_event::<T>);
        klass.focus = Some(widget_focus::<T>);
        klass.focus_in_event = Some(widget_focus_in_event::<T>);
        klass.focus_out_event = Some(widget_focus_out_event::<T>);
        klass.get_preferred_height = Some(widget_get_preferred_height::<T>);
        klass.get_preferred_height_for_width = Some(widget_get_preferred_height_for_width::<T>);
        klass.get_preferred_width = Some(widget_get_preferred_width::<T>);
        klass.get_preferred_width_for_height = Some(widget_get_preferred_width_for_height::<T>);
        klass.get_request_mode = Some(widget_get_request_mode::<T>);
        klass.grab_broken_event = Some(widget_grab_broken_event::<T>);
        klass.grab_focus = Some(widget_grab_focus::<T>);
        klass.grab_notify = Some(widget_grab_notify::<T>);
        klass.hide = Some(widget_hide::<T>);
        klass.hierarchy_changed = Some(widget_hierarchy_changed::<T>);
        klass.key_press_event = Some(widget_key_press_event::<T>);
        klass.key_release_event = Some(widget_key_release_event::<T>);
        klass.keynav_failed = Some(widget_keynav_failed::<T>);
        klass.leave_notify_event = Some(widget_leave_notify_event::<T>);
        klass.map = Some(widget_map::<T>);
        klass.map_event = Some(widget_map_event::<T>);
        klass.mnemonic_activate = Some(widget_mnemonic_activate::<T>);
        klass.motion_notify_event = Some(widget_motion_notify_event::<T>);
        klass.move_focus = Some(widget_move_focus::<T>);
        klass.parent_set = Some(widget_parent_set::<T>);
        klass.popup_menu = Some(widget_popup_menu::<T>);
        klass.property_notify_event = Some(widget_property_notify_event::<T>);
        klass.proximity_in_event = Some(widget_proximity_in_event::<T>);
        klass.proximity_out_event = Some(widget_proximity_out_event::<T>);
        klass.query_tooltip = Some(widget_query_tooltip::<T>);
        klass.queue_draw_region = Some(widget_queue_draw_region::<T>);
        klass.realize = Some(widget_realize::<T>);
        klass.screen_changed = Some(widget_screen_changed::<T>);
        klass.scroll_event = Some(widget_scroll_event::<T>);
        klass.selection_clear_event = Some(widget_selection_clear_event::<T>);
        klass.selection_get = Some(widget_selection_get::<T>);
        klass.selection_notify_event = Some(widget_selection_notify_event::<T>);
        klass.selection_received = Some(widget_selection_received::<T>);
        klass.selection_request_event = Some(widget_selection_request_event::<T>);
        klass.show = Some(widget_show::<T>);
        klass.show_all = Some(widget_show_all::<T>);
        klass.show_help = Some(widget_show_help::<T>);
        klass.size_allocate = Some(widget_size_allocate::<T>);
        klass.state_flags_changed = Some(widget_state_flags_changed::<T>);
        klass.style_updated = Some(widget_style_updated::<T>);
        klass.touch_event = Some(widget_touch_event::<T>);
        klass.unmap = Some(widget_unmap::<T>);
        klass.unmap_event = Some(widget_unmap_event::<T>);
        klass.unrealize = Some(widget_unrealize::<T>);
        klass.visibility_notify_event = Some(widget_visibility_notify_event::<T>);
        klass.window_state_event = Some(widget_window_state_event::<T>);
    }
}

unsafe extern "C" fn widget_adjust_baseline_allocation<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    baseptr: *mut i32,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        imp.adjust_baseline_allocation(&mut *baseptr)
    }
}

unsafe extern "C" fn widget_adjust_baseline_request<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    minptr: *mut i32,
    natptr: *mut i32,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        imp.adjust_baseline_request(&mut *minptr, &mut *natptr)
    }
}

unsafe extern "C" fn widget_adjust_size_allocation<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    orientation: ffi::GtkOrientation,
    minptr: *mut i32,
    natptr: *mut i32,
    posptr: *mut i32,
    sizeptr: *mut i32,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let wrap_orientation: Orientation = from_glib(orientation);

        imp.adjust_size_allocation(
            wrap_orientation,
            &mut *minptr,
            &mut *natptr,
            &mut *posptr,
            &mut *sizeptr,
        )
    }
}

unsafe extern "C" fn widget_adjust_size_request<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    orientation: ffi::GtkOrientation,
    minptr: *mut i32,
    natptr: *mut i32,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let wrap_orientation: Orientation = from_glib(orientation);

        imp.adjust_size_request(wrap_orientation, &mut *minptr, &mut *natptr)
    }
}

unsafe extern "C" fn widget_button_press_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    btnptr: *mut gdk::ffi::GdkEventButton,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let evwrap: Borrowed<gdk::EventButton> = from_glib_borrow(btnptr);

        imp.button_press_event(&evwrap).into_glib()
    }
}

unsafe extern "C" fn widget_button_release_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    btnptr: *mut gdk::ffi::GdkEventButton,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let evwrap: Borrowed<gdk::EventButton> = from_glib_borrow(btnptr);

        imp.button_release_event(&evwrap).into_glib()
    }
}

unsafe extern "C" fn widget_can_activate_accel<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    signal_id: u32,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        if let Some(signal_id) = NonZeroU32::new(signal_id).map(|nz| SignalId::new(nz)) {
            imp.can_activate_accel(signal_id).into_glib()
        } else {
            // 0 is invalid, so there is nothing to hand the impl; defer to GTK, where at the
            // bottom of the chain `gtk_widget_real_can_activate_accel()` doesn't even look at
            // the signal ID at all, and instead returns something based on the widget's state.
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWidgetClass;
            if let Some(f) = (*parent_class).can_activate_accel {
                f(ptr, signal_id)
            } else {
                false.into_glib()
            }
        }
    }
}

unsafe extern "C" fn widget_child_notify<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    paramptr: *mut glib::gobject_ffi::GParamSpec,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let paramwrap: Borrowed<glib::ParamSpec> = from_glib_borrow(paramptr);

        imp.child_notify(&paramwrap)
    }
}

unsafe extern "C" fn widget_composited_changed<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        imp.composited_changed()
    }
}

unsafe extern "C" fn widget_compute_expand<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    hexpand_ptr: *mut glib::ffi::gboolean,
    vexpand_ptr: *mut glib::ffi::gboolean,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        let widget = imp.obj();
        let widget = widget.unsafe_cast_ref::<Widget>();
        let mut hexpand: bool = if widget.is_hexpand_set() {
            widget.hexpands()
        } else {
            from_glib(*hexpand_ptr)
        };
        let mut vexpand: bool = if widget.is_vexpand_set() {
            widget.vexpands()
        } else {
            from_glib(*vexpand_ptr)
        };

        imp.compute_expand(&mut hexpand, &mut vexpand);
        *hexpand_ptr = hexpand.into_glib();
        *vexpand_ptr = vexpand.into_glib();
    }
}

unsafe extern "C" fn widget_configure_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    confptr: *mut gdk::ffi::GdkEventConfigure,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let evwrap: Borrowed<gdk::EventConfigure> = from_glib_borrow(confptr);

        imp.configure_event(&evwrap).into_glib()
    }
}

unsafe extern "C" fn widget_window_state_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    winstateptr: *mut gdk::ffi::GdkEventWindowState,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let evwrap: Borrowed<gdk::EventWindowState> = from_glib_borrow(winstateptr);

        imp.window_state_event(&evwrap).into_glib()
    }
}

unsafe extern "C" fn widget_damage_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    exposeptr: *mut gdk::ffi::GdkEventExpose,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let evwrap: Borrowed<gdk::EventExpose> = from_glib_borrow(exposeptr);

        imp.damage_event(&evwrap).into_glib()
    }
}

unsafe extern "C" fn widget_delete_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    anyptr: *mut gdk::ffi::GdkEventAny,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let evwrap: Borrowed<gdk::Event> = from_glib_borrow(anyptr);

        imp.delete_event(&evwrap).into_glib()
    }
}

unsafe extern "C" fn widget_destroy<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        imp.destroy()
    }
}

unsafe extern "C" fn widget_destroy_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    anyptr: *mut gdk::ffi::GdkEventAny,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let evwrap: Borrowed<gdk::Event> = from_glib_borrow(anyptr);

        imp.destroy_event(&evwrap).into_glib()
    }
}

unsafe extern "C" fn widget_direction_changed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    directnptr: ffi::GtkTextDirection,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let dirwrap: TextDirection = from_glib(directnptr);

        imp.direction_changed(dirwrap)
    }
}

unsafe extern "C" fn widget_dispatch_child_properties_changed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    n_pspec_ptr: u32,
    pspecsptr: *mut *mut glib::gobject_ffi::GParamSpec,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let pspecs: Vec<glib::ParamSpec> =
            FromGlibContainer::from_glib_none_num(pspecsptr, n_pspec_ptr as usize);

        imp.dispatch_child_properties_changed(&pspecs)
    }
}

unsafe extern "C" fn widget_drag_begin<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    ctxptr: *mut gdk::ffi::GdkDragContext,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

        imp.drag_begin(&context)
    }
}

unsafe extern "C" fn widget_drag_data_delete<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    ctxptr: *mut gdk::ffi::GdkDragContext,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

        imp.drag_data_delete(&context)
    }
}

unsafe extern "C" fn widget_drag_data_get<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    ctxptr: *mut gdk::ffi::GdkDragContext,
    selectptr: *mut ffi::GtkSelectionData,
    info: u32,
    time: u32,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);
        let selection_data: Borrowed<SelectionData> = from_glib_borrow(selectptr);

        imp.drag_data_get(&context, &selection_data, info, time)
    }
}

unsafe extern "C" fn widget_drag_data_received<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    ctxptr: *mut gdk::ffi::GdkDragContext,
    x: i32,
    y: i32,
    selectptr: *mut ffi::GtkSelectionData,
    info: u32,
    time: u32,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);
        let selection_data: Borrowed<SelectionData> = from_glib_borrow(selectptr);

        imp.drag_data_received(&context, x, y, &selection_data, info, time)
    }
}

unsafe extern "C" fn widget_drag_drop<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    ctxptr: *mut gdk::ffi::GdkDragContext,
    x: i32,
    y: i32,
    time: u32,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

        imp.drag_drop(&context, x, y, time).into_glib()
    }
}

unsafe extern "C" fn widget_drag_end<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    ctxptr: *mut gdk::ffi::GdkDragContext,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

        imp.drag_end(&context)
    }
}

unsafe extern "C" fn widget_drag_failed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    ctxptr: *mut gdk::ffi::GdkDragContext,
    resultptr: ffi::GtkDragResult,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);
        let result: DragResult = from_glib(resultptr);

        imp.drag_failed(&context, result).into_glib()
    }
}

unsafe extern "C" fn widget_drag_leave<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    ctxptr: *mut gdk::ffi::GdkDragContext,
    time: u32,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

        imp.drag_leave(&context, time)
    }
}

unsafe extern "C" fn widget_drag_motion<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    ctxptr: *mut gdk::ffi::GdkDragContext,
    x: i32,
    y: i32,
    time: u32,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

        imp.drag_motion(&context, x, y, time).into_glib()
    }
}

unsafe extern "C" fn widget_draw<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    cr_ptr: *mut cairo::ffi::cairo_t,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let cr: Borrowed<cairo::Context> = from_glib_borrow(cr_ptr);

        imp.draw(&cr).into_glib()
    }
}

unsafe extern "C" fn widget_get_request_mode<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
) -> ffi::GtkSizeRequestMode {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        imp.request_mode().into_glib()
    }
}

unsafe extern "C" fn widget_get_preferred_height<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    minptr: *mut c_int,
    natptr: *mut c_int,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        let (min_size, nat_size) = imp.preferred_height();
        if !minptr.is_null() {
            *minptr = min_size;
        }
        if !natptr.is_null() {
            *natptr = nat_size;
        }
    }
}

unsafe extern "C" fn widget_get_preferred_width_for_height<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    height: c_int,
    min_width_ptr: *mut c_int,
    nat_width_ptr: *mut c_int,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        let (min_width, nat_width) = imp.preferred_width_for_height(height);
        if !min_width_ptr.is_null() {
            *min_width_ptr = min_width;
        }
        if !nat_width_ptr.is_null() {
            *nat_width_ptr = nat_width;
        }
    }
}

unsafe extern "C" fn widget_get_preferred_width<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    minptr: *mut c_int,
    natptr: *mut c_int,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let (min_size, nat_size) = imp.preferred_width();
        if !minptr.is_null() {
            *minptr = min_size;
        }
        if !natptr.is_null() {
            *natptr = nat_size;
        }
    }
}

unsafe extern "C" fn widget_get_preferred_height_for_width<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    width: c_int,
    min_height_ptr: *mut c_int,
    nat_height_ptr: *mut c_int,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        let (min_height, nat_height) = imp.preferred_height_for_width(width);
        if !min_height_ptr.is_null() {
            *min_height_ptr = min_height;
        }
        if !nat_height_ptr.is_null() {
            *nat_height_ptr = nat_height;
        }
    }
}

unsafe extern "C" fn widget_get_preferred_height_and_baseline_for_width<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    width: c_int,
    min_height_ptr: *mut c_int,
    nat_height_ptr: *mut c_int,
    min_baseline_ptr: *mut c_int,
    nat_baseline_ptr: *mut c_int,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        let (min_height, nat_height, min_baseline, nat_baseline) =
            imp.preferred_height_and_baseline_for_width(width);
        if !min_height_ptr.is_null() {
            *min_height_ptr = min_height;
        }
        if !nat_height_ptr.is_null() {
            *nat_height_ptr = nat_height;
        }
        if !min_baseline_ptr.is_null() {
            *min_baseline_ptr = min_baseline;
        }
        if !nat_baseline_ptr.is_null() {
            *nat_baseline_ptr = nat_baseline;
        }
    }
}

unsafe extern "C" fn widget_size_allocate<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    allocation: *mut ffi::GtkAllocation,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let allocate: &Allocation = &from_glib_none(allocation);

        imp.size_allocate(allocate);
    }
}

unsafe extern "C" fn widget_realize<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        imp.realize();
    }
}

unsafe extern "C" fn widget_unrealize<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        imp.unrealize();
    }
}

unsafe extern "C" fn widget_map<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.map();
    }
}

unsafe extern "C" fn widget_unmap<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.unmap();
    }
}

unsafe extern "C" fn widget_show<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.show();
    }
}

unsafe extern "C" fn widget_hide<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.hide();
    }
}

unsafe extern "C" fn widget_grab_focus<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.grab_focus();
    }
}

unsafe extern "C" fn widget_focus<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    direction: ffi::GtkDirectionType,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.focus(from_glib(direction)).into_glib()
    }
}

unsafe extern "C" fn widget_motion_notify_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    mptr: *mut gdk::ffi::GdkEventMotion,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventMotion> = from_glib_borrow(mptr);

        imp.motion_notify_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_scroll_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    mptr: *mut gdk::ffi::GdkEventScroll,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventScroll> = from_glib_borrow(mptr);

        imp.scroll_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_enter_notify_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    mptr: *mut gdk::ffi::GdkEventCrossing,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventCrossing> = from_glib_borrow(mptr);

        imp.enter_notify_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_leave_notify_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    mptr: *mut gdk::ffi::GdkEventCrossing,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventCrossing> = from_glib_borrow(mptr);

        imp.leave_notify_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_focus_in_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    mptr: *mut gdk::ffi::GdkEventFocus,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventFocus> = from_glib_borrow(mptr);

        imp.focus_in_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_focus_out_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    mptr: *mut gdk::ffi::GdkEventFocus,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventFocus> = from_glib_borrow(mptr);

        imp.focus_out_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_key_press_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    mptr: *mut gdk::ffi::GdkEventKey,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventKey> = from_glib_borrow(mptr);

        imp.key_press_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_key_release_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    mptr: *mut gdk::ffi::GdkEventKey,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventKey> = from_glib_borrow(mptr);

        imp.key_release_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_map_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    anyptr: *mut gdk::ffi::GdkEventAny,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let evwrap: Borrowed<gdk::Event> = from_glib_borrow(anyptr);

        imp.map_event(&evwrap).into_glib()
    }
}

unsafe extern "C" fn widget_unmap_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    anyptr: *mut gdk::ffi::GdkEventAny,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let evwrap: Borrowed<gdk::Event> = from_glib_borrow(anyptr);

        imp.unmap_event(&evwrap).into_glib()
    }
}

unsafe extern "C" fn widget_show_all<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.show_all();
    }
}

unsafe extern "C" fn widget_style_updated<T: WidgetImpl>(ptr: *mut ffi::GtkWidget) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.style_updated();
    }
}

unsafe extern "C" fn widget_state_flags_changed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    flags: ffi::GtkStateFlags,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.state_flags_changed(from_glib(flags));
    }
}

unsafe extern "C" fn widget_parent_set<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    parentptr: *mut ffi::GtkWidget,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let previous_parent: Borrowed<Option<Widget>> = from_glib_borrow(parentptr);

        imp.parent_set(previous_parent.as_ref().as_ref());
    }
}

unsafe extern "C" fn widget_hierarchy_changed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    toplevelptr: *mut ffi::GtkWidget,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let previous_toplevel: Borrowed<Option<Widget>> = from_glib_borrow(toplevelptr);

        imp.hierarchy_changed(previous_toplevel.as_ref().as_ref());
    }
}

unsafe extern "C" fn widget_grab_notify<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    was_grabbed: glib::ffi::gboolean,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.grab_notify(from_glib(was_grabbed));
    }
}

unsafe extern "C" fn widget_mnemonic_activate<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    group_cycling: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.mnemonic_activate(from_glib(group_cycling)).into_glib()
    }
}

unsafe extern "C" fn widget_move_focus<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    direction: ffi::GtkDirectionType,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.move_focus(from_glib(direction));
    }
}

unsafe extern "C" fn widget_keynav_failed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    direction: ffi::GtkDirectionType,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.keynav_failed(from_glib(direction)).into_glib()
    }
}

unsafe extern "C" fn widget_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    evptr: *mut gdk::ffi::GdkEvent,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::Event> = from_glib_borrow(evptr);

        imp.event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_property_notify_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    evptr: *mut gdk::ffi::GdkEventProperty,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventProperty> = from_glib_borrow(evptr);

        imp.property_notify_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_selection_clear_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    evptr: *mut gdk::ffi::GdkEventSelection,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventSelection> = from_glib_borrow(evptr);

        imp.selection_clear_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_selection_request_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    evptr: *mut gdk::ffi::GdkEventSelection,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventSelection> = from_glib_borrow(evptr);

        imp.selection_request_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_selection_notify_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    evptr: *mut gdk::ffi::GdkEventSelection,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventSelection> = from_glib_borrow(evptr);

        imp.selection_notify_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_proximity_in_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    evptr: *mut gdk::ffi::GdkEventProximity,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventProximity> = from_glib_borrow(evptr);

        imp.proximity_in_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_proximity_out_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    evptr: *mut gdk::ffi::GdkEventProximity,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventProximity> = from_glib_borrow(evptr);

        imp.proximity_out_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_visibility_notify_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    evptr: *mut gdk::ffi::GdkEventVisibility,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventVisibility> = from_glib_borrow(evptr);

        imp.visibility_notify_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_grab_broken_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    evptr: *mut gdk::ffi::GdkEventGrabBroken,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventGrabBroken> = from_glib_borrow(evptr);

        imp.grab_broken_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_touch_event<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    evptr: *mut gdk::ffi::GdkEventTouch,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let event: Borrowed<gdk::EventTouch> = from_glib_borrow(evptr);

        imp.touch_event(&event).into_glib()
    }
}

unsafe extern "C" fn widget_selection_get<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    selectptr: *mut ffi::GtkSelectionData,
    info: u32,
    time: u32,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let selection_data: Borrowed<SelectionData> = from_glib_borrow(selectptr);

        imp.selection_get(&selection_data, info, time);
    }
}

unsafe extern "C" fn widget_selection_received<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    selectptr: *mut ffi::GtkSelectionData,
    time: u32,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let selection_data: Borrowed<SelectionData> = from_glib_borrow(selectptr);

        imp.selection_received(&selection_data, time);
    }
}

unsafe extern "C" fn widget_popup_menu<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.popup_menu().into_glib()
    }
}

unsafe extern "C" fn widget_show_help<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    help_type: ffi::GtkWidgetHelpType,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.show_help(from_glib(help_type)).into_glib()
    }
}

unsafe extern "C" fn widget_screen_changed<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    screenptr: *mut gdk::ffi::GdkScreen,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let previous_screen: Borrowed<Option<gdk::Screen>> = from_glib_borrow(screenptr);

        imp.screen_changed(previous_screen.as_ref().as_ref());
    }
}

unsafe extern "C" fn widget_query_tooltip<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    x: c_int,
    y: c_int,
    keyboard_tooltip: glib::ffi::gboolean,
    tooltipptr: *mut ffi::GtkTooltip,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let tooltip: Borrowed<Tooltip> = from_glib_borrow(tooltipptr);

        imp.query_tooltip(x, y, from_glib(keyboard_tooltip), &tooltip)
            .into_glib()
    }
}

unsafe extern "C" fn widget_queue_draw_region<T: WidgetImpl>(
    ptr: *mut ffi::GtkWidget,
    regionptr: *const cairo::ffi::cairo_region_t,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let region: Borrowed<cairo::Region> = from_glib_borrow(mut_override(regionptr));

        imp.queue_draw_region(&region);
    }
}

pub unsafe trait WidgetClassSubclassExt: ClassStruct<Type: WidgetImpl> {
    fn set_accessible_type(&mut self, type_: glib::Type) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_accessible_type(widget_class, type_.into_glib());
        }
    }

    fn set_accessible_role(&mut self, role: atk::Role) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_accessible_role(widget_class, role.into_glib());
        }
    }

    fn set_template_bytes(&mut self, template: &glib::Bytes) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_template(widget_class, template.to_glib_none().0);
        }
    }

    // rustdoc-stripper-ignore-next
    /// Enables baseline support for this widget class.
    ///
    /// You call this for two reasons:
    ///
    /// 1. You want your widget subclass to implement baseline support, in which case you must
    ///    implement [`WidgetImpl::preferred_height_and_baseline_for_width`].  You should *not*
    ///    implement [`WidgetImpl::preferred_height`] or
    ///    [`WidgetImpl::preferred_height_for_width`], as they will never be called.
    /// 2. You are subclassing a widget that implements baselines, and you want the parent's
    ///    baselines to continue working.  In that case you should call this function (as the
    ///    baseline-enabled state is *not* inherited), but do not implement the
    ///    height/height-for-width trait items yourself.
    ///
    /// If you want to control sizing yourself, without baseline support, do not call this, and
    /// just implement [`WidgetImpl::preferred_height`] and
    /// [`WidgetImpl::preferred_height_for_width`].
    fn enable_baseline_support(&mut self) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            (*widget_class).get_preferred_height_and_baseline_for_width =
                Some(widget_get_preferred_height_and_baseline_for_width::<Self::Type>);
        }
    }

    fn set_template(&mut self, template: &[u8]) {
        let template_bytes = glib::Bytes::from(template);
        self.set_template_bytes(&template_bytes);
    }

    fn set_template_static(&mut self, template: &'static [u8]) {
        let template_bytes = glib::Bytes::from_static(template);
        self.set_template_bytes(&template_bytes);
    }

    fn set_template_from_resource(&mut self, resource_name: &str) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_template_from_resource(
                widget_class,
                resource_name.to_glib_none().0,
            );
        }
    }

    fn bind_template_child(&mut self, name: &str) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_bind_template_child_full(
                widget_class,
                name.to_glib_none().0,
                false as glib::ffi::gboolean,
                0,
            );
        }
    }

    #[allow(clippy::missing_safety_doc)]
    unsafe fn bind_template_child_with_offset<T>(
        &mut self,
        name: &str,
        offset: field_offset::FieldOffset<Self::Type, TemplateChild<T>>,
    ) where
        T: ObjectType + FromGlibPtrNone<*mut <T as ObjectType>::GlibType>,
    {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            let private_offset = <Self::Type as ObjectSubclassType>::type_data()
                .as_ref()
                .impl_offset();
            ffi::gtk_widget_class_bind_template_child_full(
                widget_class,
                name.to_glib_none().0,
                false as glib::ffi::gboolean,
                private_offset + (offset.get_byte_offset() as isize),
            )
        }
    }

    #[doc(alias = "gtk_widget_class_set_css_name")]
    fn set_css_name(&mut self, name: &str) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_set_css_name(widget_class, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_widget_class_get_css_name")]
    fn css_name(&self) -> glib::GString {
        unsafe {
            let widget_class = self as *const _ as *mut ffi::GtkWidgetClass;
            from_glib_none(ffi::gtk_widget_class_get_css_name(widget_class))
        }
    }
}

unsafe impl<T: ClassStruct> WidgetClassSubclassExt for T where T::Type: WidgetImpl {}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct TemplateChild<T>
where
    T: ObjectType + FromGlibPtrNone<*mut <T as ObjectType>::GlibType>,
{
    ptr: *mut <T as ObjectType>::GlibType,
}

impl<T> Default for TemplateChild<T>
where
    T: ObjectType + FromGlibPtrNone<*mut <T as ObjectType>::GlibType>,
{
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }
}

impl<T> std::ops::Deref for TemplateChild<T>
where
    T: ObjectType + FromGlibPtrNone<*mut <T as ObjectType>::GlibType>,
{
    type Target = T;

    // rustdoc-stripper-ignore-next
    /// # Safety
    ///
    /// Since the template child may not be properly bound,
    /// this cast is potentially dangerous if, for example,
    /// the template child isn't bound or is of the wrong type.
    /// The caller is responsible for ensuring that the template
    /// child is bound and of the right type.
    fn deref(&self) -> &Self::Target {
        unsafe {
            assert!(!self.ptr.is_null());
            &*(&self.ptr as *const _ as *const T)
        }
    }
}

impl<T> TemplateChild<T>
where
    T: ObjectType + FromGlibPtrNone<*mut <T as ObjectType>::GlibType>,
{
    #[track_caller]
    pub fn get(&self) -> T {
        unsafe {
            Option::<T>::from_glib_none(self.ptr)
                .expect("Failed to retrieve template child. Please check that it has been bound.")
        }
    }
}

pub trait CompositeTemplate: WidgetImpl {
    fn bind_template(klass: &mut Self::Class);
}
