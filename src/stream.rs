// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ::ffi::{self, cairo_status_t};
use ::{Status, Surface, UserDataKey};

use libc::{c_void, c_double, c_uchar, c_uint};
use std::any::Any;
use std::cell::UnsafeCell;
use std::io;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

macro_rules! for_stream_constructors {
    ($constructor_ffi: ident) => {
        /// Takes full ownership of the output stream,
        /// which is not allowed to borrow any lifetime shorter than `'static`.
        ///
        /// Because the underlying `cairo_surface_t` is reference-counted,
        /// a lifetime parameter in a Rust wrapper type would not be enough to track
        /// how long it can keep writing to the stream.
        pub fn for_stream<W: io::Write + 'static>(width: f64, height: f64, stream: W) -> Self {
            Self {
                inner: Surface::_for_stream(
                    ffi::$constructor_ffi,
                    width,
                    height,
                    stream,
                ),
            }
        }

        /// Allows writing to a borrowed stream. The lifetime of the borrow is not tracked.
        ///
        /// # Safety
        ///
        /// The value that `stream` points to must live at least until the underlying `cairo_surface_t`
        /// (which maybe be longer then the Rust `PdfSurface` wrapper, because of reference-counting),
        /// or until the output stream is removed from the surface with [`Surface::take_output_stream`].
        ///
        /// Since the former is hard to track for sure, the latter is strongly recommended.
        /// The concrete type behind the `Box<dyn Any>` value returned by `take_output_stream`
        /// is private, so you won’t be able to downcast it.
        /// But removing it anyway ensures that later writes do no go through a dangling pointer.
        pub unsafe fn for_raw_stream<W: io::Write + 'static>(width: f64, height: f64, stream: *mut W) -> Self {
            Self {
                inner: Surface::_for_raw_stream(
                    ffi::$constructor_ffi,
                    width,
                    height,
                    stream,
                ),
            }
        }
    };
}

impl Surface {
    pub(crate) fn _for_stream<W: io::Write + 'static>(
        constructor: Constructor,
        width: f64,
        height: f64,
        stream: W,
    ) -> Self {
        let env_rc = Rc::new(UnsafeCell::new(CallbackEnvironment {
            stream: Some(Box::new(stream)),
            io_error: None,
            unwind_payload: None,
        }));
        let env: *const UnsafeCell<CallbackEnvironment> = &*env_rc;
        unsafe {
            let ptr = constructor(Some(write_callback::<W>), env as *mut c_void, width, height);
            let surface = Surface::from_raw_full(ptr);
            surface.set_user_data(&STREAM_CALLBACK_ENVIRONMENT, env_rc);
            surface
        }
    }

    pub(crate) unsafe fn _for_raw_stream<W: io::Write + 'static>(
        constructor: Constructor,
        width: f64,
        height: f64,
        stream: *mut W,
    ) -> Self {
        Self::_for_stream(constructor, width, height, RawStream(stream))
    }

    fn with_stream_env<R, F>(&self, f: F) -> Option<R>
        where F: FnOnce(&mut CallbackEnvironment) -> Option<R>
    {
        let env = self.get_user_data_ptr(&STREAM_CALLBACK_ENVIRONMENT)?;

        // Safety: contract of `get_user_data_ptr`
        let env = unsafe { env.as_ref() };

        with_mut_env(env, |env| {
            if let Some(payload) = env.unwind_payload.take() {
                std::panic::resume_unwind(payload)
            }
            f(env)
        })
    }

    /// Remove and return the output stream, if any.
    ///
    /// This is relevant for surfaces created for example with [`PdfSurface::for_stream`].
    /// Consider calling [`Surface::finish`] first,
    /// to ensure that all writes to the stream are done.
    ///
    /// Use [`Box::downcast`] to recover the concrete type.
    pub fn take_output_stream(&self) -> Option<Box<dyn Any>> {
        self.with_stream_env(|env| env.stream.take())
    }

    /// Remove and return the last error that occurred while writing to the output stream, if any.
    pub fn take_io_error(&self) -> Result<(), io::Error> {
        match self.with_stream_env(|env| env.io_error.take()) {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    /// Return whether the surface has an associated output stream (that hasn’t been removed yet).
    pub fn has_output_stream(&self) -> bool {
        self.with_stream_env(|env| Some(env.stream.is_some())).unwrap_or(false)
    }

    /// Return whether an error that occurred while writing to the output stream, if any.
    pub fn has_io_error(&self) -> bool {
        self.with_stream_env(|env| Some(env.io_error.is_some())).unwrap_or(false)
    }
}

pub(crate) type Constructor = unsafe extern fn(
    ffi::cairo_write_func_t,
    *mut c_void,
    c_double,
    c_double,
) -> *mut ffi::cairo_surface_t;

static STREAM_CALLBACK_ENVIRONMENT: UserDataKey<UnsafeCell<CallbackEnvironment>> =
    UserDataKey::new();

struct CallbackEnvironment {
    stream: Option<Box<dyn Any>>,
    io_error: Option<io::Error>,
    unwind_payload: Option<Box<dyn Any + Send + 'static>>,
}

// Safety: unwinding into C is undefined behavior (https://github.com/rust-lang/rust/issues/58794)
// so code outside of the `catch_unwind` call must never panic.
extern "C" fn write_callback<W: io::Write + 'static>(
    env: *mut c_void,
    data: *mut c_uchar,
    length: c_uint,
) -> cairo_status_t {
    // This is consistent with the type of `env` in `Surface::_for_stream`.
    let env: *const UnsafeCell<CallbackEnvironment> = env as _;

    // Safety: the user data entry keeps `Rc<UnsafeCell<CallbackEnvironment>>` alive
    // until the surface is destroyed.
    // If this is called by cairo, the surface is still alive.
    let env: &UnsafeCell<CallbackEnvironment> = unsafe { &*env };

    with_mut_env(env, |env| {
        if let CallbackEnvironment {
            stream: Some(stream),
            // Don’t attempt another write if a previous one errored or panicked:
            io_error: None,
            unwind_payload: None,
        } = env {
            // Safety: `write_callback<W>` was instanciated in `Surface::_for_stream`
            // with a W parameter consistent with the box that was unsized to `Box<dyn Any>`.
            let stream = unsafe {
                stream.downcast_mut_unchecked::<W>()
            };
            // Safety: this is the callback contract from cairo’s API
            let data = unsafe {
                std::slice::from_raw_parts(data, length as usize)
            };
            // Because `<W as Write>::write_all` is a generic,
            // we must conservatively assume that it can panic.
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| stream.write_all(data)));
            match result {
                Ok(Ok(())) => {
                    return Status::Success
                }
                Ok(Err(error)) => {
                    env.io_error = Some(error);
                }
                Err(payload) => {
                    env.unwind_payload = Some(payload);
                }
            }
        }
        Status::WriteError
    }).into()
}

fn with_mut_env<R, F>(env: &UnsafeCell<CallbackEnvironment>, f: F) -> R
    where F: FnOnce(&mut CallbackEnvironment) -> R
{
    // Safety:
    //
    // * The only long-lived pointers to this environment are
    //   the `void` pointer passed to surface constructor, and the one in user data.
    // * `STREAM_CALLBACK_ENVIRONMENT` is private,
    //   so the user data entry is only accessible in this module.
    // * `Surface` is !Send and !Sync, so a given surface is only used on a single thread.
    //
    // Therefore, there are no other ongoing reference to (part of) this `CallbackEnvironment`
    // and it is sound to claim exclusive / mutable access with `&mut`.
    unsafe {
        f(&mut *env.get())
    }
}

struct RawStream<W>(*mut W);

impl<W: io::Write> io::Write for RawStream<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { (*self.0).write(buf) }}
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> { unsafe { (*self.0).write_all(buf) }}
    fn flush(&mut self) -> io::Result<()> { unsafe { (*self.0).flush() } }
}

trait AnyExt {
    /// Any::downcast_mut, but YOLO
    unsafe fn downcast_mut_unchecked<T>(&mut self) -> &mut T {
        let ptr = self as *mut Self as *mut T;
        &mut *ptr
    }
}
impl AnyExt for dyn Any {}
