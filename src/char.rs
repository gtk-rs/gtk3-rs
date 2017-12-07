use libc::{c_char, c_uchar};
use translate::FromGlib;
use translate::ToGlib;

/// Wrapper for values where C functions expect a plain C `char`
///
/// Consider the following C function prototype from glib:
///
/// ```C
/// void g_key_file_set_list_separator (GKeyFile *key_file, gchar separator);
/// ```
///
/// This function plainly expects a byte as the `separator` argument.  However,
/// having this function exposed to Rust as the following would be inconvenient:
///
/// ```ignore
/// impl KeyFile {
///     pub fn set_list_separator(&self, separator: i8) { }
/// }
/// ```
///
/// This would be inconvenient because users would have to do the conversion from a Rust `char` to an `i8` by hand.  It would be the same case for `libc::c_char`, which is just a type alias
/// for `i8`.
///
/// This `Char` type is a wrapper over an `i8`, so that we can pass it to Glib or C functions.
/// The check for whether a Rust `char` (a Unicode scalar value) actually fits in a `i8` is
/// done in the `new` function; see its documentation for details.
///
/// The inner `i8` (which is equivalent to `libc::c_char` can be extracted with `.0`, or
/// by calling `my_char.to_glib()`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Char(pub i8);

impl Char {
    /// Creates a `Some(Char)` if the given `char` is representable as an `i8`
    ///
    /// # Example
    /// ```ignore
    /// extern "C" fn have_a_byte(b: libc::c_char);
    ///
    /// let a = Char::new('a').unwrap();
    /// assert!(a.0 == 65);
    /// have_a_byte(a.to_glib());
    ///
    /// let not_representable = Char::new('☔');
    /// assert!(not_representable.is_none());
    /// ```
    pub fn new(c: char) -> Option<Char> {
        if c as u32 > 255 {
            None
        } else {
            Some(Char(c as i8))
        }
    }
}

impl From<Char> for char {
    fn from(c: Char) -> char {
        c.0 as u8 as char
    }
}

#[doc(hidden)]
impl FromGlib<c_char> for Char {
    fn from_glib(value: c_char) -> Self {
        Char(value)
    }
}

#[doc(hidden)]
impl ToGlib for Char {
    type GlibType = c_char;

    fn to_glib(&self) -> c_char {
        self.0
    }
}

/// Wrapper for values where C functions expect a plain C `unsigned char`
///
/// This `UChar` type is a wrapper over an `u8`, so that we can pass it to Glib or C functions.
/// The check for whether a Rust `char` (a Unicode scalar value) actually fits in a `u8` is
/// done in the `new` function; see its documentation for details.
///
/// The inner `u8` (which is equivalent to `libc::c_uchar` can be extracted with `.0`, or
/// by calling `my_char.to_glib()`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct UChar(pub u8);

impl UChar {
    /// Creates a `Some(UChar)` if the given `char` is representable as an `u8`
    ///
    /// # Example
    /// ```ignore
    /// extern "C" fn have_a_byte(b: libc::c_uchar);
    ///
    /// let a = Char::new('a').unwrap();
    /// assert!(a.0 == 65);
    /// have_a_byte(a.to_glib());
    ///
    /// let not_representable = Char::new('☔');
    /// assert!(not_representable.is_none());
    /// ```
    pub fn new(c: char) -> Option<UChar> {
        if c as u32 > 255 {
            None
        } else {
            Some(UChar(c as u8))
        }
    }
}

impl From<UChar> for char {
    fn from(c: UChar) -> char {
        c.0 as char
    }
}

#[doc(hidden)]
impl FromGlib<c_uchar> for UChar {
    fn from_glib(value: c_uchar) -> Self {
        UChar(value)
    }
}

#[doc(hidden)]
impl ToGlib for UChar {
    type GlibType = c_uchar;

    fn to_glib(&self) -> c_uchar {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use translate::from_glib;

    #[test]
    fn converts_single_byte_chars() {
        assert_eq!(Char::new(0 as char), Some(Char(0)));
        assert_eq!(Char::new(255 as char), Some(Char(-1)));
        assert_eq!(Char::new('ñ'), Some(Char(241 as u8 as i8)));
        assert_eq!(UChar::new(0 as char), Some(UChar(0)));
        assert_eq!(UChar::new(255 as char), Some(UChar(255)));
        assert_eq!(UChar::new('ñ'), Some(UChar(241 as u8)));
    }

    #[test]
    fn refuses_multibyte_chars() {
        assert_eq!(Char::new('☔'), None); // no umbrella for you
        assert_eq!(UChar::new('☔'), None);
    }

    #[test]
    fn into_i8() {
        assert_eq!(Char::new('A').unwrap().to_glib(), 65i8);
    }

    #[test]
    fn into_u8() {
        assert_eq!(UChar::new('A').unwrap().to_glib(), 65u8);
    }

    #[test]
    fn into_char() {
        assert_eq!(char::from(Char(65i8)), 'A');
        assert_eq!('ñ', UChar(241u8).into());
    }

    #[test]
    fn convert_from_glib() {
        assert_eq!(Char(65i8), from_glib(65i8));
        assert_eq!(UChar(241u8), from_glib(241u8));
    }
}
