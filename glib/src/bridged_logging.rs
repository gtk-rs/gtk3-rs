// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use crate::ffi;
use crate::log as glib_log;
use crate::translate::*;

/// Enumeration of the possible formatting behaviours for a
/// [`GlibLogger`](struct.GlibLogger.html).
///
/// In order to use this type, `glib` must be built with the `log` feature
/// enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlibLoggerFormat {
    /// A simple format, writing only the message on output.
    Plain,
    /// A simple format, writing file, line and message on output.
    LineAndFile,
    /// A logger using glib structured logging. Structured logging is available
    /// only on features `v2_56` and later.
    #[cfg(any(feature = "v2_56", feature = "dox"))]
    Structured,
}

/// Enumeration of the possible domain handling behaviours for a
/// [`GlibLogger`](struct.GlibLogger.html).
///
/// In order to use this type, `glib` must be built with the `log` feature
/// enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlibLoggerDomain {
    /// Logs will have no domain specified.
    None,
    /// Logs will use the `target` of the log crate as a domain; this allows
    /// Rust code like `warn!(target: "my-domain", "...");` to log to the glib
    /// logger using the specified domain.
    CrateTarget,
    /// Logs will use the crate path as the log domain.
    CratePath,
}

/// An implementation of a [`log`](https://crates.io/crates/log) compatible
/// logger which logs over glib logging facilities.
///
/// In order to use this type, `glib` must be built with the `log` feature
/// enabled.
///
/// Use this if you want to use glib as the main logging output in your application,
/// and want to route all logging happening through the log crate to glib logging.
/// If you want the opposite, see
/// [`rust_log_handler`](fn.rust_log_handler.html).
///
/// NOTE: This should never be used when
/// [`rust_log_handler`](fn.rust_log_handler.html) has
/// been registered as a default glib log handler, otherwise a stack overflow
/// will occur.
///
/// Example:
///
/// ```no_run
/// extern crate log;
///
/// static glib_logger: glib::GlibLogger = glib::GlibLogger::new(
///     glib::GlibLoggerFormat::Plain,
///     glib::GlibLoggerDomain::CrateTarget,
/// );
///
/// log::set_logger(&glib_logger);
/// log::set_max_level(log::LevelFilter::Debug);
///
/// log::info!("This line will get logged by glib");
/// ```
#[derive(Debug)]
pub struct GlibLogger {
    format: GlibLoggerFormat,
    domain: GlibLoggerDomain,
}

impl GlibLogger {
    /// Creates a new instance of [`GlibLogger`](struct.GlibLogger.html).
    /// See documentation of [`GlibLogger`](struct.GlibLogger.html) for more
    /// information.
    ///
    /// Example:
    ///
    /// ```no_run
    /// extern crate log;
    ///
    /// static glib_logger: glib::GlibLogger = glib::GlibLogger::new(
    ///     glib::GlibLoggerFormat::Plain,
    ///     glib::GlibLoggerDomain::CrateTarget,
    /// );
    ///
    /// log::set_logger(&glib_logger);
    /// log::set_max_level(log::LevelFilter::Debug);
    ///
    /// log::info!("This line will get logged by glib");
    /// ```
    pub const fn new(format: GlibLoggerFormat, domain: GlibLoggerDomain) -> Self {
        GlibLogger { format, domain }
    }

    fn level_to_glib(level: rs_log::Level) -> ffi::GLogLevelFlags {
        match level {
            // Errors are mapped to critical to avoid automatic termination
            rs_log::Level::Error => ffi::G_LOG_LEVEL_CRITICAL,
            rs_log::Level::Warn => ffi::G_LOG_LEVEL_WARNING,
            rs_log::Level::Info => ffi::G_LOG_LEVEL_INFO,
            rs_log::Level::Debug => ffi::G_LOG_LEVEL_DEBUG,
            // There is no equivalent to trace level in glib
            rs_log::Level::Trace => ffi::G_LOG_LEVEL_DEBUG,
        }
    }

    fn write_log(domain: Option<&str>, level: rs_log::Level, message: &str) {
        unsafe {
            crate::ffi::g_log(
                domain.to_glib_none().0,
                GlibLogger::level_to_glib(level),
                message.replace("%", "%%").to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn write_log_structured(
        domain: Option<&str>,
        level: log::Level,
        file: Option<&str>,
        line: Option<u32>,
        func: Option<&str>,
        message: &str,
    ) {
        let line_str = match line {
            None => None,
            Some(l) => Some(l.to_string()),
        };

        unsafe {
            crate::ffi::g_log_structured_standard(
                domain.to_glib_none().0,
                GlibLogger::level_to_glib(level),
                file.to_glib_none().0,
                line_str.to_glib_none().0,
                func.to_glib_none().0,
                message.replace("%", "%%").to_glib_none().0,
            );
        }
    }
}

impl rs_log::Log for GlibLogger {
    fn enabled(&self, _: &rs_log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &rs_log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let domain = match &self.domain {
            GlibLoggerDomain::None => None,
            GlibLoggerDomain::CrateTarget => Some(record.metadata().target()),
            GlibLoggerDomain::CratePath => record.module_path(),
        };

        match self.format {
            GlibLoggerFormat::Plain => {
                let s = format!("{}", record.args());
                GlibLogger::write_log(domain, record.level(), &s)
            }
            GlibLoggerFormat::LineAndFile => {
                let s = match (record.file(), record.line()) {
                    (Some(file), Some(line)) => format!("{}:{}: {}", file, line, record.args()),
                    (Some(file), None) => format!("{}: {}", file, record.args()),
                    _ => format!("{}", record.args()),
                };

                GlibLogger::write_log(domain, record.level(), &s);
            }
            #[cfg(any(feature = "v2_56", feature = "dox"))]
            GlibLoggerFormat::Structured => {
                GlibLogger::write_log_structured(
                    domain,
                    record.level(),
                    record.file(),
                    record.line(),
                    None,
                    &format!("{}", record.args()),
                );
            }
        };
    }

    fn flush(&self) {}
}

/// Provides a glib log handler which routes all logging messages to the
/// [`log crate`](https://crates.io/crates/log).
///
/// In order to use this function, `glib` must be built with the `log` feature
/// enabled.
///
/// Use this function if you want to use the log crate as the main logging
/// output in your application, and want to route all logging happening in
/// glib to the log crate. If you want the opposite, use [`GlibLogger`](struct.GlibLogger.html).
///
/// NOTE: This should never be used when [`GlibLogger`](struct.GlibLogger.html) is
/// registered as a logger, otherwise a stack overflow will occur.
///
/// ```no_run
/// glib::log_set_default_handler(glib::rust_log_handler);
/// ```
pub fn rust_log_handler(domain: Option<&str>, level: glib_log::LogLevel, message: &str) {
    let level = match level {
        glib_log::LogLevel::Error | glib_log::LogLevel::Critical => log::Level::Error,
        glib_log::LogLevel::Warning => log::Level::Warn,
        glib_log::LogLevel::Message | glib_log::LogLevel::Info => log::Level::Info,
        glib_log::LogLevel::Debug => log::Level::Debug,
    };

    rs_log::log!(target: domain.unwrap_or("<null>"), level, "{}", message);
}

/// A macro which behaves exactly as `log::error!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` constant (and fails
/// to build if not defined).
///
/// In order to use this macro, `glib` must be built with the `log_macros`
/// feature enabled and the [`GlibLogger`](struct.GlibLogger.html) must have been
/// initialized using [`GlibLoggerDomain::CrateTarget`](enum.GlibLoggerDomain.html).
///
/// ```no_run
/// #[macro_use] extern crate glib;
///
/// static G_LOG_DOMAIN: &str = "my-domain";
///
/// error!("This will be logged under 'my-domain'");
/// ```
#[macro_export]
#[cfg(any(feature = "dox", feature = "log_macros"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "log_macros")))]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Error, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Error, $($arg)+);
    )
}

/// A macro which behaves exactly as `log::warn!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` constant (and fails
/// to build if not defined).
///
/// In order to use this macro, `glib` must be built with the `log_macros`
/// feature enabled and the [`GlibLogger`](struct.GlibLogger.html) must have been
/// initialized using [`GlibLoggerDomain::CrateTarget`](enum.GlibLoggerDomain.html).
///
/// ```no_run
/// #[macro_use] extern crate glib;
///
/// static G_LOG_DOMAIN: &str = "my-domain";
///
/// warn!("This will be logged under 'my-domain'");
/// ```
#[macro_export]
#[cfg(any(feature = "dox", feature = "log_macros"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "log_macros")))]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Warn, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Warn, $($arg)+);
    )
}

/// A macro which behaves exactly as `log::info!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` constant (and fails
/// to build if not defined).
///
/// In order to use this macro, `glib` must be built with the `log_macros`
/// feature enabled and the [`GlibLogger`](struct.GlibLogger.html) must have been
/// initialized using [`GlibLoggerDomain::CrateTarget`](enum.GlibLoggerDomain.html).
///
/// ```no_run
/// #[macro_use] extern crate glib;
///
/// static G_LOG_DOMAIN: &str = "my-domain";
///
/// info!("This will be logged under 'my-domain'");
/// ```
#[macro_export]
#[cfg(any(feature = "dox", feature = "log_macros"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "log_macros")))]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Info, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Info, $($arg)+);
    )
}

/// A macro which behaves exactly as `log::debug!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` constant (and fails
/// to build if not defined).
///
/// In order to use this macro, `glib` must be built with the `log_macros`
/// feature enabled and the [`GlibLogger`](struct.GlibLogger.html) must have been
/// initialized using [`GlibLoggerDomain::CrateTarget`](enum.GlibLoggerDomain.html).
///
/// ```no_run
/// #[macro_use] extern crate glib;
///
/// static G_LOG_DOMAIN: &str = "my-domain";
///
/// debug!("This will be logged under 'my-domain'");
/// ```
#[macro_export]
#[cfg(any(feature = "dox", feature = "log_macros"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "log_macros")))]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Debug, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Debug, $($arg)+);
    )
}

/// A macro which behaves exactly as `log::trace!` except that it sets the
/// current log target to the contents of a `G_LOG_DOMAIN` constant (and fails
/// to build if not defined).
///
/// In order to use this macro, `glib` must be built with the `log_macros`
/// feature enabled and the [`GlibLogger`](struct.GlibLogger.html) must have been
/// initialized using [`GlibLoggerDomain::CrateTarget`](enum.GlibLoggerDomain.html).
///
/// ```no_run
/// #[macro_use] extern crate glib;
///
/// static G_LOG_DOMAIN: &str = "my-domain";
///
/// trace!("This will be logged under 'my-domain'");
/// ```
#[macro_export]
#[cfg(any(feature = "dox", feature = "log_macros"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "log_macros")))]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        log::log!(target: $target, log::Level::Trace, $($arg)+);
    );
    ($($arg:tt)+) => (
        log::log!(target: G_LOG_DOMAIN, log::Level::Trace, $($arg)+);
    )
}
