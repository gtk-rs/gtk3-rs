// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `Error` binding and helper trait.

use glib_sys;
use std::borrow::Cow;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::str;
use translate::*;
use Quark;

glib_wrapper! {
    /// A generic error capable of representing various error domains (types).
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Error(Boxed<glib_sys::GError>);

    match fn {
        copy => |ptr| glib_sys::g_error_copy(ptr),
        free => |ptr| glib_sys::g_error_free(ptr),
        get_type => || glib_sys::g_error_get_type(),
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl Error {
    /// Creates an error with supplied error enum variant and message.
    pub fn new<T: ErrorDomain>(error: T, message: &str) -> Error {
        unsafe {
            from_glib_full(glib_sys::g_error_new_literal(
                T::domain().to_glib(),
                error.code(),
                message.to_glib_none().0,
            ))
        }
    }

    /// Checks if the error domain matches `T`.
    pub fn is<T: ErrorDomain>(&self) -> bool {
        self.0.domain == T::domain().to_glib()
    }

    /// Tries to convert to a specific error enum.
    ///
    /// Returns `Some` if the error belongs to the enum's error domain and
    /// `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// if let Some(file_error) = error.kind::<FileError>() {
    ///     match file_error {
    ///         FileError::Exist => ...
    ///         FileError::Isdir => ...
    ///         ...
    ///     }
    /// }
    /// ```
    ///
    /// ```ignore
    /// match error {
    ///     Some(FileError::Exist) => ...
    ///     Some(FileError::Isdir) => ...
    ///     ...
    /// }
    /// ```
    pub fn kind<T: ErrorDomain>(&self) -> Option<T> {
        if self.0.domain == T::domain().to_glib() {
            T::from(self.0.code)
        } else {
            None
        }
    }

    fn message(&self) -> &str {
        unsafe {
            let bytes = CStr::from_ptr(self.0.message).to_bytes();
            str::from_utf8(bytes)
                .unwrap_or_else(|err| str::from_utf8(&bytes[..err.valid_up_to()]).unwrap())
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("domain", &::Quark::from_glib(self.0.domain))
            .field("code", &self.0.code)
            .field("message", &self.message())
            .finish()
    }
}

/// `GLib` error domain.
///
/// This trait is implemented by error enums that represent error domains (types).
pub trait ErrorDomain: Copy {
    /// Returns the quark identifying the error domain.
    ///
    /// As returned from `g_some_error_quark`.
    fn domain() -> Quark;

    /// Gets the integer representation of the variant.
    fn code(self) -> i32;

    /// Tries to convert an integer code to an enum variant.
    ///
    /// By convention, the `Failed` variant, if present, is a catch-all,
    /// i.e. any unrecognized codes map to it.
    fn from(code: i32) -> Option<Self>
    where
        Self: Sized;
}

/// Generic error used for functions that fail without any further information
#[macro_export]
macro_rules! glib_bool_error(
// Plain strings
    ($msg:expr) =>  {
        $crate::BoolError::new($msg, file!(), module_path!(), line!())
    };

// Format strings
    ($($msg:tt)*) =>  { {
        $crate::BoolError::new(format!($($msg)*), file!(), module_path!(), line!())
    }};
);

#[macro_export]
macro_rules! glib_result_from_gboolean(
// Plain strings
    ($ffi_bool:expr, $msg:expr) =>  {
        $crate::BoolError::from_glib($ffi_bool, $msg, file!(), module_path!(), line!())
    };

// Format strings
    ($ffi_bool:expr, $($msg:tt)*) =>  { {
        $crate::BoolError::from_glib(
            $ffi_bool,
            format!($($msg)*),
            file!(),
            module_path!(),
            line!(),
        )
    }};
);

#[derive(Debug, Clone)]
pub struct BoolError {
    pub message: Cow<'static, str>,
    #[doc(hidden)]
    pub filename: &'static str,
    #[doc(hidden)]
    pub function: &'static str,
    #[doc(hidden)]
    pub line: u32,
}

impl BoolError {
    pub fn new<Msg: Into<Cow<'static, str>>>(
        message: Msg,
        filename: &'static str,
        function: &'static str,
        line: u32,
    ) -> Self {
        BoolError {
            message: message.into(),
            filename,
            function,
            line,
        }
    }

    pub fn from_glib<Msg: Into<Cow<'static, str>>>(
        b: glib_sys::gboolean,
        message: Msg,
        filename: &'static str,
        function: &'static str,
        line: u32,
    ) -> Result<(), Self> {
        match b {
            glib_sys::GFALSE => Err(BoolError::new(message, filename, function, line)),
            _ => Ok(()),
        }
    }
}

impl fmt::Display for BoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error {:?} in {:?} at {}:{}",
            self.message, self.function, self.filename, self.line
        )
    }
}

impl error::Error for BoolError {
    fn description(&self) -> &str {
        self.message.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_error() {
        use std::error::Error;

        let from_static_msg = glib_bool_error!("Static message");
        assert_eq!(from_static_msg.description(), "Static message");

        let from_dynamic_msg = glib_bool_error!("{} message", "Dynamic");
        assert_eq!(from_dynamic_msg.description(), "Dynamic message");

        let false_static_res = glib_result_from_gboolean!(glib_sys::GFALSE, "Static message");
        assert!(false_static_res.is_err());
        let static_err = false_static_res.err().unwrap();
        assert_eq!(static_err.description(), "Static message");

        let true_static_res = glib_result_from_gboolean!(glib_sys::GTRUE, "Static message");
        assert!(true_static_res.is_ok());

        let false_dynamic_res =
            glib_result_from_gboolean!(glib_sys::GFALSE, "{} message", "Dynamic");
        assert!(false_dynamic_res.is_err());
        let dynamic_err = false_dynamic_res.err().unwrap();
        assert_eq!(dynamic_err.description(), "Dynamic message");

        let true_dynamic_res = glib_result_from_gboolean!(glib_sys::GTRUE, "{} message", "Dynamic");
        assert!(true_dynamic_res.is_ok());
    }
}
