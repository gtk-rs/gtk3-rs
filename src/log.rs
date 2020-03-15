// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib_sys;
use once_cell::sync::Lazy;
use std::boxed::Box as Box_;
use std::sync::Mutex;
use translate::*;
use GString;
use LogLevelFlags;
use LogWriterOutput;

pub enum LogValue {
    String(String),
    Value(Vec<u8>),
}

pub struct LogField {
    key: GString,
    value: LogValue,
}

#[doc(hidden)]
impl ToGlib for LogField {
    type GlibType = glib_sys::GLogField;

    fn to_glib(&self) -> glib_sys::GLogField {
        match self.value {
            LogValue::String(ref s) => {
                let s: *const libc::c_char = s.to_glib_none().0;
                glib_sys::GLogField {
                    key: self.key.to_glib_none().0,
                    value: s as _,
                    length: -1,
                }
            }
            LogValue::Value(ref v) => glib_sys::GLogField {
                key: self.key.to_glib_none().0,
                value: v.as_ptr() as _,
                length: v.len() as _,
            },
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const glib_sys::GLogField> for &'a [&LogField] {
    type Storage = Vec<glib_sys::GLogField>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const glib_sys::GLogField, Self> {
        let fields = self.iter().map(|x| x.to_glib()).collect::<Vec<_>>();
        Stash(fields.as_ptr() as *const _, fields)
    }

    #[inline]
    fn to_glib_full(&self) -> *const glib_sys::GLogField {
        let ptr = unsafe {
            glib_sys::g_malloc0(::std::mem::size_of::<glib_sys::GLogField>() * self.len())
                as *mut glib_sys::GLogField
        };
        for (pos, x) in self.iter().enumerate() {
            unsafe {
                *(ptr.offset(pos as _)) = x.to_glib();
            }
        }
        ptr as *const _
    }
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
pub fn log_set_handler_full<P: Fn(&str, &LogLevelFlags, &str) + Send + Sync + 'static>(
    log_domain: Option<&str>,
    log_levels: LogLevelFlags,
    log_func: P,
) -> u32 {
    let log_func_data: Box_<P> = Box_::new(log_func);
    unsafe extern "C" fn log_func_func<
        P: Fn(&str, &LogLevelFlags, &str) + Send + Sync + 'static,
    >(
        log_domain: *const libc::c_char,
        log_level: glib_sys::GLogLevelFlags,
        message: *const libc::c_char,
        user_data: glib_sys::gpointer,
    ) {
        let log_domain: GString = from_glib_borrow(log_domain);
        let log_level = from_glib(log_level);
        let message: GString = from_glib_borrow(message);
        let callback: &P = &*(user_data as *mut _);
        (*callback)(log_domain.as_str(), &log_level, message.as_str());
    }
    let log_func = Some(log_func_func::<P> as _);
    unsafe extern "C" fn destroy_func<P: Fn(&str, &LogLevelFlags, &str) + Send + Sync + 'static>(
        data: glib_sys::gpointer,
    ) {
        let _callback: Box_<P> = Box_::from_raw(data as *mut _);
    }
    let destroy_call4 = Some(destroy_func::<P> as _);
    let super_callback0: Box_<P> = log_func_data;
    unsafe {
        glib_sys::g_log_set_handler_full(
            log_domain.to_glib_none().0,
            log_levels.to_glib(),
            log_func,
            Box_::into_raw(super_callback0) as *mut _,
            destroy_call4,
        )
    }
}

pub fn log_set_handler<P: Fn(&str, &LogLevelFlags, &str) + Send + Sync + 'static>(
    log_domain: Option<&str>,
    log_levels: LogLevelFlags,
    log_func: P,
) -> u32 {
    let log_func_data: Box_<P> = Box_::new(log_func);
    unsafe extern "C" fn log_func_func<
        P: Fn(&str, &LogLevelFlags, &str) + Send + Sync + 'static,
    >(
        log_domain: *const libc::c_char,
        log_level: glib_sys::GLogLevelFlags,
        message: *const libc::c_char,
        user_data: glib_sys::gpointer,
    ) {
        let log_domain: GString = from_glib_borrow(log_domain);
        let log_level = from_glib(log_level);
        let message: GString = from_glib_borrow(message);
        let callback: &P = &*(user_data as *mut _);
        (*callback)(log_domain.as_str(), &log_level, message.as_str());
    }
    let log_func = Some(log_func_func::<P> as _);
    let super_callback0: Box_<P> = log_func_data;
    unsafe {
        glib_sys::g_log_set_handler(
            log_domain.to_glib_none().0,
            log_levels.to_glib(),
            log_func,
            Box_::into_raw(super_callback0) as *mut _,
        )
    }
}

static PRINT_HANDLER: Lazy<Mutex<Option<Box_<Box_<dyn Fn(GString) + Send + Sync + 'static>>>>> =
    Lazy::new(|| Mutex::new(None));

pub fn set_print_handler<P: Fn(GString) + Send + Sync + 'static>(func: Option<P>) {
    unsafe extern "C" fn func_func(string: *const libc::c_char) {
        match PRINT_HANDLER.lock() {
            Ok(handler) => {
                if let Some(ref handler) = *handler {
                    let string: GString = from_glib_borrow(string);
                    (*handler)(string)
                } else {
                    panic!("PRINT_HANDLER cannot be None!");
                }
            }
            Err(_) => {
                // should we log something here?
            }
        }
    }
    let callback = if func.is_some() {
        Some(func_func as _)
    } else {
        None
    };
    let func = func.map(|f| Box_::new(Box_::new(f)));
    match PRINT_HANDLER.lock() {
        Ok(mut handler) => {
            *handler = unsafe { ::std::mem::transmute(func) };
        }
        Err(_) => {
            // should we log something?
        }
    }
    unsafe { glib_sys::g_set_printerr_handler(callback) };
}

static PRINTERR_HANDLER: Lazy<Mutex<Option<Box_<Box_<dyn Fn(GString) + Send + Sync + 'static>>>>> =
    Lazy::new(|| Mutex::new(None));

pub fn set_printerr_handler<P: Fn(GString) + Send + Sync + 'static>(func: Option<P>) {
    unsafe extern "C" fn func_func(string: *const libc::c_char) {
        match PRINTERR_HANDLER.lock() {
            Ok(handler) => {
                if let Some(ref handler) = *handler {
                    let string: GString = from_glib_borrow(string);
                    (*handler)(string)
                } else {
                    panic!("PRINTERR_HANDLER cannot be None!");
                }
            }
            Err(_) => {
                // should we log something here?
            }
        }
    }
    let callback = if func.is_some() {
        Some(func_func as _)
    } else {
        None
    };
    let func = func.map(|f| Box_::new(Box_::new(f)));
    match PRINTERR_HANDLER.lock() {
        Ok(mut handler) => {
            *handler = unsafe { ::std::mem::transmute(func) };
        }
        Err(_) => {
            // should we log something?
        }
    }
    unsafe { glib_sys::g_set_printerr_handler(callback) };
}

pub fn log_set_default_handler<P: Fn(&str, &LogLevelFlags, &str) + Send + Sync + 'static>(
    log_func: P,
) {
    let log_func_data: Box_<P> = Box_::new(log_func);
    unsafe extern "C" fn log_func_func<
        P: Fn(&str, &LogLevelFlags, &str) + Send + Sync + 'static,
    >(
        log_domain: *const libc::c_char,
        log_level: glib_sys::GLogLevelFlags,
        message: *const libc::c_char,
        user_data: glib_sys::gpointer,
    ) {
        let log_domain: GString = from_glib_borrow(log_domain);
        let log_level = from_glib(log_level);
        let message: GString = from_glib_borrow(message);
        let callback: &P = &*(user_data as *mut _);
        (*callback)(log_domain.as_str(), &log_level, message.as_str());
    }
    let log_func = Some(log_func_func::<P> as _);
    let super_callback: Box_<P> = log_func_data;
    unsafe {
        glib_sys::g_log_set_default_handler(log_func, Box_::into_raw(super_callback) as *mut _);
    }
}

/// Macro used to log using GLib logging system. Is uses [g_log].
///
/// [g_log]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log)
///
/// Example:
///
/// ```no_run
/// use glib::{LogLevelFlags, g_log};
///
/// g_log!(None, LogLevelFlags::FLAG_RECURSION, "test");
/// g_log!(Some("test"), LogLevelFlags::FLAG_FATAL, "test");
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_log!(None, LogLevelFlags::FLAG_RECURSION, "test: {}", x);
/// g_log!(Some("test"), LogLevelFlags::FLAG_RECURSION, "test: {}", x);
/// g_log!(None, LogLevelFlags::FLAG_FATAL, "test: {} {}", x, "a");
/// ```
#[macro_export]
macro_rules! g_log {
    ($log_domain:expr, $log_level:expr, $format:expr) => {{
        use $crate::translate::{ToGlib, ToGlibPtr};
        use $crate::LogLevelFlags;

        fn check_log_args(_log_domain: &Option<&str>, _log_level: &LogLevelFlags, _format: &str) {}

        check_log_args(&$log_domain, &$log_level, $format);
        // the next line is used to enforce the type for the macro checker...
        let log_domain: Option<&str> = $log_domain;
        unsafe {
            $crate::glib_sys::g_log(
                log_domain.to_glib_none().0,
                $log_level.to_glib(),
                // to prevent the glib formatter to look for arguments which don't exist
                $format.replace("%", "%%").to_glib_none().0,
            );
        }
    }};
    ($log_domain:expr, $log_level:expr, $format:expr, $($arg:tt),*) => {{
        use $crate::translate::{ToGlib, ToGlibPtr};
        use $crate::LogLevelFlags;

        fn check_log_args(_log_domain: &Option<&str>, _log_level: &LogLevelFlags, _format: &str) {}

        check_log_args(&$log_domain, &$log_level, $format);
        // the next line is used to enforce the type for the macro checker...
        let log_domain: Option<&str> = $log_domain;
        unsafe {
            $crate::glib_sys::g_log(
                log_domain.to_glib_none().0,
                $log_level.to_glib(),
                format!($format, $($arg),*)
                    // to prevent the glib formatter to look for arguments which don't exist
                    .replace("%", "%%").to_glib_none().0,
            );
        }
    }};
}

/// Macro used to log using GLib logging system. Is uses [g_log_structured][gls].
///
/// [gls]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log-structured)
///
/// Example:
///
/// ```no_run
/// use glib::{LogLevelFlags, g_log_structured};
///
/// g_log_structured!("test", LogLevelFlags::FLAG_RECURSION, {"MESSAGE" => "tadam!"});
/// g_log_structured!("test", LogLevelFlags::FLAG_FATAL, {"MESSAGE" => "tadam!", "random" => "yes"});
/// ```
#[cfg(any(feature = "v2_50", feature = "dox"))]
#[macro_export]
macro_rules! g_log_structured {
    ($log_domain:expr, $log_level:expr, {$($key:expr => $value:expr),+}) => {{
        use $crate::translate::{Stash, ToGlib, ToGlibPtr};
        use $crate::LogLevelFlags;
        use std::ffi::CString;

        fn check_log_args(_log_domain: &str, _log_level: &LogLevelFlags) {}
        fn check_key(key: &str) -> Stash<*const i8, str> { key.to_glib_none() }

        check_log_args(&$log_domain, &$log_level);
        unsafe {
            glib_sys::g_log_structured(
                $log_domain.to_glib_none().0,
                $log_level.to_glib(),
                $(check_key($key).0, check_key(format!("{}", $value).as_str()).0 ),+
            )
        }
    }};
}

pub fn log_default_handler(
    log_domain: Option<&str>,
    log_level: LogLevelFlags,
    message: Option<&str>,
) {
    unsafe {
        glib_sys::g_log_default_handler(
            log_domain.to_glib_none().0,
            log_level.to_glib(),
            message.to_glib_none().0,
            ::std::ptr::null_mut(),
        )
    }
}

#[cfg(any(feature = "v2_50", feature = "dox"))]
pub fn log_set_writer_func(
    func: Option<Box_<dyn Fn(&LogLevelFlags, &[LogField]) -> LogWriterOutput + 'static>>,
) {
    let func_data: Box_<
        Option<Box_<dyn Fn(&LogLevelFlags, &[LogField]) -> LogWriterOutput + 'static>>,
    > = Box_::new(func);
    unsafe extern "C" fn func_func(
        log_level: glib_sys::GLogLevelFlags,
        fields: *const glib_sys::GLogField,
        n_fields: libc::size_t,
        user_data: glib_sys::gpointer,
    ) -> glib_sys::GLogWriterOutput {
        let log_level = from_glib(log_level);
        let fields = Vec::from_raw_parts(fields as _, n_fields as _, n_fields as _);
        let callback: &Option<
            Box_<dyn Fn(&LogLevelFlags, &[LogField]) -> LogWriterOutput + 'static>,
        > = &*(user_data as *mut _);
        let res = if let Some(ref callback) = *callback {
            callback(&log_level, &fields)
        } else {
            panic!("cannot get closure...")
        };
        ::std::mem::forget(fields);
        res.to_glib()
    }
    let func = if func_data.is_some() {
        Some(func_func as _)
    } else {
        None
    };
    unsafe extern "C" fn user_data_free_func(data: glib_sys::gpointer) {
        let _callback: Box_<
            Option<Box_<dyn Fn(&LogLevelFlags, &[LogField]) -> LogWriterOutput + 'static>>,
        > = Box_::from_raw(data as *mut _);
    }
    let destroy_call2 = Some(user_data_free_func as _);
    let super_callback0: Box_<
        Option<Box_<dyn Fn(&LogLevelFlags, &[LogField]) -> LogWriterOutput + 'static>>,
    > = func_data;
    unsafe {
        glib_sys::g_log_set_writer_func(
            func,
            Box_::into_raw(super_callback0) as *mut _,
            destroy_call2,
        );
    }
}
