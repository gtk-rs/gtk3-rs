// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::value::Value;
use crate::CStr;
use crate::Type;
use std::cmp;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum UserDirectory {
    Desktop,
    Documents,
    Downloads,
    Music,
    Pictures,
    PublicShare,
    Templates,
    Videos,
    #[doc(hidden)]
    NDirectories,
}

#[doc(hidden)]
impl ToGlib for UserDirectory {
    type GlibType = ffi::GUserDirectory;

    fn to_glib(&self) -> ffi::GUserDirectory {
        match *self {
            UserDirectory::Desktop => ffi::G_USER_DIRECTORY_DESKTOP,
            UserDirectory::Documents => ffi::G_USER_DIRECTORY_DOCUMENTS,
            UserDirectory::Downloads => ffi::G_USER_DIRECTORY_DOWNLOAD,
            UserDirectory::Music => ffi::G_USER_DIRECTORY_MUSIC,
            UserDirectory::Pictures => ffi::G_USER_DIRECTORY_PICTURES,
            UserDirectory::PublicShare => ffi::G_USER_DIRECTORY_PUBLIC_SHARE,
            UserDirectory::Templates => ffi::G_USER_DIRECTORY_TEMPLATES,
            UserDirectory::Videos => ffi::G_USER_DIRECTORY_VIDEOS,
            UserDirectory::NDirectories => ffi::G_USER_N_DIRECTORIES,
        }
    }
}

/// Representation of an `enum` for dynamically, at runtime, querying the values of the enum and
/// using them.
#[derive(Debug)]
pub struct EnumClass(*mut gobject_ffi::GEnumClass);

unsafe impl Send for EnumClass {}
unsafe impl Sync for EnumClass {}

impl EnumClass {
    /// Create a new `EnumClass` from a `Type`.
    ///
    /// Returns `None` if `type_` is not representing an enum.
    pub fn new(type_: Type) -> Option<Self> {
        unsafe {
            let is_enum: bool = from_glib(gobject_ffi::g_type_is_a(
                type_.to_glib(),
                gobject_ffi::G_TYPE_ENUM,
            ));
            if !is_enum {
                return None;
            }

            Some(EnumClass(
                gobject_ffi::g_type_class_ref(type_.to_glib()) as *mut _
            ))
        }
    }

    /// `Type` of the enum.
    pub fn type_(&self) -> Type {
        unsafe { from_glib((*self.0).g_type_class.g_type) }
    }

    /// Gets `EnumValue` by integer `value`, if existing.
    ///
    /// Returns `None` if the enum does not contain any value
    /// with `value`.
    #[doc(alias = "g_enum_get_value")]
    pub fn get_value(&self, value: i32) -> Option<EnumValue> {
        unsafe {
            let v = gobject_ffi::g_enum_get_value(self.0, value);
            if v.is_null() {
                None
            } else {
                Some(EnumValue(v, self.clone()))
            }
        }
    }

    /// Gets `EnumValue` by string name `name`, if existing.
    ///
    /// Returns `None` if the enum does not contain any value
    /// with name `name`.
    #[doc(alias = "g_enum_get_value_by_name")]
    pub fn get_value_by_name(&self, name: &str) -> Option<EnumValue> {
        unsafe {
            let v = gobject_ffi::g_enum_get_value_by_name(self.0, name.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(EnumValue(v, self.clone()))
            }
        }
    }

    /// Gets `EnumValue` by string nick `nick`, if existing.
    ///
    /// Returns `None` if the enum does not contain any value
    /// with nick `nick`.
    #[doc(alias = "g_enum_get_value_by_nick")]
    pub fn get_value_by_nick(&self, nick: &str) -> Option<EnumValue> {
        unsafe {
            let v = gobject_ffi::g_enum_get_value_by_nick(self.0, nick.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(EnumValue(v, self.clone()))
            }
        }
    }

    /// Gets all `EnumValue` of this `EnumClass`.
    pub fn get_values(&self) -> Vec<EnumValue> {
        unsafe {
            let n = (*self.0).n_values;
            let mut res = Vec::with_capacity(n as usize);
            for i in 0..(n as usize) {
                res.push(EnumValue((*self.0).values.add(i), self.clone()))
            }
            res
        }
    }

    /// Converts integer `value` to a `Value`, if part of the enum.
    pub fn to_value(&self, value: i32) -> Option<Value> {
        self.get_value(value).map(|v| v.to_value())
    }

    /// Converts string name `name` to a `Value`, if part of the enum.
    pub fn to_value_by_name(&self, name: &str) -> Option<Value> {
        self.get_value_by_name(name).map(|v| v.to_value())
    }

    /// Converts string nick `nick` to a `Value`, if part of the enum.
    pub fn to_value_by_nick(&self, nick: &str) -> Option<Value> {
        self.get_value_by_nick(nick).map(|v| v.to_value())
    }
}

impl Drop for EnumClass {
    fn drop(&mut self) {
        unsafe {
            gobject_ffi::g_type_class_unref(self.0 as *mut _);
        }
    }
}

impl Clone for EnumClass {
    fn clone(&self) -> Self {
        unsafe { EnumClass(gobject_ffi::g_type_class_ref(self.type_().to_glib()) as *mut _) }
    }
}

/// Representation of a single enum value of an `EnumClass`.
#[derive(Debug, Clone)]
pub struct EnumValue(*const gobject_ffi::GEnumValue, EnumClass);

unsafe impl Send for EnumValue {}
unsafe impl Sync for EnumValue {}

impl EnumValue {
    /// Get integer value corresponding to the value.
    pub fn get_value(&self) -> i32 {
        unsafe { (*self.0).value }
    }

    /// Get name corresponding to the value.
    pub fn get_name(&self) -> &str {
        unsafe { CStr::from_ptr((*self.0).value_name).to_str().unwrap() }
    }

    /// Get nick corresponding to the value.
    pub fn get_nick(&self) -> &str {
        unsafe { CStr::from_ptr((*self.0).value_nick).to_str().unwrap() }
    }

    /// Convert enum value to a `Value`.
    pub fn to_value(&self) -> Value {
        unsafe {
            let mut v = Value::from_type(self.1.type_());
            gobject_ffi::g_value_set_enum(v.to_glib_none_mut().0, (*self.0).value);
            v
        }
    }

    /// Convert enum value from a `Value`.
    pub fn from_value(value: &Value) -> Option<EnumValue> {
        unsafe {
            let enum_class = EnumClass::new(value.type_());
            enum_class
                .and_then(|e| e.get_value(gobject_ffi::g_value_get_enum(value.to_glib_none().0)))
        }
    }

    /// Get `EnumClass` to which the enum value belongs.
    pub fn get_class(&self) -> &EnumClass {
        &self.1
    }
}

impl PartialEq for EnumValue {
    fn eq(&self, other: &Self) -> bool {
        self.get_value().eq(&other.get_value())
    }
}

impl Eq for EnumValue {}

impl PartialOrd for EnumValue {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.get_value().partial_cmp(&other.get_value())
    }
}

impl Ord for EnumValue {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

/// Representation of a `flags` for dynamically, at runtime, querying the values of the enum and
/// using them
#[derive(Debug)]
pub struct FlagsClass(*mut gobject_ffi::GFlagsClass);

unsafe impl Send for FlagsClass {}
unsafe impl Sync for FlagsClass {}

impl FlagsClass {
    /// Create a new `FlagsClass` from a `Type`
    ///
    /// Returns `None` if `type_` is not representing a flags type.
    pub fn new(type_: Type) -> Option<Self> {
        unsafe {
            let is_flags: bool = from_glib(gobject_ffi::g_type_is_a(
                type_.to_glib(),
                gobject_ffi::G_TYPE_FLAGS,
            ));
            if !is_flags {
                return None;
            }

            Some(FlagsClass(
                gobject_ffi::g_type_class_ref(type_.to_glib()) as *mut _
            ))
        }
    }

    /// `Type` of the flags.
    pub fn type_(&self) -> Type {
        unsafe { from_glib((*self.0).g_type_class.g_type) }
    }

    /// Gets `FlagsValue` by integer `value`, if existing.
    ///
    /// Returns `None` if the flags do not contain any value
    /// with `value`.
    #[doc(alias = "g_flags_get_first_value")]
    pub fn get_value(&self, value: u32) -> Option<FlagsValue> {
        unsafe {
            let v = gobject_ffi::g_flags_get_first_value(self.0, value);
            if v.is_null() {
                None
            } else {
                Some(FlagsValue(v, self.clone()))
            }
        }
    }

    /// Gets `FlagsValue` by string name `name`, if existing.
    ///
    /// Returns `None` if the flags do not contain any value
    /// with name `name`.
    #[doc(alias = "g_flags_get_value_by_name")]
    pub fn get_value_by_name(&self, name: &str) -> Option<FlagsValue> {
        unsafe {
            let v = gobject_ffi::g_flags_get_value_by_name(self.0, name.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(FlagsValue(v, self.clone()))
            }
        }
    }

    /// Gets `FlagsValue` by string nick `nick`, if existing.
    ///
    /// Returns `None` if the flags do not contain any value
    /// with nick `nick`.
    #[doc(alias = "g_flags_get_value_by_nick")]
    pub fn get_value_by_nick(&self, nick: &str) -> Option<FlagsValue> {
        unsafe {
            let v = gobject_ffi::g_flags_get_value_by_nick(self.0, nick.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(FlagsValue(v, self.clone()))
            }
        }
    }

    /// Gets all `FlagsValue` of this `FlagsClass`.
    pub fn get_values(&self) -> Vec<FlagsValue> {
        unsafe {
            let n = (*self.0).n_values;
            let mut res = Vec::with_capacity(n as usize);
            for i in 0..(n as usize) {
                res.push(FlagsValue((*self.0).values.add(i), self.clone()))
            }
            res
        }
    }

    /// Converts integer `value` to a `Value`, if part of the flags.
    pub fn to_value(&self, value: u32) -> Option<Value> {
        self.get_value(value).map(|v| v.to_value())
    }

    /// Converts string name `name` to a `Value`, if part of the flags.
    pub fn to_value_by_name(&self, name: &str) -> Option<Value> {
        self.get_value_by_name(name).map(|v| v.to_value())
    }

    /// Converts string nick `nick` to a `Value`, if part of the flags.
    pub fn to_value_by_nick(&self, nick: &str) -> Option<Value> {
        self.get_value_by_nick(nick).map(|v| v.to_value())
    }

    /// Checks if the flags corresponding to integer `f` is set in `value`.
    pub fn is_set(&self, value: &Value, f: u32) -> bool {
        unsafe {
            if self.type_() != value.type_() {
                return false;
            }

            let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
            flags & f != 0
        }
    }

    /// Checks if the flags corresponding to string name `name` is set in `value`.
    pub fn is_set_by_name(&self, value: &Value, name: &str) -> bool {
        unsafe {
            if self.type_() != value.type_() {
                return false;
            }

            if let Some(f) = self.get_value_by_name(name) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                flags & f.get_value() != 0
            } else {
                false
            }
        }
    }

    /// Checks if the flags corresponding to string nick `nick` is set in `value`.
    pub fn is_set_by_nick(&self, value: &Value, nick: &str) -> bool {
        unsafe {
            if self.type_() != value.type_() {
                return false;
            }

            if let Some(f) = self.get_value_by_nick(nick) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                flags & f.get_value() != 0
            } else {
                false
            }
        }
    }

    /// Sets flags value corresponding to integer `f` in `value`, if part of that flags. If the
    /// flag is already set, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
    /// value otherwise.
    #[doc(alias = "g_value_set_flags")]
    pub fn set(&self, mut value: Value, f: u32) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.get_value(f) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags | f.get_value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    /// Sets flags value corresponding to string name `name` in `value`, if part of that flags.
    /// If the flag is already set, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn set_by_name(&self, mut value: Value, name: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.get_value_by_name(name) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags | f.get_value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    /// Sets flags value corresponding to string nick `nick` in `value`, if part of that flags.
    /// If the flag is already set, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn set_by_nick(&self, mut value: Value, nick: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.get_value_by_nick(nick) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags | f.get_value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    /// Unsets flags value corresponding to integer `f` in `value`, if part of that flags.
    /// If the flag is already unset, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn unset(&self, mut value: Value, f: u32) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.get_value(f) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags & !f.get_value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    /// Unsets flags value corresponding to string name `name` in `value`, if part of that flags.
    /// If the flag is already unset, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn unset_by_name(&self, mut value: Value, name: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.get_value_by_name(name) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags & !f.get_value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    /// Unsets flags value corresponding to string nick `nick` in `value`, if part of that flags.
    /// If the flag is already unset, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn unset_by_nick(&self, mut value: Value, nick: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.get_value_by_nick(nick) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags & !f.get_value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    /// Returns a new `FlagsBuilder` for conveniently setting/unsetting flags
    /// and building a `Value`.
    pub fn builder(&self) -> FlagsBuilder {
        FlagsBuilder::new(self)
    }

    /// Returns a new `FlagsBuilder` for conveniently setting/unsetting flags
    /// and building a `Value`. The `Value` is initialized with `value`.
    pub fn builder_with_value(&self, value: Value) -> Option<FlagsBuilder> {
        if self.type_() != value.type_() {
            return None;
        }

        Some(FlagsBuilder::with_value(self, value))
    }
}

impl Drop for FlagsClass {
    fn drop(&mut self) {
        unsafe {
            gobject_ffi::g_type_class_unref(self.0 as *mut _);
        }
    }
}

impl Clone for FlagsClass {
    fn clone(&self) -> Self {
        unsafe { FlagsClass(gobject_ffi::g_type_class_ref(self.type_().to_glib()) as *mut _) }
    }
}

/// Representation of a single flags value of a `FlagsClass`.
#[derive(Debug, Clone)]
pub struct FlagsValue(*const gobject_ffi::GFlagsValue, FlagsClass);

unsafe impl Send for FlagsValue {}
unsafe impl Sync for FlagsValue {}

impl FlagsValue {
    /// Get integer value corresponding to the value.
    pub fn get_value(&self) -> u32 {
        unsafe { (*self.0).value }
    }

    /// Get name corresponding to the value.
    pub fn get_name(&self) -> &str {
        unsafe { CStr::from_ptr((*self.0).value_name).to_str().unwrap() }
    }

    /// Get nick corresponding to the value.
    pub fn get_nick(&self) -> &str {
        unsafe { CStr::from_ptr((*self.0).value_nick).to_str().unwrap() }
    }

    /// Convert flags value to a `Value`.
    pub fn to_value(&self) -> Value {
        unsafe {
            let mut v = Value::from_type(self.1.type_());
            gobject_ffi::g_value_set_flags(v.to_glib_none_mut().0, (*self.0).value);
            v
        }
    }

    /// Convert flags values from a `Value`. This returns all flags that are set.
    pub fn from_value(value: &Value) -> Vec<FlagsValue> {
        unsafe {
            let flags_class = FlagsClass::new(value.type_());
            let mut res = Vec::new();
            if let Some(flags_class) = flags_class {
                let f = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                for v in flags_class.get_values() {
                    if v.get_value() & f != 0 {
                        res.push(v);
                    }
                }
            }
            res
        }
    }

    /// Get `FlagsClass` to which the flags value belongs.
    pub fn get_class(&self) -> &FlagsClass {
        &self.1
    }
}

impl PartialEq for FlagsValue {
    fn eq(&self, other: &Self) -> bool {
        self.get_value().eq(&other.get_value())
    }
}

impl Eq for FlagsValue {}

/// Builder for conveniently setting/unsetting flags and returning a `Value`.
///
/// Example for getting a flags property, unsetting some flags and setting the updated flags on the
/// object again:
///
/// ```ignore
/// let flags = obj.get_property("flags").unwrap();
/// let flags_class = FlagsClass::new(flags.type_()).unwrap();
/// let flags = flags_class.builder_with_value(flags).unwrap()
///     .unset_by_nick("some-flag")
///     .unset_by_nick("some-other-flag")
///     .build()
///     .unwrap();
/// obj.set_property("flags", &flags).unwrap();
/// ```
///
/// If setting/unsetting any value fails, `build()` returns `None`.
pub struct FlagsBuilder<'a>(&'a FlagsClass, Option<Value>);
impl<'a> FlagsBuilder<'a> {
    fn new(flags_class: &FlagsClass) -> FlagsBuilder {
        let value = Value::from_type(flags_class.type_());
        FlagsBuilder(flags_class, Some(value))
    }

    fn with_value(flags_class: &FlagsClass, value: Value) -> FlagsBuilder {
        FlagsBuilder(flags_class, Some(value))
    }

    /// Sets flags corresponding to integer value `f`.
    pub fn set(mut self, f: u32) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set(value, f).ok();
        }

        self
    }

    /// Sets flags corresponding to string name `name`.
    pub fn set_by_name(mut self, name: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set_by_name(value, name).ok();
        }

        self
    }

    /// Sets flags corresponding to string nick `nick`.
    pub fn set_by_nick(mut self, nick: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set_by_nick(value, nick).ok();
        }

        self
    }

    /// Unsets flags corresponding to integer value `f`.
    pub fn unset(mut self, f: u32) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset(value, f).ok();
        }

        self
    }

    /// Unsets flags corresponding to string name `name`.
    pub fn unset_by_name(mut self, name: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset_by_name(value, name).ok();
        }

        self
    }

    /// Unsets flags corresponding to string nick `nick`.
    pub fn unset_by_nick(mut self, nick: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset_by_nick(value, nick).ok();
        }

        self
    }

    /// Converts to the final `Value`, unless any previous setting/unsetting of flags failed.
    pub fn build(self) -> Option<Value> {
        self.1
    }
}
