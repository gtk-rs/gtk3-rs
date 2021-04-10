// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Cancellable;
use crate::File;
use crate::FileCreateFlags;
use crate::FileEnumerator;
use crate::FileQueryInfoFlags;
use glib::object::IsA;
use glib::translate::*;
use std::pin::Pin;
use std::ptr;

pub trait FileExtManual: Sized {
    #[doc(alias = "g_file_replace_contents_async")]
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

    #[doc(alias = "g_file_enumerate_children_async")]
    fn enumerate_children_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<FileEnumerator, glib::Error>) + Send + 'static,
    >(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn enumerate_children_async_future(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<FileEnumerator, glib::Error>> + 'static>>;
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

    fn enumerate_children_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<FileEnumerator, glib::Error>) + Send + 'static,
    >(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn create_async_trampoline<
            Q: FnOnce(Result<FileEnumerator, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_file_enumerate_children_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = create_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_enumerate_children_async(
                self.as_ref().to_glib_none().0,
                attributes.to_glib_none().0,
                flags.to_glib(),
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn enumerate_children_async_future(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<FileEnumerator, glib::Error>> + 'static>>
    {
        Box::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.enumerate_children_async(
                attributes,
                flags,
                io_priority,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }
}
