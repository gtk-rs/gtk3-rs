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
/// by calling `i8::from(my_char)`.
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
    /// have_a_byte(i8::from(a));
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

impl From<Char> for i8 {
    fn from(c: Char) -> i8 {
        c.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_single_byte_chars() {
        assert_eq!(Char::new(0 as char), Some(Char(0)));
        assert_eq!(Char::new(255 as char), Some(Char(-1)));
        assert_eq!(Char::new('ñ'), Some(Char(241 as u8 as i8)));
    }

    #[test]
    fn refuses_multibyte_chars() {
        assert_eq!(Char::new('☔'), None); // no umbrella for you
    }

    #[test]
    fn into_i8() {
        assert_eq!(i8::from(Char::new('A').unwrap()), 65i8);
    }
}
