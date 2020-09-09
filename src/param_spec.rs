// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>
use gobject_sys;
use libc;
use translate::*;
use value;
use ParamFlags;
use StaticType;
use Type;
use Value;

use std::ffi::CStr;

// Can't use get_type here as this is not a boxed type but another fundamental type
glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ParamSpec(Shared<gobject_sys::GParamSpec>);

    match fn {
        ref => |ptr| gobject_sys::g_param_spec_ref_sink(ptr),
        unref => |ptr| gobject_sys::g_param_spec_unref(ptr),
    }
}

impl StaticType for ParamSpec {
    fn static_type() -> Type {
        from_glib(gobject_sys::G_TYPE_PARAM)
    }
}

#[doc(hidden)]
impl<'a> value::FromValueOptional<'a> for ParamSpec {
    #[allow(clippy::missing_safety_doc)]
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        from_glib_full(gobject_sys::g_value_dup_param(value.to_glib_none().0))
    }
}

#[doc(hidden)]
impl value::SetValue for ParamSpec {
    #[allow(clippy::missing_safety_doc)]
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_param(value.to_glib_none_mut().0, this.to_glib_none().0)
    }
}

#[doc(hidden)]
impl value::SetValueOptional for ParamSpec {
    #[allow(clippy::missing_safety_doc)]
    unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
        gobject_sys::g_value_set_param(value.to_glib_none_mut().0, this.to_glib_none().0)
    }
}

unsafe impl Send for ParamSpec {}
unsafe impl Sync for ParamSpec {}

impl ParamSpec {
    pub fn downcast<T: ParamSpecType>(self) -> Result<T, ParamSpec> {
        unsafe {
            if self.get_type() == T::static_type() {
                Ok(from_glib_full(self.to_glib_full()))
            } else {
                Err(self)
            }
        }
    }

    pub fn downcast_ref<T: ParamSpecType>(&self) -> Option<&T> {
        unsafe {
            if self.get_type() == T::static_type() {
                Some(&*(self as *const ParamSpec as *const T))
            } else {
                None
            }
        }
    }

    pub fn get_type(&self) -> Type {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib((*(*ptr).g_type_instance.g_class).g_type)
        }
    }

    pub fn get_value_type(&self) -> ::Type {
        unsafe { from_glib((*self.to_glib_none().0).value_type) }
    }

    pub fn get_owner_type(&self) -> ::Type {
        unsafe { from_glib((*self.to_glib_none().0).owner_type) }
    }

    pub fn get_flags(&self) -> ParamFlags {
        unsafe { from_glib((*self.to_glib_none().0).flags) }
    }

    pub fn get_blurb(&self) -> &str {
        unsafe {
            CStr::from_ptr(gobject_sys::g_param_spec_get_blurb(self.to_glib_none().0))
                .to_str()
                .unwrap()
        }
    }

    pub fn get_default_value(&self) -> &Value {
        unsafe {
            &*(gobject_sys::g_param_spec_get_default_value(self.to_glib_none().0) as *const ::Value)
        }
    }

    pub fn get_name<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(gobject_sys::g_param_spec_get_name(self.to_glib_none().0))
                .to_str()
                .unwrap()
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    pub fn get_name_quark(&self) -> ::Quark {
        unsafe {
            from_glib(gobject_sys::g_param_spec_get_name_quark(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_nick(&self) -> &str {
        unsafe {
            CStr::from_ptr(gobject_sys::g_param_spec_get_nick(self.to_glib_none().0))
                .to_str()
                .unwrap()
        }
    }

    //pub fn get_qdata(&self, quark: /*Ignored*/glib::Quark) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gobject_sys::g_param_spec_get_qdata() }
    //}

    pub fn get_redirect_target(&self) -> Option<ParamSpec> {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_get_redirect_target(
                self.to_glib_none().0,
            ))
        }
    }

    //pub fn set_qdata(&self, quark: /*Ignored*/glib::Quark, data: Option</*Unimplemented*/Fundamental: Pointer>) {
    //    unsafe { TODO: call gobject_sys::g_param_spec_set_qdata() }
    //}

    //pub fn set_qdata_full(&self, quark: /*Ignored*/glib::Quark, data: Option</*Unimplemented*/Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call gobject_sys::g_param_spec_set_qdata_full() }
    //}

    //pub fn steal_qdata(&self, quark: /*Ignored*/glib::Quark) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gobject_sys::g_param_spec_steal_qdata() }
    //}

    pub fn boolean(
        name: &str,
        nick: &str,
        blurb: &str,
        default_value: bool,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_boolean(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                default_value.to_glib(),
                flags.to_glib(),
            ))
        }
    }

    pub fn boxed(
        name: &str,
        nick: &str,
        blurb: &str,
        boxed_type: ::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_boxed(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                boxed_type.to_glib(),
                flags.to_glib(),
            ))
        }
    }

    pub fn char(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: i8,
        maximum: i8,
        default_value: i8,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_char(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn double(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: f64,
        maximum: f64,
        default_value: f64,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_double(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn enum_(
        name: &str,
        nick: &str,
        blurb: &str,
        enum_type: ::Type,
        default_value: i32,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_enum(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                enum_type.to_glib(),
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn flags(
        name: &str,
        nick: &str,
        blurb: &str,
        flags_type: ::Type,
        default_value: u32,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_flags(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                flags_type.to_glib(),
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn float(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: f32,
        maximum: f32,
        default_value: f32,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_float(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn gtype(
        name: &str,
        nick: &str,
        blurb: &str,
        is_a_type: ::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_gtype(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                is_a_type.to_glib(),
                flags.to_glib(),
            ))
        }
    }

    pub fn int(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: i32,
        maximum: i32,
        default_value: i32,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_int(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn int64(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: i64,
        maximum: i64,
        default_value: i64,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_int64(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn long(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: libc::c_long,
        maximum: libc::c_long,
        default_value: libc::c_long,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_long(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn object(
        name: &str,
        nick: &str,
        blurb: &str,
        object_type: ::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_object(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                object_type.to_glib(),
                flags.to_glib(),
            ))
        }
    }

    pub fn override_(name: &str, overridden: &ParamSpec) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_override(
                name.to_glib_none().0,
                overridden.to_glib_none().0,
            ))
        }
    }

    pub fn param(
        name: &str,
        nick: &str,
        blurb: &str,
        param_type: ::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_param(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                param_type.to_glib(),
                flags.to_glib(),
            ))
        }
    }

    pub fn pointer(name: &str, nick: &str, blurb: &str, flags: ParamFlags) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_pointer(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    pub fn string(
        name: &str,
        nick: &str,
        blurb: &str,
        default_value: Option<&str>,
        flags: ParamFlags,
    ) -> ParamSpec {
        let default_value = default_value.to_glib_none();
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_string(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                default_value.0,
                flags.to_glib(),
            ))
        }
    }

    pub fn uchar(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: u8,
        maximum: u8,
        default_value: u8,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_uchar(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn uint(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: u32,
        maximum: u32,
        default_value: u32,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_uint(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn uint64(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: u64,
        maximum: u64,
        default_value: u64,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_uint64(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn ulong(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: libc::c_ulong,
        maximum: libc::c_ulong,
        default_value: libc::c_ulong,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_ulong(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.to_glib(),
            ))
        }
    }

    pub fn unichar(
        name: &str,
        nick: &str,
        blurb: &str,
        default_value: char,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_unichar(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                default_value.to_glib(),
                flags.to_glib(),
            ))
        }
    }

    pub fn value_array(
        name: &str,
        nick: &str,
        blurb: &str,
        element_spec: &ParamSpec,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_value_array(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                element_spec.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    pub fn variant(
        name: &str,
        nick: &str,
        blurb: &str,
        type_: &::VariantTy,
        default_value: Option<&::Variant>,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_variant(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                type_.to_glib_none().0,
                default_value.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }
}

pub trait ParamSpecType:
    StaticType + FromGlibPtrFull<*mut gobject_sys::GParamSpec> + 'static
{
}

extern "C" {
    pub static g_param_spec_types: *const glib_sys::GType;
}

macro_rules! define_param_spec {
    ($rust_type:ident, $ffi_type:path, $mod_name:ident, $rust_type_offset:expr) => {
        mod $mod_name {
            use super::*;

            // Can't use get_type here as this is not a boxed type but another fundamental type
            glib_wrapper! {
                #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub struct $rust_type(Shared<$ffi_type>);

                match fn {
                    ref => |ptr| gobject_sys::g_param_spec_ref_sink(ptr as *mut gobject_sys::GParamSpec) as *mut $ffi_type,
                    unref => |ptr| gobject_sys::g_param_spec_unref(ptr as *mut gobject_sys::GParamSpec),
                }
            }

            impl StaticType for $rust_type {
                fn static_type() -> Type {
                    unsafe {
                        from_glib(*g_param_spec_types.add($rust_type_offset))
                    }
                }
            }

            #[doc(hidden)]
            impl<'a> value::FromValueOptional<'a> for $rust_type {
                #[allow(clippy::missing_safety_doc)]
                unsafe fn from_value_optional(value: &Value) -> Option<Self> {
                    from_glib_full(gobject_sys::g_value_dup_param(value.to_glib_none().0) as *mut $ffi_type)
                }
            }

            #[doc(hidden)]
            impl value::SetValue for $rust_type {
                #[allow(clippy::missing_safety_doc)]
                unsafe fn set_value(value: &mut Value, this: &Self) {
                    gobject_sys::g_value_set_param(value.to_glib_none_mut().0, this.to_glib_none().0 as *mut gobject_sys::GParamSpec)
                }
            }

            #[doc(hidden)]
            impl value::SetValueOptional for $rust_type {
                #[allow(clippy::missing_safety_doc)]
                unsafe fn set_value_optional(value: &mut Value, this: Option<&Self>) {
                    gobject_sys::g_value_set_param(value.to_glib_none_mut().0, this.to_glib_none().0 as *mut gobject_sys::GParamSpec)
                }
            }

            unsafe impl Send for $rust_type {}
            unsafe impl Sync for $rust_type {}

            impl std::ops::Deref for $rust_type {
                type Target = ParamSpec;

                fn deref(&self) -> &Self::Target {
                    unsafe {
                        &*(self as *const $rust_type as *const ParamSpec)
                    }
                }
            }

            impl ParamSpecType for $rust_type {}

            #[doc(hidden)]
            impl FromGlibPtrFull<*mut gobject_sys::GParamSpec> for $rust_type {
                unsafe fn from_glib_full(ptr: *mut gobject_sys::GParamSpec) -> Self {
                    from_glib_full(ptr as *mut $ffi_type)
                }
            }

            impl $rust_type {
                pub fn upcast(self) -> ParamSpec {
                    unsafe {
                        from_glib_full(self.to_glib_full() as *mut gobject_sys::GParamSpec)
                    }
                }

                pub fn upcast_ref(&self) -> &ParamSpec {
                    &*self
                }
            }
        }

        pub use self::$mod_name::$rust_type;

    };
}

macro_rules! define_param_spec_default {
    ($rust_type:ident, $value_type:ty, $from_glib:expr) => {
        impl $rust_type {
            pub fn get_default_value(&self) -> $value_type {
                unsafe {
                    let ptr = self.to_glib_none().0;
                    $from_glib((*ptr).default_value)
                }
            }
        }
    };
}

macro_rules! define_param_spec_min_max {
    ($rust_type:ident, $value_type:ty, $from_glib:expr) => {
        impl $rust_type {
            pub fn get_minimum(&self) -> $value_type {
                unsafe {
                    let ptr = self.to_glib_none().0;
                    $from_glib((*ptr).minimum)
                }
            }

            pub fn get_maximum(&self) -> $value_type {
                unsafe {
                    let ptr = self.to_glib_none().0;
                    $from_glib((*ptr).maximum)
                }
            }
        }
    };
}

macro_rules! define_param_spec_numeric {
    ($rust_type:ident, $ffi_type:path, $value_type:ty, $mod_name:ident, $rust_type_offset:expr, $from_glib:expr) => {
        define_param_spec!($rust_type, $ffi_type, $mod_name, $rust_type_offset);
        define_param_spec_default!($rust_type, $value_type, $from_glib);
        define_param_spec_min_max!($rust_type, $value_type, $from_glib);
    };
}

define_param_spec_numeric!(
    ParamSpecChar,
    gobject_sys::GParamSpecChar,
    i8,
    param_spec_char,
    0,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecUChar,
    gobject_sys::GParamSpecUChar,
    u8,
    param_spec_uchar,
    1,
    |x| x
);

define_param_spec!(
    ParamSpecBoolean,
    gobject_sys::GParamSpecBoolean,
    param_spec_bool,
    2
);

define_param_spec_default!(ParamSpecBoolean, bool, |x| from_glib(x));

define_param_spec_numeric!(
    ParamSpecInt,
    gobject_sys::GParamSpecInt,
    i32,
    param_spec_int,
    3,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecUInt,
    gobject_sys::GParamSpecUInt,
    u32,
    param_spec_uint,
    4,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecLong,
    gobject_sys::GParamSpecLong,
    libc::c_long,
    param_spec_long,
    5,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecULong,
    gobject_sys::GParamSpecULong,
    libc::c_ulong,
    param_spec_ulong,
    6,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecInt64,
    gobject_sys::GParamSpecInt64,
    i64,
    param_spec_int64,
    7,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecUInt64,
    gobject_sys::GParamSpecUInt64,
    u64,
    param_spec_uint64,
    8,
    |x| x
);

define_param_spec!(
    ParamSpecUnichar,
    gobject_sys::GParamSpecUnichar,
    param_spec_unichar,
    9
);

define_param_spec_default!(ParamSpecUnichar, char, |x| from_glib(x));

define_param_spec!(
    ParamSpecEnum,
    gobject_sys::GParamSpecEnum,
    param_spec_enum,
    10
);

define_param_spec_default!(ParamSpecEnum, i32, |x| x);

impl ParamSpecEnum {
    pub fn get_enum_class(&self) -> ::EnumClass {
        unsafe {
            let ptr = self.to_glib_none().0;

            assert!(!(*ptr).enum_class.is_null());

            ::EnumClass::new(from_glib((*(*ptr).enum_class).g_type_class.g_type))
                .expect("Invalid enum class")
        }
    }
}

define_param_spec!(
    ParamSpecFlags,
    gobject_sys::GParamSpecFlags,
    param_spec_flags,
    11
);

define_param_spec_default!(ParamSpecFlags, u32, |x| x);

impl ParamSpecFlags {
    pub fn get_flags_class(&self) -> ::FlagsClass {
        unsafe {
            let ptr = self.to_glib_none().0;

            assert!(!(*ptr).flags_class.is_null());

            ::FlagsClass::new(from_glib((*(*ptr).flags_class).g_type_class.g_type))
                .expect("Invalid flags class")
        }
    }
}

define_param_spec_numeric!(
    ParamSpecFloat,
    gobject_sys::GParamSpecFloat,
    f32,
    param_spec_float,
    12,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecDouble,
    gobject_sys::GParamSpecDouble,
    f64,
    param_spec_double,
    13,
    |x| x
);

define_param_spec!(
    ParamSpecString,
    gobject_sys::GParamSpecString,
    param_spec_string,
    14
);

define_param_spec_default!(ParamSpecString, Option<&str>, |x: *mut libc::c_char| {
    use std::ffi::CStr;

    if x.is_null() {
        None
    } else {
        Some(CStr::from_ptr(x).to_str().unwrap())
    }
});

define_param_spec!(
    ParamSpecParam,
    gobject_sys::GParamSpecParam,
    param_spec_param,
    15
);

define_param_spec!(
    ParamSpecBoxed,
    gobject_sys::GParamSpecBoxed,
    param_spec_boxed,
    16
);

define_param_spec!(
    ParamSpecPointer,
    gobject_sys::GParamSpecPointer,
    param_spec_pointer,
    17
);

define_param_spec!(
    ParamSpecValueArray,
    gobject_sys::GParamSpecValueArray,
    param_spec_value_array,
    18
);

impl ParamSpecValueArray {
    pub fn get_element_spec(&self) -> Option<ParamSpec> {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib_none((*ptr).element_spec)
        }
    }

    pub fn get_fixed_n_elements(&self) -> u32 {
        unsafe {
            let ptr = self.to_glib_none().0;

            (*ptr).fixed_n_elements
        }
    }
}

define_param_spec!(
    ParamSpecObject,
    gobject_sys::GParamSpecObject,
    param_spec_object,
    19
);

define_param_spec!(
    ParamSpecOverride,
    gobject_sys::GParamSpecOverride,
    param_spec_override,
    20
);

impl ParamSpecOverride {
    pub fn get_overridden(&self) -> ParamSpec {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib_none((*ptr).overridden)
        }
    }
}

define_param_spec!(
    ParamSpecGType,
    gobject_sys::GParamSpecGType,
    param_spec_gtype,
    21
);

define_param_spec!(
    ParamSpecVariant,
    gobject_sys::GParamSpecVariant,
    param_spec_variant,
    22
);

define_param_spec_default!(
    ParamSpecVariant,
    Option<::Variant>,
    |x: *mut glib_sys::GVariant| from_glib_none(x)
);

impl ParamSpecVariant {
    pub fn get_type(&self) -> Option<&::VariantTy> {
        unsafe {
            let ptr = self.to_glib_none().0;

            if (*ptr).type_.is_null() {
                None
            } else {
                Some(::VariantTy::from_ptr((*ptr).type_))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_spec_string() {
        let pspec = ParamSpec::string(
            "name",
            "nick",
            "blurb",
            Some("default"),
            ParamFlags::READWRITE,
        );

        assert_eq!(pspec.get_name(), "name");
        assert_eq!(pspec.get_nick(), "nick");
        assert_eq!(pspec.get_blurb(), "blurb");
        let default_value = pspec.get_default_value();
        assert_eq!(default_value.get::<&str>().unwrap(), Some("default"));
        assert_eq!(pspec.get_flags(), ParamFlags::READWRITE);
        assert_eq!(pspec.get_value_type(), Type::String);
        assert_eq!(pspec.get_type(), ParamSpecString::static_type());

        let pspec_ref = pspec
            .downcast_ref::<ParamSpecString>()
            .expect("Not a string param spec");
        assert_eq!(pspec_ref.get_default_value(), Some("default"));

        let pspec = pspec
            .downcast::<ParamSpecString>()
            .expect("Not a string param spec");
        assert_eq!(pspec.get_default_value(), Some("default"));
    }
}
