// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ::ffi::{self, cairo_status_t};
use ::{Status, Surface, UserDataKey};

use libc::{c_void, c_double, c_uchar, c_uint};
use std::any::Any;
use std::cell::{Cell, RefCell};
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
        let env_rc = Rc::new(CallbackEnvironment {
            mutable: RefCell::new(MutableCallbackEnvironment {
                stream: Some(Box::new(stream)),
                io_error: None,
                unwind_payload: None,
            }),
            saw_already_borrowed: Cell::new(false),
        });
        let env: *const CallbackEnvironment = &*env_rc;
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
        where F: FnOnce(&mut MutableCallbackEnvironment) -> Option<R>
    {
        let env = self.get_user_data_ptr(&STREAM_CALLBACK_ENVIRONMENT)?;

        // Safety: contract of `get_user_data_ptr`
        let env = unsafe { env.as_ref() };

        if env.saw_already_borrowed.get() {
            panic!("The output stream’s RefCell was already borrowed when cairo attempted a write")
        }

        let mutable = &mut *env.mutable.borrow_mut();
        if let Some(payload) = mutable.unwind_payload.take() {
            std::panic::resume_unwind(payload)
        }
        f(mutable)
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

static STREAM_CALLBACK_ENVIRONMENT: UserDataKey<CallbackEnvironment> =
    UserDataKey::new();

struct CallbackEnvironment {
    mutable: RefCell<MutableCallbackEnvironment>,
    saw_already_borrowed: Cell<bool>,
}

struct MutableCallbackEnvironment {
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
    let env: *const CallbackEnvironment = env as _;

    // Safety: the user data entry keeps `Rc<CallbackEnvironment>` alive
    // until the surface is destroyed.
    // If this is called by cairo, the surface is still alive.
    let env: &CallbackEnvironment = unsafe { &*env };

    if let Ok(mut mutable) = env.mutable.try_borrow_mut() {
        if let MutableCallbackEnvironment {
            stream: Some(stream),
            // Don’t attempt another write if a previous one errored or panicked:
            io_error: io_error @ None,
            unwind_payload: unwind_payload @ None,
        } = &mut *mutable {
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
                    return Status::Success.into()
                }
                Ok(Err(error)) => {
                    *io_error = Some(error);
                }
                Err(payload) => {
                    *unwind_payload = Some(payload);
                }
            }
        }
    } else {
        // This can happen if `W` holds a reference to the surface,
        // and caused cairo to make a write while the previous one was still ongoing.
        env.saw_already_borrowed.set(true)
    }
    Status::WriteError.into()
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
