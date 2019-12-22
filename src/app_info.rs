// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[cfg(any(feature = "v2_60", feature = "dox"))]
use gio_sys;
use glib::object::IsA;
#[cfg(any(feature = "v2_60", feature = "dox"))]
use glib::translate::*;
#[cfg(any(feature = "v2_60", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v2_60", feature = "dox"))]
use gobject_sys;
#[cfg(any(feature = "v2_60", feature = "dox"))]
use std::boxed::Box as Box_;
#[cfg(any(feature = "v2_60", feature = "dox"))]
use std::pin::Pin;
#[cfg(any(feature = "v2_60", feature = "dox"))]
use std::ptr;
use AppInfo;
#[cfg(any(feature = "v2_60", feature = "dox"))]
use AppLaunchContext;
#[cfg(any(feature = "v2_60", feature = "dox"))]
use Cancellable;

pub trait AppInfoExtManual: 'static {
    #[cfg(any(feature = "v2_60", feature = "dox"))]
    fn launch_uris_async<
        P: IsA<AppLaunchContext>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        uris: &[&str],
        context: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    fn launch_uris_async_future<P: IsA<AppLaunchContext> + Clone + 'static>(
        &self,
        uris: &[&str],
        context: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;
}

impl<O: IsA<AppInfo>> AppInfoExtManual for O {
    #[cfg(any(feature = "v2_60", feature = "dox"))]
    fn launch_uris_async<
        P: IsA<AppLaunchContext>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        uris: &[&str],
        context: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<(R, *mut *mut libc::c_char)> =
            Box_::new((callback, uris.to_glib_full()));
        unsafe extern "C" fn launch_uris_async_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                gio_sys::g_app_info_launch_uris_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<(R, *mut *mut libc::c_char)> = Box_::from_raw(user_data as *mut _);
            (callback.0)(result);
            glib_sys::g_strfreev(callback.1);
        }
        let callback = launch_uris_async_trampoline::<R>;
        unsafe {
            gio_sys::g_app_info_launch_uris_async(
                self.as_ref().to_glib_none().0,
                uris.to_glib_none().0,
                context.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    fn launch_uris_async_future<P: IsA<AppLaunchContext> + Clone + 'static>(
        &self,
        uris: &[&str],
        context: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let uris = uris.iter().copied().map(String::from).collect::<Vec<_>>();
        let context = context.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let uris = uris
                .iter()
                .map(::std::borrow::Borrow::borrow)
                .collect::<Vec<_>>();
            obj.launch_uris_async(
                uris.as_ref(),
                context.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }
}
