// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi as ffi;

use EventType;
use Window;

glib_wrapper! {
    /// A generic GDK event.
    pub struct Event(Boxed<ffi::GdkEvent>);

    match fn {
        copy => |ptr| ffi::gdk_event_copy(ptr),
        free => |ptr| ffi::gdk_event_free(ptr),
    }
}

impl Event {
    /// Returns the event type.
    pub fn get_event_type(&self) -> EventType {
        self.as_ref().type_
    }

    /// Returns the associated `Window` if applicable.
    pub fn get_window(&self) -> Option<Window> {
        unsafe { from_glib_none(self.as_ref().window) }
    }

    /// Returns whether the event was sent explicitly.
    pub fn get_send_event(&self) -> bool {
        from_glib(self.as_ref().send_event as i32)
    }

    /// Returns `true` if the event type matches `T`.
    pub fn is<T: FromEvent>(&self) -> bool {
        T::is(self)
    }

    /// Tries to downcast to a specific event type.
    pub fn downcast<T: FromEvent>(self) -> Result<T, Self> {
        T::from(self)
    }
}

/// A helper trait implemented by all event subtypes.
pub trait FromEvent: Sized {
    fn is(ev: &Event) -> bool;
    fn from(ev: Event) -> Result<Self, Event>;
}

macro_rules! event_wrapper {
    ($name:ident, $ffi_name:ident) => {
        impl<'a> ToGlibPtr<'a, *const ::ffi::$ffi_name> for $name {
            type Storage = &'a Self;

            #[inline]
            fn to_glib_none(&'a self) -> Stash<'a, *const ::ffi::$ffi_name, Self> {
                let ptr = ToGlibPtr::<*const ::ffi::GdkEvent>::to_glib_none(&self.0).0;
                Stash(ptr as *const ::ffi::$ffi_name, self)
            }
        }

        impl<'a> ToGlibPtrMut<'a, *mut ::ffi::$ffi_name> for $name {
            type Storage = &'a mut Self;

            #[inline]
            fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ::ffi::$ffi_name, Self> {
                let ptr = ToGlibPtrMut::<*mut ::ffi::GdkEvent>::to_glib_none_mut(&mut self.0).0;
                StashMut(ptr as *mut ::ffi::$ffi_name, self)
            }
        }

        impl FromGlibPtrNone<*mut ::ffi::$ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut ::ffi::$ffi_name) -> Self {
                $name(from_glib_none(ptr as *mut ::ffi::GdkEvent))
            }
        }

        impl FromGlibPtrFull<*mut ::ffi::$ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut ::ffi::$ffi_name) -> Self {
                $name(from_glib_full(ptr as *mut ::ffi::GdkEvent))
            }
        }

        impl AsRef<::ffi::$ffi_name> for $name {
            #[inline]
            fn as_ref(&self) -> &::ffi::$ffi_name {
                unsafe {
                    let ptr: *const ::ffi::$ffi_name = self.to_glib_none().0;
                    &*ptr
                }
            }
        }

        impl AsMut<::ffi::$ffi_name> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut ::ffi::$ffi_name {
                unsafe {
                    let ptr: *mut ::ffi::$ffi_name = self.to_glib_none_mut().0;
                    &mut *ptr
                }
            }
        }
    }
}

event_wrapper!(Event, GdkEventAny);

macro_rules! event_subtype {
    ($name:ident, $($ty:ident)|+) => {
        impl ::event::FromEvent for $name {
            #[inline]
            fn is(ev: &::event::Event) -> bool {
                skip_assert_initialized!();
                use EventType::*;
                match ev.as_ref().type_ {
                    $($ty)|+ => true,
                    _ => false,
                }
            }

            #[inline]
            fn from(ev: ::event::Event) -> Result<Self, ::event::Event> {
                skip_assert_initialized!();
                if Self::is(&ev) {
                    Ok($name(ev))
                }
                else {
                    Err(ev)
                }
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = ::event::Event;

            fn deref(&self) -> &::event::Event {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut ::event::Event {
                &mut self.0
            }
        }
    }
}
