// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::{DragAction, Event, ModifierType};
use glib::ffi::gboolean;
use glib::signal::{connect_raw, Inhibit, SignalHandlerId};
use glib::translate::*;
use glib::Continue;
use std::mem::transmute;
use std::ptr;

use crate::prelude::*;
use crate::{DestDefaults, Rectangle, TargetEntry, Widget};

pub struct TickCallbackId {
    id: u32,
    widget: glib::WeakRef<Widget>,
}

impl TickCallbackId {
    #[doc(alias = "gtk_widget_remove_tick_callback")]
    pub fn remove(self) {
        if let Some(widget) = self.widget.upgrade() {
            unsafe {
                ffi::gtk_widget_remove_tick_callback(widget.to_glib_none().0, self.id);
            }
        }
    }
}

pub trait WidgetExtManual: 'static {
    /// Sets a widget as a potential drop destination, and adds default behaviors.
    ///
    /// The default behaviors listed in `flags` have an effect similar
    /// to installing default handlers for the widget’s drag-and-drop signals
    /// (`GtkWidget::::drag-motion`, `GtkWidget::::drag-drop`, ...). They all exist
    /// for convenience. When passing GTK_DEST_DEFAULT_ALL for instance it is
    /// sufficient to connect to the widget’s `GtkWidget::::drag-data-received`
    /// signal to get primitive, but consistent drag-and-drop support.
    ///
    /// Things become more complicated when you try to preview the dragged data,
    /// as described in the documentation for `GtkWidget::::drag-motion`. The default
    /// behaviors described by `flags` make some assumptions, that can conflict
    /// with your own signal handlers. For instance GTK_DEST_DEFAULT_DROP causes
    /// invokations of `gdk_drag_status` in the context of `GtkWidget::::drag-motion`,
    /// and invokations of `gtk_drag_finish` in `GtkWidget::::drag-data-received`.
    /// Especially the later is dramatic, when your own `GtkWidget::::drag-motion`
    /// handler calls [WidgetExt::drag_get_data](crate::prelude::WidgetExt::drag_get_data) to inspect the dragged data.
    ///
    /// There’s no way to set a default action here, you can use the
    /// `GtkWidget::::drag-motion` callback for that. Here’s an example which selects
    /// the action to use depending on whether the control key is pressed or not:
    ///
    /// ```C
    /// static void
    /// drag_motion (GtkWidget *widget,
    ///              GdkDragContext *context,
    ///              gint x,
    ///              gint y,
    ///              guint time)
    /// {
    ///   GdkModifierType mask;
    ///
    ///   gdk_window_get_pointer (gtk_widget_get_window (widget),
    ///                           NULL, NULL, &mask);
    ///   if (mask & GDK_CONTROL_MASK)
    ///     gdk_drag_status (context, GDK_ACTION_COPY, time);
    ///   else
    ///     gdk_drag_status (context, GDK_ACTION_MOVE, time);
    /// }
    /// ```
    /// ## `flags`
    /// which types of default drag behavior to use
    /// ## `targets`
    /// a pointer to an array of
    ///  GtkTargetEntrys indicating the drop types that this `self` will
    ///  accept, or [`None`]. Later you can access the list with
    ///  [WidgetExt::drag_dest_get_target_list](crate::prelude::WidgetExt::drag_dest_get_target_list) and [WidgetExt::drag_dest_find_target](crate::prelude::WidgetExt::drag_dest_find_target).
    /// ## `actions`
    /// a bitmask of possible actions for a drop onto this `self`.
    #[doc(alias = "gtk_drag_dest_set")]
    fn drag_dest_set(&self, flags: DestDefaults, targets: &[TargetEntry], actions: DragAction);

    /// Sets up a widget so that GTK+ will start a drag operation when the user
    /// clicks and drags on the widget. The widget must have a window.
    /// ## `start_button_mask`
    /// the bitmask of buttons that can start the drag
    /// ## `targets`
    /// the table of targets
    ///  that the drag will support, may be [`None`]
    /// ## `actions`
    /// the bitmask of possible actions for a drag from this widget
    #[doc(alias = "gtk_drag_source_set")]
    fn drag_source_set(
        &self,
        start_button_mask: ModifierType,
        targets: &[TargetEntry],
        actions: DragAction,
    );

    /// Computes the intersection of a `self`’s area and `area`, storing
    /// the intersection in `intersection`, and returns [`true`] if there was
    /// an intersection. `intersection` may be [`None`] if you’re only
    /// interested in whether there was an intersection.
    /// ## `area`
    /// a rectangle
    ///
    /// # Returns
    ///
    /// [`true`] if there was an intersection
    ///
    /// ## `intersection`
    /// rectangle to store
    ///  intersection of `self` and `area`
    #[doc(alias = "gtk_widget_intersect")]
    fn intersect(&self, area: &Rectangle, intersection: Option<&mut Rectangle>) -> bool;

    /// The ::map-event signal will be emitted when the `widget`'s window is
    /// mapped. A window is mapped when it becomes visible on the screen.
    ///
    /// To receive this signal, the [gdk::Window](crate::gdk::Window) associated to the widget needs
    /// to enable the GDK_STRUCTURE_MASK mask. GDK will enable this mask
    /// automatically for all new windows.
    /// ## `event`
    /// the GdkEventAny which triggered this signal.
    ///
    /// # Returns
    ///
    /// [`true`] to stop other handlers from being invoked for the event.
    ///  [`false`] to propagate the event further.
    fn connect_map_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F)
        -> SignalHandlerId;

    /// The ::unmap-event signal will be emitted when the `widget`'s window is
    /// unmapped. A window is unmapped when it becomes invisible on the screen.
    ///
    /// To receive this signal, the [gdk::Window](crate::gdk::Window) associated to the widget needs
    /// to enable the GDK_STRUCTURE_MASK mask. GDK will enable this mask
    /// automatically for all new windows.
    /// ## `event`
    /// the GdkEventAny which triggered this signal
    ///
    /// # Returns
    ///
    /// [`true`] to stop other handlers from being invoked for the event.
    ///  [`false`] to propagate the event further.
    fn connect_unmap_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// Queues an animation frame update and adds a callback to be called
    /// before each frame. Until the tick callback is removed, it will be
    /// called frequently (usually at the frame rate of the output device
    /// or as quickly as the application can be repainted, whichever is
    /// slower). For this reason, is most suitable for handling graphics
    /// that change every frame or every few frames. The tick callback does
    /// not automatically imply a relayout or repaint. If you want a
    /// repaint or relayout, and aren’t changing widget properties that
    /// would trigger that (for example, changing the text of a [Label](crate::Label)),
    /// then you will have to call [WidgetExt::queue_resize](crate::prelude::WidgetExt::queue_resize) or
    /// [WidgetExt::queue_draw_area](crate::prelude::WidgetExt::queue_draw_area) yourself.
    ///
    /// [FrameClock::frame_time](crate::gdk::FrameClock::frame_time) should generally be used for timing
    /// continuous animations and
    /// `gdk_frame_timings_get_predicted_presentation_time` if you are
    /// trying to display isolated frames at particular times.
    ///
    /// This is a more convenient alternative to connecting directly to the
    /// `GdkFrameClock::::update` signal of [gdk::FrameClock](crate::gdk::FrameClock), since you don't
    /// have to worry about when a [gdk::FrameClock](crate::gdk::FrameClock) is assigned to a widget.
    /// ## `callback`
    /// function to call for updating animations
    /// ## `notify`
    /// function to call to free `user_data` when the callback is removed.
    ///
    /// # Returns
    ///
    /// an id for the connection of this callback. Remove the callback
    ///  by passing it to `gtk_widget_remove_tick_callback`
    #[doc(alias = "gtk_widget_add_tick_callback")]
    fn add_tick_callback<P: Fn(&Self, &gdk::FrameClock) -> Continue + 'static>(
        &self,
        callback: P,
    ) -> TickCallbackId;

    /// Adds the events in the bitfield `events` to the event mask for
    /// `self`. See [WidgetExtManual::set_events](crate::prelude::WidgetExtManual::set_events) and the
    /// [input handling overview][event-masks] for details.
    /// ## `events`
    /// an event mask, see GdkEventMask
    #[doc(alias = "gtk_widget_add_events")]
    fn add_events(&self, events: gdk::EventMask);

    /// Returns the event mask (see GdkEventMask) for the widget. These are the
    /// events that the widget will receive.
    ///
    /// Note: Internally, the widget event mask will be the logical OR of the event
    /// mask set through [WidgetExtManual::set_events](crate::prelude::WidgetExtManual::set_events) or [WidgetExtManual::add_events](crate::prelude::WidgetExtManual::add_events), and the
    /// event mask necessary to cater for every [EventController](crate::EventController) created for the
    /// widget.
    ///
    /// # Returns
    ///
    /// event mask for `self`
    #[doc(alias = "gtk_widget_get_events")]
    #[doc(alias = "get_events")]
    fn events(&self) -> gdk::EventMask;

    /// Sets the event mask (see GdkEventMask) for a widget. The event
    /// mask determines which events a widget will receive. Keep in mind
    /// that different widgets have different default event masks, and by
    /// changing the event mask you may disrupt a widget’s functionality,
    /// so be careful. This function must be called while a widget is
    /// unrealized. Consider [WidgetExtManual::add_events](crate::prelude::WidgetExtManual::add_events) for widgets that are
    /// already realized, or if you want to preserve the existing event
    /// mask. This function can’t be used with widgets that have no window.
    /// (See [WidgetExt::has_window](crate::prelude::WidgetExt::has_window)). To get events on those widgets,
    /// place them inside a [EventBox](crate::EventBox) and receive events on the event
    /// box.
    /// ## `events`
    /// event mask
    #[doc(alias = "gtk_widget_set_events")]
    fn set_events(&self, events: gdk::EventMask);

    // rustdoc-stripper-ignore-next
    /// Calls `gtk_widget_destroy()` on this widget.
    ///
    /// # Safety
    ///
    /// This will not necessarily entirely remove the widget from existence but
    /// you must *NOT* query the widget's state subsequently.  Do not call this
    /// yourself unless you really mean to.
    #[doc(alias = "gtk_widget_destroy")]
    unsafe fn destroy(&self);

    /// Utility function; intended to be connected to the `GtkWidget::::delete-event`
    /// signal on a [Window](crate::Window). The function calls [WidgetExt::hide](crate::prelude::WidgetExt::hide) on its
    /// argument, then returns [`true`]. If connected to ::delete-event, the
    /// result is that clicking the close button for a window (on the
    /// window frame, top right corner usually) will hide but not destroy
    /// the window. By default, GTK+ destroys windows when ::delete-event
    /// is received.
    ///
    /// # Returns
    ///
    /// [`true`]
    #[doc(alias = "gtk_widget_hide_on_delete")]
    fn hide_on_delete(&self) -> Inhibit;
}

impl<O: IsA<Widget>> WidgetExtManual for O {
    fn drag_dest_set(&self, flags: DestDefaults, targets: &[TargetEntry], actions: DragAction) {
        let stashes: Vec<_> = targets.iter().map(|e| e.to_glib_none()).collect();
        let t: Vec<_> = stashes.iter().map(|stash| unsafe { *stash.0 }).collect();
        let t_ptr: *mut ffi::GtkTargetEntry = if !t.is_empty() {
            t.as_ptr() as *mut _
        } else {
            ptr::null_mut()
        };
        unsafe {
            ffi::gtk_drag_dest_set(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                t_ptr,
                t.len() as i32,
                actions.into_glib(),
            )
        };
    }

    fn drag_source_set(
        &self,
        start_button_mask: ModifierType,
        targets: &[TargetEntry],
        actions: DragAction,
    ) {
        let stashes: Vec<_> = targets.iter().map(|e| e.to_glib_none()).collect();
        let t: Vec<_> = stashes.iter().map(|stash| unsafe { *stash.0 }).collect();
        let t_ptr: *mut ffi::GtkTargetEntry = if !t.is_empty() {
            t.as_ptr() as *mut _
        } else {
            ptr::null_mut()
        };
        unsafe {
            ffi::gtk_drag_source_set(
                self.as_ref().to_glib_none().0,
                start_button_mask.into_glib(),
                t_ptr,
                t.len() as i32,
                actions.into_glib(),
            )
        };
    }

    fn intersect(&self, area: &Rectangle, mut intersection: Option<&mut Rectangle>) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_intersect(
                self.as_ref().to_glib_none().0,
                area.to_glib_none().0,
                intersection.to_glib_none_mut().0,
            ))
        }
    }

    fn connect_map_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_any_trampoline<T, F: Fn(&T, &Event) -> Inhibit + 'static>(
            this: *mut ffi::GtkWidget,
            event: *mut gdk::ffi::GdkEventAny,
            f: &F,
        ) -> gboolean
        where
            T: IsA<Widget>,
        {
            f(
                &Widget::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(event),
            )
            .into_glib()
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"map-event\0".as_ptr() as *mut _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    event_any_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    fn connect_unmap_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_any_trampoline<T, F: Fn(&T, &Event) -> Inhibit + 'static>(
            this: *mut ffi::GtkWidget,
            event: *mut gdk::ffi::GdkEventAny,
            f: &F,
        ) -> gboolean
        where
            T: IsA<Widget>,
        {
            f(
                &Widget::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(event),
            )
            .into_glib()
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"unmap-event\0".as_ptr() as *mut _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    event_any_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    fn add_tick_callback<P: Fn(&Self, &gdk::FrameClock) -> Continue + 'static>(
        &self,
        callback: P,
    ) -> TickCallbackId {
        let callback_data: Box<P> = Box::new(callback);

        unsafe extern "C" fn callback_func<
            O: IsA<Widget>,
            P: Fn(&O, &gdk::FrameClock) -> Continue + 'static,
        >(
            widget: *mut ffi::GtkWidget,
            frame_clock: *mut gdk::ffi::GdkFrameClock,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let widget: Borrowed<Widget> = from_glib_borrow(widget);
            let frame_clock = from_glib_borrow(frame_clock);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&widget.unsafe_cast_ref(), &frame_clock);
            res.into_glib()
        }
        let callback = Some(callback_func::<Self, P> as _);

        unsafe extern "C" fn notify_func<
            O: IsA<Widget>,
            P: Fn(&O, &gdk::FrameClock) -> Continue + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box<P> = Box::from_raw(data as *mut _);
        }
        let destroy_call = Some(notify_func::<Self, P> as _);

        let id = unsafe {
            ffi::gtk_widget_add_tick_callback(
                self.as_ref().to_glib_none().0,
                callback,
                Box::into_raw(callback_data) as *mut _,
                destroy_call,
            )
        };
        TickCallbackId {
            id,
            widget: self.upcast_ref().downgrade(),
        }
    }

    fn add_events(&self, events: gdk::EventMask) {
        unsafe {
            ffi::gtk_widget_add_events(self.as_ref().to_glib_none().0, events.into_glib() as i32);
        }
    }

    fn events(&self) -> gdk::EventMask {
        unsafe { from_glib(ffi::gtk_widget_get_events(self.as_ref().to_glib_none().0) as u32) }
    }

    fn set_events(&self, events: gdk::EventMask) {
        unsafe {
            ffi::gtk_widget_set_events(self.as_ref().to_glib_none().0, events.into_glib() as i32);
        }
    }

    unsafe fn destroy(&self) {
        ffi::gtk_widget_destroy(self.as_ref().to_glib_none().0);
    }

    fn hide_on_delete(&self) -> Inhibit {
        unsafe {
            Inhibit(from_glib(ffi::gtk_widget_hide_on_delete(
                self.as_ref().to_glib_none().0,
            )))
        }
    }
}

pub trait InitializingWidgetExt {
    fn init_template(&self);
}

impl<T: crate::subclass::widget::WidgetImpl> InitializingWidgetExt
    for glib::subclass::InitializingObject<T>
{
    fn init_template(&self) {
        unsafe {
            self.as_ref().unsafe_cast_ref::<Widget>().init_template();
        }
    }
}
