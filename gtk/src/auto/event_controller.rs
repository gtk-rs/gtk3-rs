// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::PropagationPhase;
use crate::Widget;
use gdk;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct EventController(Object<ffi::GtkEventController, ffi::GtkEventControllerClass>);

    match fn {
        get_type => || ffi::gtk_event_controller_get_type(),
    }
}

pub const NONE_EVENT_CONTROLLER: Option<&EventController> = None;

pub trait EventControllerExt: 'static {
    fn get_propagation_phase(&self) -> PropagationPhase;

    fn get_widget(&self) -> Option<Widget>;

    fn handle_event(&self, event: &gdk::Event) -> bool;

    fn reset(&self);

    fn set_propagation_phase(&self, phase: PropagationPhase);

    fn connect_property_propagation_phase_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<EventController>> EventControllerExt for O {
    fn get_propagation_phase(&self) -> PropagationPhase {
        unsafe {
            from_glib(ffi::gtk_event_controller_get_propagation_phase(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_event_controller_get_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_handle_event(
                self.as_ref().to_glib_none().0,
                event.to_glib_none().0,
            ))
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::gtk_event_controller_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn set_propagation_phase(&self, phase: PropagationPhase) {
        unsafe {
            ffi::gtk_event_controller_set_propagation_phase(
                self.as_ref().to_glib_none().0,
                phase.to_glib(),
            );
        }
    }

    fn connect_property_propagation_phase_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_propagation_phase_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEventController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<EventController>,
        {
            let f: &F = &*(f as *const F);
            f(&EventController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::propagation-phase\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_propagation_phase_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EventController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventController")
    }
}
