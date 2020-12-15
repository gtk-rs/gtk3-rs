// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Cancellable;
use crate::File;
use crate::FileCreateFlags;
use glib::object::IsA;
use glib::translate::*;
use std::pin::Pin;
use std::ptr;

pub trait FileExtManual: Sized {
    fn replace_contents_async<
        B: AsRef<[u8]> + Send + 'static,
        R: FnOnce(Result<(B, glib::GString), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
        cancellable: Option<&C>,
        callback: R,
    );

    fn replace_contents_async_future<B: AsRef<[u8]> + Send + 'static>(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<(B, glib::GString), (B, glib::Error)>>
                + 'static,
        >,
    >;
}

impl<O: IsA<File>> FileExtManual for O {
    fn replace_contents_async<
        B: AsRef<[u8]> + Send + 'static,
        R: FnOnce(Result<(B, glib::GString), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
        cancellable: Option<&C>,
        callback: R,
    ) {
        let etag = etag.to_glib_none();
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let user_data: Box<Option<(R, B)>> = Box::new(Some((callback, contents)));
        // Need to do this after boxing as the contents pointer might change by moving into the box
        let (count, contents_ptr) = {
            let contents = &(*user_data).as_ref().unwrap().1;
            let slice = contents.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe extern "C" fn replace_contents_async_trampoline<
            B: AsRef<[u8]> + Send + 'static,
            R: FnOnce(Result<(B, glib::GString), (B, glib::Error)>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut user_data: Box<Option<(R, B)>> = Box::from_raw(user_data as *mut _);
            let (callback, contents) = user_data.take().unwrap();

            let mut error = ptr::null_mut();
            let mut new_etag = ptr::null_mut();
            let _ = ffi::g_file_replace_contents_finish(
                _source_object as *mut _,
                res,
                &mut new_etag,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((contents, from_glib_full(new_etag)))
            } else {
                Err((contents, from_glib_full(error)))
            };
            callback(result);
        }
        let callback = replace_contents_async_trampoline::<B, R>;
        unsafe {
            ffi::g_file_replace_contents_async(
                self.as_ref().to_glib_none().0,
                mut_override(contents_ptr),
                count,
                etag.0,
                make_backup.to_glib(),
                flags.to_glib(),
                gcancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn replace_contents_async_future<B: AsRef<[u8]> + Send + 'static>(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<(B, glib::GString), (B, glib::Error)>>
                + 'static,
        >,
    > {
        let etag = etag.map(glib::GString::from);
        Box::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.replace_contents_async(
                contents,
                etag.as_ref().map(|s| s.as_str()),
                make_backup,
                flags,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }
}
