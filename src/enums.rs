// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use gobject_ffi;
use translate::*;
use Type;
use CStr;
use value::Value;
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

#[derive(Debug)]
pub struct EnumClass(*mut gobject_ffi::GEnumClass);

impl EnumClass {
    pub fn new(type_: Type) -> Option<Self> {
        unsafe {
            let is_enum: bool = from_glib(gobject_ffi::g_type_is_a(type_.to_glib(), gobject_ffi::G_TYPE_ENUM));
            if !is_enum {
                return None;
            }

            Some(EnumClass(gobject_ffi::g_type_class_ref(type_.to_glib()) as *mut _))
        }
    }

    pub fn type_(&self) -> Type {
        unsafe {
            from_glib((*self.0).g_type_class.g_type)
        }
    }

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

    pub fn get_values(&self) -> Vec<EnumValue> {
        unsafe {
            let n = (*self.0).n_values;
            let mut res = Vec::with_capacity(n as usize);
            for i in 0..n {
                res.push(EnumValue((*self.0).values.offset(i as isize), self.clone()))
            }
            res
        }
    }

    pub fn to_value(&self, value: i32) -> Option<Value> {
        self.get_value(value).map(|v| v.to_value())
    }

    pub fn to_value_by_name(&self, name: &str) -> Option<Value> {
        self.get_value_by_name(name).map(|v| v.to_value())
    }

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
        unsafe {
            EnumClass(gobject_ffi::g_type_class_ref(self.type_().to_glib()) as *mut _)
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumValue(*const gobject_ffi::GEnumValue, EnumClass);

impl EnumValue {
    pub fn get_value(&self) -> i32 {
        unsafe {
            (*self.0).value
        }
    }

    pub fn get_name(&self) -> &str {
        unsafe {
            CStr::from_ptr((*self.0).value_name).to_str().unwrap()
        }
    }

    pub fn get_nick(&self) -> &str {
        unsafe {
            CStr::from_ptr((*self.0).value_nick).to_str().unwrap()
        }
    }

    pub fn to_value(&self) -> Value {
        unsafe {
            let mut v = Value::uninitialized();
            gobject_ffi::g_value_init(v.to_glib_none_mut().0, self.1.type_().to_glib());
            gobject_ffi::g_value_set_enum(v.to_glib_none_mut().0, (*self.0).value);
            v
        }
    }

    pub fn from_value(value: &Value) -> Option<EnumValue> {
        unsafe {
            let enum_class = EnumClass::new(value.type_());
            enum_class.and_then(|e| e.get_value(gobject_ffi::g_value_get_enum(value.to_glib_none().0)))
        }
    }

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

#[derive(Debug)]
pub struct FlagsClass(*mut gobject_ffi::GFlagsClass);

impl FlagsClass {
    pub fn new(type_: Type) -> Option<Self> {
        unsafe {
            let is_flags: bool = from_glib(gobject_ffi::g_type_is_a(type_.to_glib(), gobject_ffi::G_TYPE_FLAGS));
            if !is_flags {
                return None;
            }

            Some(FlagsClass(gobject_ffi::g_type_class_ref(type_.to_glib()) as *mut _))
        }
    }

    pub fn type_(&self) -> Type {
        unsafe {
            from_glib((*self.0).g_type_class.g_type)
        }
    }

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

    pub fn get_values(&self) -> Vec<FlagsValue> {
        unsafe {
            let n = (*self.0).n_values;
            let mut res = Vec::with_capacity(n as usize);
            for i in 0..n {
                res.push(FlagsValue((*self.0).values.offset(i as isize), self.clone()))
            }
            res
        }
    }

    pub fn to_value(&self, value: u32) -> Option<Value> {
        self.get_value(value).map(|v| v.to_value())
    }

    pub fn to_value_by_name(&self, name: &str) -> Option<Value> {
        self.get_value_by_name(name).map(|v| v.to_value())
    }

    pub fn to_value_by_nick(&self, nick: &str) -> Option<Value> {
        self.get_value_by_nick(nick).map(|v| v.to_value())
    }

    pub fn is_set(&self, value: &Value, f: u32) -> bool {
        unsafe {
            if self.type_() != value.type_() {
                return false;
            }

            let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
            flags & f != 0
        }
    }

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

    pub fn builder(&self) -> FlagsBuilder {
        FlagsBuilder::new(self)
    }

    pub fn builder_with_value(&self, value: Value) -> Option<FlagsBuilder> {
        if self.type_() != value.type_() {
            return None;
        }

        Some(FlagsBuilder::new_with_value(self, value))
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
        unsafe {
            FlagsClass(gobject_ffi::g_type_class_ref(self.type_().to_glib()) as *mut _)
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlagsValue(*const gobject_ffi::GFlagsValue, FlagsClass);

impl FlagsValue {
    pub fn get_value(&self) -> u32 {
        unsafe {
            (*self.0).value
        }
    }

    pub fn get_name(&self) -> &str {
        unsafe {
            CStr::from_ptr((*self.0).value_name).to_str().unwrap()
        }
    }

    pub fn get_nick(&self) -> &str {
        unsafe {
            CStr::from_ptr((*self.0).value_nick).to_str().unwrap()
        }
    }

    pub fn to_value(&self) -> Value {
        unsafe {
            let mut v = Value::uninitialized();
            gobject_ffi::g_value_init(v.to_glib_none_mut().0, self.1.type_().to_glib());
            gobject_ffi::g_value_set_flags(v.to_glib_none_mut().0, (*self.0).value);
            v
        }
    }

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

pub struct FlagsBuilder<'a>(&'a FlagsClass, Option<Value>);
impl<'a> FlagsBuilder<'a> {
    fn new(flags_class: &FlagsClass) -> FlagsBuilder {
        let value = unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, flags_class.type_().to_glib());
            value
        };

        FlagsBuilder(flags_class, Some(value))
    }

    fn new_with_value(flags_class: &FlagsClass, value: Value) -> FlagsBuilder {
        FlagsBuilder(flags_class, Some(value))
    }

    pub fn set(mut self, f: u32) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set(value, f).ok();
        }

        self
    }

    pub fn set_by_name(mut self, name: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set_by_name(value, name).ok();
        }

        self
    }

    pub fn set_by_nick(mut self, nick: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set_by_nick(value, nick).ok();
        }

        self
    }

    pub fn unset(mut self, f: u32) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset(value, f).ok();
        }

        self
    }

    pub fn unset_by_name(mut self, name: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset_by_name(value, name).ok();
        }

        self
    }

    pub fn unset_by_nick(mut self, nick: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset_by_nick(value, nick).ok();
        }

        self
    }

    pub fn build(self) -> Option<Value> {
        self.1
    }
}
