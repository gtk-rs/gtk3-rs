// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::{DragAction, Event, ModifierType};
use glib::ffi::gboolean;
use glib::object::{Cast, IsA, WeakRef};
use glib::signal::{connect_raw, Inhibit, SignalHandlerId};
use glib::translate::*;
use glib::Continue;
use glib::ObjectExt;
use std::mem::transmute;
use std::ptr;

use crate::{DestDefaults, Rectangle, TargetEntry, Widget, WidgetExt};

pub struct TickCallbackId {
    id: u32,
    widget: WeakRef<Widget>,
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
    #[doc(alias = "gtk_drag_dest_set")]
    fn drag_dest_set(&self, flags: DestDefaults, targets: &[TargetEntry], actions: DragAction);

    #[doc(alias = "gtk_drag_source_set")]
    fn drag_source_set(
        &self,
        start_button_mask: ModifierType,
        targets: &[TargetEntry],
        actions: DragAction,
    );

    #[doc(alias = "gtk_widget_intersect")]
    fn intersect(&self, area: &Rectangle, intersection: Option<&mut Rectangle>) -> bool;

    fn connect_map_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_unmap_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "gtk_widget_add_tick_callback")]
    fn add_tick_callback<P: Fn(&Self, &gdk::FrameClock) -> Continue + 'static>(
        &self,
        callback: P,
    ) -> TickCallbackId;

    #[doc(alias = "gtk_widget_add_events")]
    fn add_events(&self, events: gdk::EventMask);

    #[doc(alias = "gtk_widget_get_events")]
    fn get_events(&self) -> gdk::EventMask;

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
                flags.to_glib(),
                t_ptr,
                t.len() as i32,
                actions.to_glib(),
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
                start_button_mask.to_glib(),
                t_ptr,
                t.len() as i32,
                actions.to_glib(),
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
            .to_glib()
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
            .to_glib()
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
            res.to_glib()
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
            ffi::gtk_widget_add_events(self.as_ref().to_glib_none().0, events.to_glib() as i32);
        }
    }

    fn get_events(&self) -> gdk::EventMask {
        unsafe { from_glib(ffi::gtk_widget_get_events(self.as_ref().to_glib_none().0) as u32) }
    }

    fn set_events(&self, events: gdk::EventMask) {
        unsafe {
            ffi::gtk_widget_set_events(self.as_ref().to_glib_none().0, events.to_glib() as i32);
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

impl<T: IsA<Widget>> InitializingWidgetExt for glib::subclass::InitializingObject<T> {
    fn init_template(&self) {
        unsafe {
            self.as_ref().init_template();
        }
    }
}
