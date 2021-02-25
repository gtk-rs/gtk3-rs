// Take a look at the license at the top of the repository in the LICENSE file.

//! `Variant` binding and helper traits.
//!
//! [`Variant`](struct.Variant.html) is an immutable dynamically-typed generic
//! container. Its type and value are defined at construction and never change.
//!
//! `Variant` types are described by [`VariantType`](../struct.VariantType.html)
//! "type strings".
//!
//! Although `GVariant` supports arbitrarily complex types, this binding is
//! currently limited to the basic ones: `bool`, `u8`, `i16`, `u16`, `i32`,
//! `u32`, `i64`, `u64`, `f64`, `&str`/`String`, and [`VariantDict`](../struct.VariantDict.html).
//!
//! # Examples
//!
//! ```
//! use glib::prelude::*; // or `use gtk::prelude::*;`
//! use glib::{Variant, FromVariant, ToVariant};
//! use std::collections::HashMap;
//!
//! // Using the `ToVariant` trait.
//! let num = 10.to_variant();
//!
//! // `is` tests the type of the value.
//! assert!(num.is::<i32>());
//!
//! // `get` tries to extract the value.
//! assert_eq!(num.get::<i32>(), Some(10));
//! assert_eq!(num.get::<u32>(), None);
//!
//! // `get_str` tries to borrow a string slice.
//! let hello = "Hello!".to_variant();
//! assert_eq!(hello.get_str(), Some("Hello!"));
//! assert_eq!(num.get_str(), None);
//!
//! // Variant carrying a Variant
//! let variant = Variant::variant(&hello);
//! let variant = variant.get_variant().unwrap();
//! assert_eq!(variant.get_str(), Some("Hello!"));
//!
//! // Variant carrying an array
//! let array = ["Hello".to_variant(), "there!".to_variant()];
//! let variant = Variant::array::<&str>(&array);
//! assert_eq!(variant.n_children(), 2);
//! assert_eq!(variant.get_child_value(0).get_str(), Some("Hello"));
//! assert_eq!(variant.get_child_value(1).get_str(), Some("there!"));
//!
//! // You can also convert from and to a Vec
//! let array = vec!["Hello", "there!"].to_variant();
//! assert_eq!(variant.n_children(), 2);
//! let vec = <Vec<String>>::from_variant(&array).unwrap();
//! assert_eq!(vec[0], "Hello");
//!
//! // Conversion to and from HashMap is also possible
//! let mut map: HashMap<u16, &str> = HashMap::new();
//! map.insert(1, "hi");
//! map.insert(2, "there");
//! let variant = map.to_variant();
//! assert_eq!(variant.n_children(), 2);
//! let map: HashMap<u16, String> = HashMap::from_variant(&variant).unwrap();
//! assert_eq!(map[&1], "hi");
//! assert_eq!(map[&2], "there");
//!
//! // And conversion to and from tuples.
//! let variant = ("hello", 42u16, vec![ "there", "you" ],).to_variant();
//! assert_eq!(variant.n_children(), 3);
//! assert_eq!(variant.type_().to_str(), "(sqas)");
//! let tuple = <(String, u16, Vec<String>)>::from_variant(&variant).unwrap();
//! assert_eq!(tuple.0, "hello");
//! assert_eq!(tuple.1, 42);
//! assert_eq!(tuple.2, &[ "there", "you"]);
//!
//! // `Option` is supported as well, through maybe types
//! let variant = Some("hello").to_variant();
//! assert_eq!(variant.n_children(), 1);
//! let mut s = <Option<String>>::from_variant(&variant).unwrap();
//! assert_eq!(s.unwrap(), "hello");
//! s = None;
//! let variant = s.to_variant();
//! assert_eq!(variant.n_children(), 0);
//! let s = <Option<String>>::from_variant(&variant).unwrap();
//! assert!(s.is_none());
//! ```

use crate::bytes::Bytes;
use crate::gstring::GString;
use crate::translate::*;
use crate::value;
use crate::StaticType;
use crate::Type;
use crate::Value;
use crate::VariantIter;
use crate::VariantTy;
use crate::VariantType;
use std::borrow::Cow;
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::fmt;
use std::hash::{BuildHasher, Hash, Hasher};
use std::slice;
use std::str;

wrapper! {
    /// A generic immutable value capable of carrying various types.
    ///
    /// See the [module documentation](index.html) for more details.
    pub struct Variant(Shared<ffi::GVariant>);

    match fn {
        ref => |ptr| ffi::g_variant_ref_sink(ptr),
        unref => |ptr| ffi::g_variant_unref(ptr),
    }
}

impl StaticType for Variant {
    fn static_type() -> Type {
        Type::VARIANT
    }
}

#[doc(hidden)]
impl<'a> value::FromValueOptional<'a> for Variant {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        from_glib_full(gobject_ffi::g_value_dup_variant(
            ToGlibPtr::to_glib_none(value).0,
        ))
    }
}

#[doc(hidden)]
impl value::SetValue for Variant {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_variant(
            ToGlibPtrMut::to_glib_none_mut(value).0,
            ToGlibPtr::<*mut ffi::GVariant>::to_glib_none(this).0,
        )
    }
}

#[doc(hidden)]
impl value::SetValueOptional for Variant {
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_variant(
            ToGlibPtrMut::to_glib_none_mut(value).0,
            ToGlibPtr::<*mut ffi::GVariant>::to_glib_none(&this).0,
        )
    }
}

impl Variant {
    /// Returns the type of the value.
    pub fn type_(&self) -> &VariantTy {
        unsafe { VariantTy::from_ptr(ffi::g_variant_get_type(self.to_glib_none().0)) }
    }

    /// Returns `true` if the type of the value corresponds to `T`.
    #[inline]
    pub fn is<T: StaticVariantType>(&self) -> bool {
        self.type_() == T::static_variant_type()
    }

    /// Tries to extract a value of type `T`.
    ///
    /// Returns `Some` if `T` matches the variant's type.
    #[inline]
    pub fn get<T: FromVariant>(&self) -> Option<T> {
        T::from_variant(self)
    }

    /// Boxes value.
    #[inline]
    pub fn variant(value: &Variant) -> Self {
        unsafe { from_glib_none(ffi::g_variant_new_variant(value.to_glib_none().0)) }
    }

    /// Unboxes self.
    ///
    /// Returns `Some` if self contains a `Variant`.
    #[inline]
    pub fn get_variant(&self) -> Option<Variant> {
        unsafe { from_glib_none(ffi::g_variant_get_variant(self.to_glib_none().0)) }
    }

    /// Reads a child item out of a container `Variant` instance.
    ///
    /// # Panics
    ///
    /// * if `self` is not a container type.
    /// * if given `index` is larger than number of children.
    pub fn get_child_value(&self, index: usize) -> Variant {
        assert!(index < self.n_children());
        assert!(self.is_container());

        unsafe { from_glib_none(ffi::g_variant_get_child_value(self.to_glib_none().0, index)) }
    }

    /// Tries to extract a `&str`.
    ///
    /// Returns `Some` if the variant has a string type (`s`, `o` or `g` type
    /// strings).
    pub fn get_str(&self) -> Option<&str> {
        unsafe {
            match self.type_().to_str() {
                "s" | "o" | "g" => {
                    let mut len = 0;
                    let ptr = ffi::g_variant_get_string(self.to_glib_none().0, &mut len);
                    let ret = str::from_utf8_unchecked(slice::from_raw_parts(
                        ptr as *const u8,
                        len as usize,
                    ));
                    Some(ret)
                }
                _ => None,
            }
        }
    }

    /// Creates a new GVariant array from children.
    ///
    /// All children must be of type `T`.
    pub fn array<T: StaticVariantType>(children: &[Variant]) -> Self {
        let type_ = T::static_variant_type();

        for child in children {
            assert_eq!(type_, child.type_());
        }
        unsafe {
            from_glib_none(ffi::g_variant_new_array(
                type_.as_ptr() as *const _,
                children.to_glib_none().0,
                children.len(),
            ))
        }
    }

    /// Creates a new GVariant tuple from children.
    pub fn tuple(children: &[Variant]) -> Self {
        unsafe {
            from_glib_none(ffi::g_variant_new_tuple(
                children.to_glib_none().0,
                children.len(),
            ))
        }
    }

    /// Creates a new maybe Variant.
    pub fn maybe<T: StaticVariantType>(child: Option<&Variant>) -> Self {
        let type_ = T::static_variant_type();
        let ptr = match child {
            Some(child) => {
                assert_eq!(type_, child.type_());

                child.to_glib_none().0
            }
            None => std::ptr::null(),
        };
        unsafe {
            from_glib_none(ffi::g_variant_new_maybe(
                type_.as_ptr() as *const _,
                ptr as *mut ffi::GVariant,
            ))
        }
    }

    /// Constructs a new serialised-mode GVariant instance.
    pub fn from_bytes<T: StaticVariantType>(bytes: &Bytes) -> Self {
        unsafe {
            from_glib_none(ffi::g_variant_new_from_bytes(
                T::static_variant_type().as_ptr() as *const _,
                bytes.to_glib_none().0,
                false.to_glib(),
            ))
        }
    }

    /// Constructs a new serialised-mode GVariant instance.
    ///
    /// This is the same as `from_bytes`, except that checks on the passed
    /// data are skipped.
    ///
    /// You should not use this function on data from external sources.
    ///
    /// # Safety
    ///
    /// Since the data is not validated, this is potentially dangerous if called
    /// on bytes which are not guaranteed to have come from serialising another
    /// Variant.  The caller is responsible for ensuring bad data is not passed in.
    pub unsafe fn from_bytes_trusted<T: StaticVariantType>(bytes: &Bytes) -> Self {
        from_glib_none(ffi::g_variant_new_from_bytes(
            T::static_variant_type().as_ptr() as *const _,
            bytes.to_glib_none().0,
            true.to_glib(),
        ))
    }

    /// Returns the serialised form of a GVariant instance.
    pub fn get_data_as_bytes(&self) -> Bytes {
        unsafe { from_glib_full(ffi::g_variant_get_data_as_bytes(self.to_glib_none().0)) }
    }

    /// Determines the number of children in a container GVariant instance.
    pub fn n_children(&self) -> usize {
        assert!(self.is_container());

        unsafe { ffi::g_variant_n_children(self.to_glib_none().0) }
    }

    /// Create an iterator over items in the variant.
    pub fn iter(&self) -> VariantIter {
        assert!(self.is_container());

        VariantIter::new(self.clone())
    }

    /// Variant has a container type.
    pub fn is_container(&self) -> bool {
        unsafe { ffi::g_variant_is_container(self.to_glib_none().0) != ffi::GFALSE }
    }
}

unsafe impl Send for Variant {}
unsafe impl Sync for Variant {}

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Variant")
            .field("ptr", &self.to_glib_none().0)
            .field("type", &self.type_())
            .field("value", &self.to_string())
            .finish()
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let serialized: GString =
            unsafe { from_glib_full(ffi::g_variant_print(self.to_glib_none().0, false.to_glib())) };
        f.write_str(&serialized)
    }
}

impl PartialEq for Variant {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::g_variant_equal(
                self.to_glib_none().0 as *const _,
                other.to_glib_none().0 as *const _,
            ))
        }
    }
}

impl Eq for Variant {}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe {
            if ffi::g_variant_classify(self.to_glib_none().0)
                != ffi::g_variant_classify(other.to_glib_none().0)
            {
                return None;
            }

            if self.is_container() {
                return None;
            }

            let res = ffi::g_variant_compare(
                self.to_glib_none().0 as *const _,
                other.to_glib_none().0 as *const _,
            );

            Some(res.cmp(&0))
        }
    }
}

impl Hash for Variant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { state.write_u32(ffi::g_variant_hash(self.to_glib_none().0 as *const _)) }
    }
}

/// Converts to `Variant`.
pub trait ToVariant {
    /// Returns a `Variant` clone of `self`.
    fn to_variant(&self) -> Variant;
}

/// Extracts a value.
pub trait FromVariant: Sized + StaticVariantType {
    /// Tries to extract a value.
    ///
    /// Returns `Some` if the variant's type matches `Self`.
    fn from_variant(variant: &Variant) -> Option<Self>;
}

/// Returns `VariantType` of `Self`.
pub trait StaticVariantType {
    /// Returns the `VariantType` corresponding to `Self`.
    fn static_variant_type() -> Cow<'static, VariantTy>;
}

impl StaticVariantType for Variant {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe { VariantTy::from_str_unchecked("v").into() }
    }
}

impl<'a, T: ?Sized + ToVariant> ToVariant for &'a T {
    fn to_variant(&self) -> Variant {
        <T as ToVariant>::to_variant(self)
    }
}

impl<'a, T: ?Sized + StaticVariantType> StaticVariantType for &'a T {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <T as StaticVariantType>::static_variant_type()
    }
}

macro_rules! impl_numeric {
    ($name:ty, $type_str:expr, $new_fn:ident, $get_fn:ident) => {
        impl StaticVariantType for $name {
            fn static_variant_type() -> Cow<'static, VariantTy> {
                unsafe { VariantTy::from_str_unchecked($type_str).into() }
            }
        }

        impl ToVariant for $name {
            fn to_variant(&self) -> Variant {
                unsafe { from_glib_none(ffi::$new_fn(*self)) }
            }
        }

        impl FromVariant for $name {
            fn from_variant(variant: &Variant) -> Option<Self> {
                unsafe {
                    if variant.is::<Self>() {
                        Some(ffi::$get_fn(variant.to_glib_none().0))
                    } else {
                        None
                    }
                }
            }
        }
    };
}

impl_numeric!(u8, "y", g_variant_new_byte, g_variant_get_byte);
impl_numeric!(i16, "n", g_variant_new_int16, g_variant_get_int16);
impl_numeric!(u16, "q", g_variant_new_uint16, g_variant_get_uint16);
impl_numeric!(i32, "i", g_variant_new_int32, g_variant_get_int32);
impl_numeric!(u32, "u", g_variant_new_uint32, g_variant_get_uint32);
impl_numeric!(i64, "x", g_variant_new_int64, g_variant_get_int64);
impl_numeric!(u64, "t", g_variant_new_uint64, g_variant_get_uint64);
impl_numeric!(f64, "d", g_variant_new_double, g_variant_get_double);

impl StaticVariantType for bool {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe { VariantTy::from_str_unchecked("b").into() }
    }
}

impl ToVariant for bool {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(ffi::g_variant_new_boolean(self.to_glib())) }
    }
}

impl FromVariant for bool {
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            if variant.is::<Self>() {
                Some(from_glib(ffi::g_variant_get_boolean(
                    variant.to_glib_none().0,
                )))
            } else {
                None
            }
        }
    }
}

impl StaticVariantType for String {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe { VariantTy::from_str_unchecked("s").into() }
    }
}

impl ToVariant for String {
    fn to_variant(&self) -> Variant {
        self[..].to_variant()
    }
}

impl FromVariant for String {
    fn from_variant(variant: &Variant) -> Option<Self> {
        variant.get_str().map(String::from)
    }
}

impl StaticVariantType for str {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        unsafe { VariantTy::from_str_unchecked("s").into() }
    }
}

impl ToVariant for str {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(ffi::g_variant_new_take_string(self.to_glib_full())) }
    }
}

impl<T: StaticVariantType> StaticVariantType for Option<T> {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        let child_type = T::static_variant_type();
        let signature = format!("m{}", child_type.to_str());

        VariantType::new(&signature)
            .expect("incorrect signature")
            .into()
    }
}

impl<T: StaticVariantType + ToVariant> ToVariant for Option<T> {
    fn to_variant(&self) -> Variant {
        Variant::maybe::<T>(self.as_ref().map(|m| m.to_variant()).as_ref())
    }
}

impl<T: StaticVariantType + FromVariant> FromVariant for Option<T> {
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            if variant.is::<Self>() {
                let c_child = ffi::g_variant_get_maybe(variant.to_glib_none().0);
                if !c_child.is_null() {
                    let child: Variant = from_glib_full(c_child);

                    Some(T::from_variant(&child))
                } else {
                    Some(None)
                }
            } else {
                None
            }
        }
    }
}

impl<T: StaticVariantType> StaticVariantType for [T] {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        let child_type = T::static_variant_type();
        let signature = format!("a{}", child_type.to_str());

        VariantType::new(&signature)
            .expect("incorrect signature")
            .into()
    }
}

impl<T: FromVariant> FromVariant for Vec<T> {
    fn from_variant(variant: &Variant) -> Option<Self> {
        let mut vec = Vec::with_capacity(variant.n_children());

        for i in 0..variant.n_children() {
            match variant.get_child_value(i).get() {
                Some(child) => vec.push(child),
                None => return None,
            }
        }

        Some(vec)
    }
}

impl<T: StaticVariantType + ToVariant> ToVariant for Vec<T> {
    fn to_variant(&self) -> Variant {
        let mut vec = Vec::with_capacity(self.len());
        for child in self {
            vec.push(child.to_variant());
        }
        Variant::array::<T>(&vec)
    }
}

impl<T: StaticVariantType> StaticVariantType for Vec<T> {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <[T]>::static_variant_type()
    }
}

impl<K, V, H> FromVariant for HashMap<K, V, H>
where
    K: FromVariant + Eq + Hash,
    V: FromVariant,
    H: BuildHasher + Default,
{
    fn from_variant(variant: &Variant) -> Option<Self> {
        let mut map = HashMap::default();

        for i in 0..variant.n_children() {
            let entry = variant.get_child_value(i);
            let key = match entry.get_child_value(0).get() {
                Some(key) => key,
                None => return None,
            };
            let val = match entry.get_child_value(1).get() {
                Some(val) => val,
                None => return None,
            };

            map.insert(key, val);
        }

        Some(map)
    }
}

impl<K, V> ToVariant for HashMap<K, V>
where
    K: StaticVariantType + ToVariant + Eq + Hash,
    V: StaticVariantType + ToVariant,
{
    fn to_variant(&self) -> Variant {
        let mut vec = Vec::with_capacity(self.len());
        for (key, value) in self {
            let entry = DictEntry::new(key, value).to_variant();
            vec.push(entry);
        }
        Variant::array::<DictEntry<K, V>>(&vec)
    }
}

/// A Dictionary entry.
///
/// While GVariant format allows a dictionary entry to be an independent type, typically you'll need
/// to use this in a dictionary, which is simply an array of dictionary entries. The following code
/// creates a dictionary:
///
/// ```
///# use glib::prelude::*; // or `use gtk::prelude::*;`
/// use glib::{Variant, FromVariant, ToVariant};
/// use glib::variant::DictEntry;
///
/// let entries = vec![
///     DictEntry::new("uuid", 1000u32).to_variant(),
///     DictEntry::new("guid", 1001u32).to_variant(),
/// ];
/// let dict = Variant::array::<DictEntry<&str, u32>>(&entries);
/// assert_eq!(dict.n_children(), 2);
/// assert_eq!(dict.type_().to_str(), "a{su}");
/// ```
pub struct DictEntry<K, V> {
    key: K,
    value: V,
}

impl<K, V> DictEntry<K, V>
where
    K: StaticVariantType + ToVariant + Eq + Hash,
    V: StaticVariantType + ToVariant,
{
    pub fn new(key: K, value: V) -> Self {
        DictEntry { key, value }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}

impl<K, V> FromVariant for DictEntry<K, V>
where
    K: FromVariant + Eq + Hash,
    V: FromVariant,
{
    fn from_variant(variant: &Variant) -> Option<Self> {
        let key = match variant.get_child_value(0).get() {
            Some(key) => key,
            None => return None,
        };
        let value = match variant.get_child_value(1).get() {
            Some(value) => value,
            None => return None,
        };

        Some(DictEntry { key, value })
    }
}

impl<K, V> ToVariant for DictEntry<K, V>
where
    K: StaticVariantType + ToVariant + Eq + Hash,
    V: StaticVariantType + ToVariant,
{
    fn to_variant(&self) -> Variant {
        unsafe {
            from_glib_none(ffi::g_variant_new_dict_entry(
                self.key.to_variant().to_glib_none().0,
                self.value.to_variant().to_glib_none().0,
            ))
        }
    }
}

impl ToVariant for Variant {
    fn to_variant(&self) -> Variant {
        Variant::variant(self)
    }
}

impl FromVariant for Variant {
    fn from_variant(variant: &Variant) -> Option<Self> {
        variant.get_variant()
    }
}

impl<K: StaticVariantType, V: StaticVariantType> StaticVariantType for DictEntry<K, V> {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        let key_type = K::static_variant_type();
        let value_type = V::static_variant_type();
        let signature = format!("{{{}{}}}", key_type.to_str(), value_type.to_str());

        VariantType::new(&signature)
            .expect("incorrect signature")
            .into()
    }
}

impl<K, V, H> StaticVariantType for HashMap<K, V, H>
where
    K: StaticVariantType,
    V: StaticVariantType,
    H: BuildHasher + Default,
{
    fn static_variant_type() -> Cow<'static, VariantTy> {
        let key_type = K::static_variant_type();
        let value_type = V::static_variant_type();
        let signature = format!("a{{{}{}}}", key_type.to_str(), value_type.to_str());

        VariantType::new(&signature)
            .expect("incorrect signature")
            .into()
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> StaticVariantType for ($($name,)+)
            where
                $($name: StaticVariantType,)+
            {
                fn static_variant_type() -> Cow<'static, VariantTy> {
                    let mut signature = String::with_capacity(255);
                    signature.push('(');
                    $(
                        signature.push_str($name::static_variant_type().to_str());
                    )+
                    signature.push(')');

                    VariantType::new(&signature).expect("incorrect signature").into()
                }
            }

            impl<$($name),+> FromVariant for ($($name,)+)
            where
                $($name: FromVariant,)+
            {
                fn from_variant(variant: &Variant) -> Option<Self> {
                    Some((
                        $(
                            match $name::from_variant(&variant.get_child_value($n)) {
                                Some(field) => field,
                                None => return None,
                            },
                        )+
                    ))
                }
            }

            impl<$($name),+> ToVariant for ($($name,)+)
            where
                $($name: ToVariant,)+
            {
                fn to_variant(&self) -> Variant {
                    let mut fields = Vec::with_capacity($len);
                    $(
                        let field = self.$n.to_variant();
                        fields.push(field);
                    )+
                    Variant::tuple(&fields)
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    macro_rules! unsigned {
        ($name:ident, $ty:ident) => {
            #[test]
            fn $name() {
                let mut n = $ty::max_value();
                while n > 0 {
                    let v = n.to_variant();
                    assert_eq!(v.get(), Some(n));
                    n /= 2;
                }
            }
        };
    }

    macro_rules! signed {
        ($name:ident, $ty:ident) => {
            #[test]
            fn $name() {
                let mut n = $ty::max_value();
                while n > 0 {
                    let v = n.to_variant();
                    assert_eq!(v.get(), Some(n));
                    let v = (-n).to_variant();
                    assert_eq!(v.get(), Some(-n));
                    n /= 2;
                }
            }
        };
    }

    unsigned!(test_u8, u8);
    unsigned!(test_u16, u16);
    unsigned!(test_u32, u32);
    unsigned!(test_u64, u64);
    signed!(test_i16, i16);
    signed!(test_i32, i32);
    signed!(test_i64, i64);

    #[test]
    fn test_str() {
        let s = "this is a test";
        let v = s.to_variant();
        assert_eq!(v.get_str(), Some(s));
    }

    #[test]
    fn test_string() {
        let s = String::from("this is a test");
        let v = s.to_variant();
        assert_eq!(v.get(), Some(s));
    }

    #[test]
    fn test_eq() {
        let v1 = "this is a test".to_variant();
        let v2 = "this is a test".to_variant();
        let v3 = "test".to_variant();
        assert_eq!(v1, v2);
        assert!(v1 != v3);
    }

    #[test]
    fn test_hash() {
        let v1 = "this is a test".to_variant();
        let v2 = "this is a test".to_variant();
        let v3 = "test".to_variant();
        let mut set = HashSet::new();
        set.insert(v1);
        assert!(set.contains(&v2));
        assert!(!set.contains(&v3));

        assert_eq!(
            <HashMap<&str, (&str, u8, u32)>>::static_variant_type().to_str(),
            "a{s(syu)}"
        );
    }

    #[test]
    fn test_array() {
        // Test just the signature for now.
        assert_eq!(<Vec<&str>>::static_variant_type().to_str(), "as");
        assert_eq!(
            <Vec<(&str, u8, u32)>>::static_variant_type().to_str(),
            "a(syu)"
        );
    }
}
