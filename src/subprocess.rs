#[cfg(any(feature = "v2_40", feature = "dox"))]
use Subprocess;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use Cancellable;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use Error;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::ptr;

pub trait SubprocessExtManual {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate_utf8_async<'a, P: Into<Option<String>>, Q: Into<Option<&'a Cancellable>>, R: FnOnce(Result<(String, String), Error>) + Send + 'static>(&self, stdin_buf: P, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate_utf8_async_future<P: Into<Option<String>>>(&self, stdin_buf: P) -> Box_<futures_core::Future<Item = (Self, (String, String)), Error = (Self, Error)>>;
}

#[cfg(any(feature = "v2_40", feature = "dox"))]
impl<O: IsA<Subprocess> + IsA<glib::object::Object> + Clone + 'static> SubprocessExtManual for O {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate_utf8_async<'a, P: Into<Option<String>>, Q: Into<Option<&'a Cancellable>>, R: FnOnce(Result<(String, String), Error>) + Send + 'static>(&self, stdin_buf: P, cancellable: Q, callback: R) {
        let stdin_buf = stdin_buf.into();
        let stdin_buf = stdin_buf.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<(R, *mut i8)>> = Box::new(Box::new((callback, stdin_buf.0)));
        unsafe extern "C" fn communicate_utf8_async_trampoline<R: FnOnce(Result<(String, String), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let _ = ffi::g_subprocess_communicate_utf8_finish(_source_object as *mut _, res, &mut stdout_buf, &mut stderr_buf, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf))) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<(R, *mut i8)>> = Box::from_raw(user_data as *mut _);
            glib_ffi::g_free(callback.1 as *mut _);
            callback.0(result);
        }
        let callback = communicate_utf8_async_trampoline::<R>;
        unsafe {
            ffi::g_subprocess_communicate_utf8_async(self.to_glib_none().0, stdin_buf.0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate_utf8_async_future<P: Into<Option<String>>>(&self, stdin_buf: P) -> Box_<futures_core::Future<Item = (Self, (String, String)), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        let stdin_buf = stdin_buf.into();
        let stdin_buf = stdin_buf.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.communicate_utf8_async(
                 stdin_buf.as_ref().map(::std::borrow::Borrow::borrow),
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
