#[cfg(feature = "futures")]
use futures_core;
use gio_sys;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gobject_sys;
use libc::c_char;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::ptr;
use Cancellable;
use Error;
use Subprocess;

impl Subprocess {
    pub fn communicate_utf8_async<R: FnOnce(Result<(GString, GString), Error>) + Send + 'static>(&self, stdin_buf: Option<String>, cancellable: Option<&Cancellable>, callback: R) {
        let stdin_buf = stdin_buf.to_glib_full();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<(R, *mut c_char)> = Box::new((callback, stdin_buf));
        unsafe extern "C" fn communicate_utf8_async_trampoline<R: FnOnce(Result<(GString, GString), Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer)
        {
            let mut error = ptr::null_mut();
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let _ = gio_sys::g_subprocess_communicate_utf8_finish(_source_object as *mut _, res, &mut stdout_buf, &mut stderr_buf, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf))) } else { Err(from_glib_full(error)) };
            let callback: Box<(R, *mut c_char)> = Box::from_raw(user_data as *mut _);
            glib_sys::g_free(callback.1 as *mut _);
            callback.0(result);
        }
        unsafe {
            gio_sys::g_subprocess_communicate_utf8_async(
                self.to_glib_none().0,
                stdin_buf,
                cancellable.0,
                Some(communicate_utf8_async_trampoline::<R>),
                Box::into_raw(user_data) as *mut _
            );
        }
    }

    #[cfg(feature = "futures")]
    pub fn communicate_utf8_async_future(&self, stdin_buf: Option<String>) -> Box_<futures_core::Future<Item = (Self, (GString, GString)), Error = (Self, Error)>> where Self: Clone {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.communicate_utf8_async(
                 stdin_buf,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }
}
