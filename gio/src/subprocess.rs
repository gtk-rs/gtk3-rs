// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Cancellable;
use crate::Subprocess;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use libc::c_char;
use std::pin::Pin;
use std::ptr;

impl Subprocess {
    #[doc(alias = "g_subprocess_communicate_utf8_async")]
    pub fn communicate_utf8_async<
        R: FnOnce(Result<(Option<GString>, Option<GString>), glib::Error>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        stdin_buf: Option<String>,
        cancellable: Option<&C>,
        callback: R,
    ) {
        let stdin_buf = stdin_buf.to_glib_full();
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let user_data: Box<(R, *mut c_char)> = Box::new((callback, stdin_buf));
        unsafe extern "C" fn communicate_utf8_async_trampoline<
            R: FnOnce(Result<(Option<GString>, Option<GString>), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let _ = ffi::g_subprocess_communicate_utf8_finish(
                _source_object as *mut _,
                res,
                &mut stdout_buf,
                &mut stderr_buf,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<(R, *mut c_char)> = Box::from_raw(user_data as *mut _);
            glib::ffi::g_free(callback.1 as *mut _);
            callback.0(result);
        }
        unsafe {
            ffi::g_subprocess_communicate_utf8_async(
                self.to_glib_none().0,
                stdin_buf,
                gcancellable.0,
                Some(communicate_utf8_async_trampoline::<R>),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn communicate_utf8_async_future(
        &self,
        stdin_buf: Option<String>,
    ) -> Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<(Option<GString>, Option<GString>), glib::Error>,
                > + 'static,
        >,
    > {
        Box::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.communicate_utf8_async(stdin_buf, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}
