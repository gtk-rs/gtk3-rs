// Take a look at the license at the top of the repository in the LICENSE file.

//! Module for registering boxed types for Rust types.

use crate::translate::*;

/// Trait for defining boxed types.
///
/// Links together the type name with the type itself.
///
/// See [`register_boxed_type`] for registering an implementation of this trait
/// with the type system.
///
/// [`register_boxed_type`]: fn.register_boxed_type.html
pub trait BoxedType: Clone + Sized + 'static {
    /// Boxed type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;

    /// Returns the type ID.
    ///
    /// This is usually defined via the [`GBoxed!`] derive macro.
    ///
    /// [`GBoxed!`]: ../../derive.GBoxed.html
    fn get_type() -> crate::Type;
}

/// Register a boxed `glib::Type` ID for `T`.
///
/// This must be called only once and will panic on a second call.
///
/// See [`GBoxed!`] for defining a function that ensures that
/// this is only called once and returns the type id.
///
/// [`GBoxed!`]: ../../derive.GBoxed.html
pub fn register_boxed_type<T: BoxedType>() -> crate::Type {
    unsafe extern "C" fn boxed_copy<T: BoxedType>(v: ffi::gpointer) -> ffi::gpointer {
        let v = &*(v as *mut T);
        let copy = Box::new(v.clone());

        Box::into_raw(copy) as ffi::gpointer
    }
    unsafe extern "C" fn boxed_free<T: BoxedType>(v: ffi::gpointer) {
        let v = v as *mut T;
        let _ = Box::from_raw(v);
    }
    unsafe {
        use std::ffi::CString;

        let type_name = CString::new(T::NAME).unwrap();
        if gobject_ffi::g_type_from_name(type_name.as_ptr()) != gobject_ffi::G_TYPE_INVALID {
            panic!(
                "Type {} has already been registered",
                type_name.to_str().unwrap()
            );
        }

        from_glib(gobject_ffi::g_boxed_type_register_static(
            type_name.as_ptr(),
            Some(boxed_copy::<T>),
            Some(boxed_free::<T>),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // GBoxed macro assumes 'glib' is in scope
    use crate as glib;
    use crate::value::ToValue;

    #[derive(Clone, Debug, PartialEq, Eq, glib::GBoxed)]
    #[gboxed(type_name = "MyBoxed")]
    struct MyBoxed(String);

    #[test]
    fn test_register() {
        assert_ne!(crate::Type::Invalid, MyBoxed::get_type());
    }

    #[test]
    fn test_value() {
        assert_ne!(crate::Type::Invalid, MyBoxed::get_type());

        let b = MyBoxed(String::from("abc"));
        let v = b.to_value();
        let b2 = v.get_some::<&MyBoxed>().unwrap();
        assert_eq!(&b, b2);
    }
}
