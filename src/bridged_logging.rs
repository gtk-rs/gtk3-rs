// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

extern crate log as rs_log;

use glib_sys;
use translate::*;

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

    fn level_to_glib(level: rs_log::Level) -> glib_sys::GLogLevelFlags {
        match level {
            // Errors are mapped to critical to avoid automatic termination
            rs_log::Level::Error => glib_sys::G_LOG_LEVEL_CRITICAL,
            rs_log::Level::Warn => glib_sys::G_LOG_LEVEL_WARNING,
            rs_log::Level::Info => glib_sys::G_LOG_LEVEL_INFO,
            rs_log::Level::Debug => glib_sys::G_LOG_LEVEL_DEBUG,
            // There is no equivalent to trace level in glib
            rs_log::Level::Trace => glib_sys::G_LOG_LEVEL_DEBUG,
        }
    }

    fn write_log(domain: Option<&str>, level: rs_log::Level, message: &str) {
        unsafe {
            crate::glib_sys::g_log(
                domain.to_glib_none().0,
                GlibLogger::level_to_glib(level),
                message.replace("%", "%%").to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
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
            crate::glib_sys::g_log_structured_standard(
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
