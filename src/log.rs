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
#[cfg(any(feature = "v2_50", feature = "dox"))]
use Variant;

#[derive(Debug)]
pub struct LogHandlerId(u32);

#[doc(hidden)]
impl FromGlib<u32> for LogHandlerId {
    fn from_glib(value: u32) -> LogHandlerId {
        LogHandlerId(value)
    }
}

#[doc(hidden)]
impl ToGlib for LogHandlerId {
    type GlibType = u32;

    fn to_glib(&self) -> u32 {
        self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LogFlag {
    None,
    Recursion,
    Fatal,
}

#[doc(hidden)]
impl ToGlib for LogFlag {
    type GlibType = u32;

    fn to_glib(&self) -> u32 {
        match *self {
            LogFlag::None => 0,
            LogFlag::Recursion => LogLevelFlags::FLAG_RECURSION.bits(),
            LogFlag::Fatal => LogLevelFlags::FLAG_FATAL.bits(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<u32> for LogFlag {
    fn from_glib(value: u32) -> LogFlag {
        if value & LogLevelFlags::FLAG_RECURSION.bits() != 0 {
            LogFlag::Recursion
        } else if value & LogLevelFlags::FLAG_FATAL.bits() != 0 {
            LogFlag::Fatal
        } else {
            LogFlag::None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Critical,
    Warning,
    Message,
    Info,
    Debug,
}

#[doc(hidden)]
impl ToGlib for LogLevel {
    type GlibType = u32;

    fn to_glib(&self) -> u32 {
        match *self {
            LogLevel::Error => LogLevelFlags::LEVEL_ERROR.bits(),
            LogLevel::Critical => LogLevelFlags::LEVEL_CRITICAL.bits(),
            LogLevel::Warning => LogLevelFlags::LEVEL_WARNING.bits(),
            LogLevel::Message => LogLevelFlags::LEVEL_MESSAGE.bits(),
            LogLevel::Info => LogLevelFlags::LEVEL_INFO.bits(),
            LogLevel::Debug => LogLevelFlags::LEVEL_DEBUG.bits(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<u32> for LogLevel {
    fn from_glib(value: u32) -> LogLevel {
        if value & LogLevelFlags::LEVEL_ERROR.bits() != 0 {
            LogLevel::Error
        } else if value & LogLevelFlags::LEVEL_CRITICAL.bits() != 0 {
            LogLevel::Critical
        } else if value & LogLevelFlags::LEVEL_WARNING.bits() != 0 {
            LogLevel::Warning
        } else if value & LogLevelFlags::LEVEL_MESSAGE.bits() != 0 {
            LogLevel::Message
        } else if value & LogLevelFlags::LEVEL_INFO.bits() != 0 {
            LogLevel::Info
        } else if value & LogLevelFlags::LEVEL_DEBUG.bits() != 0 {
            LogLevel::Debug
        } else {
            panic!("Unknown log level: {}", value)
        }
    }
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
pub fn log_set_handler<P: Fn(&str, LogLevel, LogFlag, &str) + Send + Sync + 'static>(
    log_domain: &str,
    log_levels: LogLevel,
    log_flag: LogFlag,
    log_func: P,
) -> LogHandlerId {
    let log_func_data: Box_<P> = Box_::new(log_func);
    unsafe extern "C" fn log_func_func<
        P: Fn(&str, LogLevel, LogFlag, &str) + Send + Sync + 'static,
    >(
        log_domain: *const libc::c_char,
        log_level: glib_sys::GLogLevelFlags,
        message: *const libc::c_char,
        user_data: glib_sys::gpointer,
    ) {
        let log_domain: GString = from_glib_borrow(log_domain);
        let message: GString = from_glib_borrow(message);
        let callback: &P = &*(user_data as *mut _);
        (*callback)(log_domain.as_str(), from_glib(log_level), from_glib(log_level), message.as_str());
    }
    let log_func = Some(log_func_func::<P> as _);
    unsafe extern "C" fn destroy_func<P: Fn(&str, LogLevel, LogFlag, &str) + Send + Sync + 'static>(
        data: glib_sys::gpointer,
    ) {
        let _callback: Box_<P> = Box_::from_raw(data as *mut _);
    }
    let destroy_call4 = Some(destroy_func::<P> as _);
    let super_callback0: Box_<P> = log_func_data;
    unsafe {
        from_glib(glib_sys::g_log_set_handler_full(
            log_domain.to_glib_none().0,
            log_levels.to_glib() | log_flag.to_glib(),
            log_func,
            Box_::into_raw(super_callback0) as *mut _,
            destroy_call4,
        ))
    }
}

pub fn log_remove_handler(log_domain: &str, handler_id: LogHandlerId) {
    unsafe {
        glib_sys::g_log_remove_handler(log_domain.to_glib_none().0, handler_id.to_glib());
    }
}

pub fn log_set_always_fatal(
    fatal_level: LogLevel,
    fatal_flag: LogFlag,
) -> (LogLevel, LogFlag) {
    let ret = unsafe {
        glib_sys::g_log_set_always_fatal(
            fatal_level.to_glib() | fatal_flag.to_glib(),
        )
    };
    (from_glib(ret), from_glib(ret))
}

pub fn log_set_fatal_mask(
    log_domain: &str,
    fatal_level: LogLevel,
    fatal_flag: LogFlag,
) -> (LogLevel, LogFlag) {
    let ret = unsafe {
        glib_sys::g_log_set_fatal_mask(
            log_domain.to_glib_none().0,
            fatal_level.to_glib() | fatal_flag.to_glib(),
        )
    };
    (from_glib(ret), from_glib(ret))
}

#[cfg(any(feature = "v2_50", feature = "dox"))]
pub fn log_variant(log_domain: Option<&str>, log_level: LogLevel, fields: &Variant) {
    unsafe {
        glib_sys::g_log_variant(
            log_domain.to_glib_none().0,
            log_level.to_glib(),
            fields.to_glib_none().0,
        );
    }
}

static PRINT_HANDLER: Lazy<Mutex<Option<Box_<Box_<dyn Fn(&str) + Send + Sync + 'static>>>>> =
    Lazy::new(|| Mutex::new(None));

/// To set back the default print handler, use the [`unset_print_handler`] function.
pub fn set_print_handler<P: Fn(&str) + Send + Sync + 'static>(func: P) {
    unsafe extern "C" fn func_func(string: *const libc::c_char) {
        match PRINT_HANDLER.lock() {
            Ok(handler) => {
                if let Some(ref handler) = *handler {
                    let string: GString = from_glib_borrow(string);
                    (*handler)(string.as_str())
                } else {
                    panic!("PRINT_HANDLER cannot be None!");
                }
            }
            Err(_) => {
                // should we log something here?
            }
        }
    }
    let func = Some(Box_::new(Box_::new(func)));
    match PRINT_HANDLER.lock() {
        Ok(mut handler) => {
            *handler = unsafe { ::std::mem::transmute(func) };
        }
        Err(_) => {
            // should we log something?
        }
    }
    unsafe { glib_sys::g_set_print_handler(Some(func_func as _)) };
}

/// To set the default print handler, use the [`set_print_handler`] function.
pub fn unset_print_handler() {
    match PRINT_HANDLER.lock() {
        Ok(mut handler) => {
            *handler = None;
        }
        Err(_) => {
            // should we log something?
        }
    }
    unsafe { glib_sys::g_set_print_handler(None) };
}

static PRINTERR_HANDLER: Lazy<Mutex<Option<Box_<Box_<dyn Fn(&str) + Send + Sync + 'static>>>>> =
    Lazy::new(|| Mutex::new(None));

/// To set back the default print handler, use the [`unset_printerr_handler`] function.
pub fn set_printerr_handler<P: Fn(&str) + Send + Sync + 'static>(func: P) {
    unsafe extern "C" fn func_func(string: *const libc::c_char) {
        match PRINTERR_HANDLER.lock() {
            Ok(handler) => {
                if let Some(ref handler) = *handler {
                    let string: GString = from_glib_borrow(string);
                    (*handler)(string.as_str())
                } else {
                    panic!("PRINTERR_HANDLER cannot be None!");
                }
            }
            Err(_) => {
                // should we log something here?
            }
        }
    }
    let func = Some(Box_::new(Box_::new(func)));
    match PRINTERR_HANDLER.lock() {
        Ok(mut handler) => {
            *handler = unsafe { ::std::mem::transmute(func) };
        }
        Err(_) => {
            // should we log something?
        }
    }
    unsafe { glib_sys::g_set_printerr_handler(Some(func_func as _)) };
}

/// To set the default print handler, use the [`set_printerr_handler`] function.
pub fn unset_printerr_handler() {
    match PRINTERR_HANDLER.lock() {
        Ok(mut handler) => {
            *handler = None;
        }
        Err(_) => {
            // should we log something?
        }
    }
    unsafe { glib_sys::g_set_printerr_handler(None) };
}

static DEFAULT_HANDLER: Lazy<
    Mutex<Option<Box_<Box_<dyn Fn(&str, LogLevel, LogFlag, &str) + Send + Sync + 'static>>>>,
> = Lazy::new(|| Mutex::new(None));

/// To set back the default print handler, use the [`log_unset_default_handler`] function.
pub fn log_set_default_handler<P: Fn(&str, LogLevel, LogFlag, &str) + Send + Sync + 'static>(
    log_func: P,
) {
    unsafe extern "C" fn func_func(
        log_domain: *const libc::c_char,
        log_level: glib_sys::GLogLevelFlags,
        message: *const libc::c_char,
        _user_data: glib_sys::gpointer,
    ) {
        match DEFAULT_HANDLER.lock() {
            Ok(handler) => {
                if let Some(ref handler) = *handler {
                    let log_domain: GString = from_glib_borrow(log_domain);
                    let message: GString = from_glib_borrow(message);
                    (*handler)(log_domain.as_str(), from_glib(log_level), from_glib(log_level), message.as_str())
                } else {
                    panic!("DEFAULT_HANDLER cannot be None!");
                }
            }
            Err(_) => {
                // should we log something here?
            }
        }
    }
    let log_func = Some(Box_::new(Box_::new(log_func)));
    match DEFAULT_HANDLER.lock() {
        Ok(mut handler) => {
            *handler = unsafe { ::std::mem::transmute(log_func) };
        }
        Err(_) => {
            // should we log something?
        }
    }
    unsafe { glib_sys::g_log_set_default_handler(Some(func_func as _), ::std::ptr::null_mut()) };
}

/// To set the default print handler, use the [`log_set_default_handler`] function.
pub fn log_unset_default_handler() {
    match PRINTERR_HANDLER.lock() {
        Ok(mut handler) => {
            *handler = None;
        }
        Err(_) => {
            // should we log something?
        }
    }
    unsafe { glib_sys::g_log_set_default_handler(None, ::std::ptr::null_mut()) };
}

/// Macro used to log using GLib logging system. Is uses [g_log].
///
/// [g_log]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log)
///
/// Example:
///
/// ```no_run
/// use glib::{LogLevel, g_log};
///
/// g_log!("test", LogLevel::Debug, "test");
/// g_log!("test", LogLevel::Message, "test");
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_log!("test", LogLevel::Error, "test: {}", x);
/// g_log!("test", LogLevel::Critical, "test: {}", x);
/// g_log!("test", LogLevel::Warning, "test: {} {}", x, "a");
/// ```
#[macro_export]
macro_rules! g_log {
    ($log_domain:expr, $log_level:expr, $format:expr) => {{
        use $crate::translate::{ToGlib, ToGlibPtr};
        use $crate::LogLevel;

        fn check_log_args(_log_domain: &str, _log_level: LogLevel, _format: &str) {}

        check_log_args(&$log_domain, $log_level, $format);
        // the next line is used to enforce the type for the macro checker...
        let log_domain: &str = $log_domain;
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
        use $crate::LogLevel;

        fn check_log_args(_log_domain: &str, _log_level: LogLevel, _format: &str) {}

        check_log_args(&$log_domain, $log_level, $format);
        // the next line is used to enforce the type for the macro checker...
        let log_domain: &str = $log_domain;
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

// /// Macro used to log using GLib logging system. Is uses [g_log_structured][gls].
// ///
// /// [gls]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log-structured)
// ///
// /// Example:
// ///
// /// ```no_run
// /// use glib::{LogLevel, g_log_structured};
// ///
// /// g_log_structured!("test", LogLevel::Debug, {"MESSAGE" => "tadam!"});
// /// g_log_structured!("test", LogLevel::Debug, {"MESSAGE" => "tadam!", "random" => "yes"});
// /// ```
// #[cfg(any(feature = "v2_50", feature = "dox"))]
// #[macro_export]
// macro_rules! g_log_structured {
//     ($log_domain:expr, $log_level:expr, {$($key:expr => $value:expr),+}) => {{
//         use $crate::translate::{Stash, ToGlib, ToGlibPtr};
//         use $crate::LogLevel;
//         use std::ffi::CString;

//         fn check_log_args(_log_domain: &str, _log_level: LogLevel) {}
//         fn check_key(key: &str) -> Stash<*const i8, str> { key.to_glib_none() }

//         check_log_args(&$log_domain, $log_level);
//         unsafe {
//             glib_sys::g_log_structured(
//                 $log_domain.to_glib_none().0,
//                 $log_level.to_glib(),
//                 $(check_key($key).0, check_key(format!("{}", $value).as_str()).0 ),+
//             )
//         }
//     }};
// }

pub fn log_default_handler(log_domain: &str, log_level: LogLevel, message: Option<&str>) {
    unsafe {
        glib_sys::g_log_default_handler(
            log_domain.to_glib_none().0,
            log_level.to_glib(),
            message.to_glib_none().0,
            ::std::ptr::null_mut(),
        )
    }
}
