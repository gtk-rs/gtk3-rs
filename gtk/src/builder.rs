// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Builder;
use glib::prelude::*;
use glib::translate::*;
use glib::GString;
use glib::Object;
use std::path::Path;

impl Builder {
    #[doc(alias = "gtk_builder_new_from_file")]
    pub fn from_file<T: AsRef<Path>>(file_path: T) -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_file(
                file_path.as_ref().to_glib_none().0,
            ))
        }
    }
}

pub trait BuilderExtManual: 'static {
    #[doc(alias = "gtk_builder_get_object")]
    #[doc(alias = "get_object")]
    fn object<T: IsA<Object>>(&self, name: &str) -> Option<T>;

    /// Parses a file containing a [GtkBuilder UI definition][BUILDER-UI]
    /// and merges it with the current contents of `self`.
    ///
    /// Most users will probably want to use `gtk_builder_new_from_file`.
    ///
    /// If an error occurs, 0 will be returned and `error` will be assigned a
    /// [glib::Error](crate::glib::Error) from the GTK_BUILDER_ERROR, G_MARKUP_ERROR or G_FILE_ERROR
    /// domain.
    ///
    /// It’s not really reasonable to attempt to handle failures of this
    /// call. You should not use this function with untrusted files (ie:
    /// files that are not part of your application). Broken [Builder](crate::Builder)
    /// files can easily crash your program, and it’s possible that memory
    /// was leaked leading up to the reported failure. The only reasonable
    /// thing to do when an error is detected is to call `g_error`.
    /// ## `filename`
    /// the name of the file to parse
    ///
    /// # Returns
    ///
    /// A positive value on success, 0 if an error occurred
    #[doc(alias = "gtk_builder_add_from_file")]
    fn add_from_file<T: AsRef<Path>>(&self, file_path: T) -> Result<(), glib::Error>;
    #[doc(alias = "gtk_builder_connect_signals_full")]
    fn connect_signals<
        P: FnMut(&Builder, &str) -> Box<dyn Fn(&[glib::Value]) -> Option<glib::Value> + 'static>,
    >(
        &self,
        func: P,
    );
}

impl<O: IsA<Builder>> BuilderExtManual for O {
    fn object<T: IsA<Object>>(&self, name: &str) -> Option<T> {
        unsafe {
            Option::<Object>::from_glib_none(ffi::gtk_builder_get_object(
                self.upcast_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
            .and_then(|obj| obj.dynamic_cast::<T>().ok())
        }
    }

    fn add_from_file<T: AsRef<Path>>(&self, file_path: T) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ::std::ptr::null_mut();
            ffi::gtk_builder_add_from_file(
                self.upcast_ref().to_glib_none().0,
                file_path.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_signals<
        P: FnMut(&Builder, &str) -> Box<dyn Fn(&[glib::Value]) -> Option<glib::Value> + 'static>,
    >(
        &self,
        func: P,
    ) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<
            P: FnMut(&Builder, &str) -> Box<dyn Fn(&[glib::Value]) -> Option<glib::Value> + 'static>,
        >(
            builder: *mut ffi::GtkBuilder,
            object: *mut glib::gobject_ffi::GObject,
            signal_name: *const libc::c_char,
            handler_name: *const libc::c_char,
            connect_object: *mut glib::gobject_ffi::GObject,
            flags: glib::gobject_ffi::GConnectFlags,
            user_data: glib::ffi::gpointer,
        ) {
            assert!(connect_object.is_null(), "Connect object is not supported");
            assert!(
                flags & glib::gobject_ffi::G_CONNECT_SWAPPED == 0,
                "Swapped signal handler is not supported"
            );

            let builder = from_glib_borrow(builder);
            let object: Borrowed<glib::Object> = from_glib_borrow(object);
            let signal_name: Borrowed<GString> = from_glib_borrow(signal_name);
            let handler_name: Borrowed<GString> = from_glib_borrow(handler_name);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let func = (*callback)(&builder, handler_name.as_str());
            object
                .connect_unsafe(
                    signal_name.as_str(),
                    flags & glib::gobject_ffi::G_CONNECT_AFTER != 0,
                    move |args| func(args),
                )
                .expect("Failed to connect to builder signal");
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_builder_connect_signals_full(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }
}
