// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::utils::is_canonical_pspec_name;
use crate::ParamFlags;
use crate::StaticType;
use crate::Type;
use crate::Value;

use std::char::CharTryFromError;
use std::convert::TryFrom;
use std::ffi::CStr;

// Can't use get_type here as this is not a boxed type but another fundamental type
wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ParamSpec(Shared<gobject_ffi::GParamSpec>);

    match fn {
        ref => |ptr| gobject_ffi::g_param_spec_ref_sink(ptr),
        unref => |ptr| gobject_ffi::g_param_spec_unref(ptr),
    }
}

impl StaticType for ParamSpec {
    fn static_type() -> Type {
        unsafe { from_glib(gobject_ffi::G_TYPE_PARAM) }
    }
}

#[doc(hidden)]
impl crate::value::ValueType for ParamSpec {
    type Type = ParamSpec;
}

#[doc(hidden)]
unsafe impl<'a> crate::value::FromValue<'a> for ParamSpec {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        let ptr = gobject_ffi::g_value_dup_param(value.to_glib_none().0);
        assert!(!ptr.is_null());
        from_glib_full(ptr as *mut gobject_ffi::GParamSpec)
    }
}

#[doc(hidden)]
impl crate::value::ToValue for ParamSpec {
    fn to_value(&self) -> crate::Value {
        unsafe {
            let mut value = crate::Value::from_type(ParamSpec::static_type());
            gobject_ffi::g_value_take_param(
                value.to_glib_none_mut().0,
                self.to_glib_full() as *mut _,
            );
            value
        }
    }

    fn value_type(&self) -> crate::Type {
        ParamSpec::static_type()
    }
}

#[doc(hidden)]
impl crate::value::ToValueOptional for ParamSpec {
    fn to_value_optional(s: Option<&Self>) -> crate::Value {
        let mut value = crate::Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, s.to_glib_full() as *mut _);
        }

        value
    }
}

unsafe impl Send for ParamSpec {}
unsafe impl Sync for ParamSpec {}

impl ParamSpec {
    pub fn downcast<T: ParamSpecType>(self) -> Result<T, ParamSpec> {
        unsafe {
            if self.type_() == T::static_type() {
                Ok(from_glib_full(self.to_glib_full()))
            } else {
                Err(self)
            }
        }
    }

    pub fn downcast_ref<T: ParamSpecType>(&self) -> Option<&T> {
        unsafe {
            if self.type_() == T::static_type() {
                Some(&*(self as *const ParamSpec as *const T))
            } else {
                None
            }
        }
    }

    #[doc(alias = "get_type")]
    pub fn type_(&self) -> Type {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib((*(*ptr).g_type_instance.g_class).g_type)
        }
    }

    #[doc(alias = "get_value_type")]
    pub fn value_type(&self) -> crate::Type {
        unsafe { from_glib((*self.to_glib_none().0).value_type) }
    }

    #[doc(alias = "get_owner_type")]
    pub fn owner_type(&self) -> crate::Type {
        unsafe { from_glib((*self.to_glib_none().0).owner_type) }
    }

    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> ParamFlags {
        unsafe { from_glib((*self.to_glib_none().0).flags) }
    }

    #[doc(alias = "g_param_spec_get_blurb")]
    #[doc(alias = "get_blurb")]
    pub fn blurb(&self) -> &str {
        unsafe {
            CStr::from_ptr(gobject_ffi::g_param_spec_get_blurb(self.to_glib_none().0))
                .to_str()
                .unwrap()
        }
    }

    #[doc(alias = "g_param_spec_get_default_value")]
    #[doc(alias = "get_default_value")]
    pub fn default_value(&self) -> &Value {
        unsafe {
            &*(gobject_ffi::g_param_spec_get_default_value(self.to_glib_none().0)
                as *const crate::Value)
        }
    }

    #[doc(alias = "g_param_spec_get_name")]
    #[doc(alias = "get_name")]
    pub fn name<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(gobject_ffi::g_param_spec_get_name(self.to_glib_none().0))
                .to_str()
                .unwrap()
        }
    }

    #[doc(alias = "g_param_spec_get_name_quark")]
    #[doc(alias = "get_name_quark")]
    pub fn name_quark(&self) -> crate::Quark {
        unsafe {
            from_glib(gobject_ffi::g_param_spec_get_name_quark(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_param_spec_get_nick")]
    #[doc(alias = "get_nick")]
    pub fn nick(&self) -> &str {
        unsafe {
            CStr::from_ptr(gobject_ffi::g_param_spec_get_nick(self.to_glib_none().0))
                .to_str()
                .unwrap()
        }
    }

    //pub fn get_qdata(&self, quark: /*Ignored*/glib::Quark) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gobject_ffi::g_param_spec_get_qdata() }
    //}

    #[doc(alias = "g_param_spec_get_redirect_target")]
    #[doc(alias = "get_redirect_target")]
    pub fn redirect_target(&self) -> Option<ParamSpec> {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_get_redirect_target(
                self.to_glib_none().0,
            ))
        }
    }

    //pub fn set_qdata(&self, quark: /*Ignored*/glib::Quark, data: Option</*Unimplemented*/Fundamental: Pointer>) {
    //    unsafe { TODO: call gobject_ffi::g_param_spec_set_qdata() }
    //}

    //pub fn set_qdata_full(&self, quark: /*Ignored*/glib::Quark, data: Option</*Unimplemented*/Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call gobject_ffi::g_param_spec_set_qdata_full() }
    //}

    //pub fn steal_qdata(&self, quark: /*Ignored*/glib::Quark) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gobject_ffi::g_param_spec_steal_qdata() }
    //}

    #[doc(alias = "g_param_spec_boolean")]
    pub fn new_boolean(
        name: &str,
        nick: &str,
        blurb: &str,
        default_value: bool,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_boolean(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                default_value.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_boxed")]
    pub fn new_boxed(
        name: &str,
        nick: &str,
        blurb: &str,
        boxed_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_boxed(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                boxed_type.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_char")]
    pub fn new_char(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: i8,
        maximum: i8,
        default_value: i8,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_char(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_double")]
    pub fn new_double(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: f64,
        maximum: f64,
        default_value: f64,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_double(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_enum")]
    pub fn new_enum(
        name: &str,
        nick: &str,
        blurb: &str,
        enum_type: crate::Type,
        default_value: i32,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_enum(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                enum_type.into_glib(),
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_flags")]
    pub fn new_flags(
        name: &str,
        nick: &str,
        blurb: &str,
        flags_type: crate::Type,
        default_value: u32,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_flags(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                flags_type.into_glib(),
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_float")]
    pub fn new_float(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: f32,
        maximum: f32,
        default_value: f32,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_float(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_gtype")]
    pub fn new_type(
        name: &str,
        nick: &str,
        blurb: &str,
        is_a_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_gtype(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                is_a_type.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_int")]
    pub fn new_int(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: i32,
        maximum: i32,
        default_value: i32,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_int(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_int64")]
    pub fn new_int64(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: i64,
        maximum: i64,
        default_value: i64,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_int64(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_long")]
    pub fn new_long(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: libc::c_long,
        maximum: libc::c_long,
        default_value: libc::c_long,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_long(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_object")]
    pub fn new_object(
        name: &str,
        nick: &str,
        blurb: &str,
        object_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_object(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                object_type.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_override")]
    pub fn new_override(name: &str, overridden: &ParamSpec) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_override(
                name.to_glib_none().0,
                overridden.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_param_spec_param")]
    pub fn new_param(
        name: &str,
        nick: &str,
        blurb: &str,
        param_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_param(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                param_type.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_pointer")]
    pub fn new_pointer(name: &str, nick: &str, blurb: &str, flags: ParamFlags) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_pointer(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_string")]
    pub fn new_string(
        name: &str,
        nick: &str,
        blurb: &str,
        default_value: Option<&str>,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        let default_value = default_value.to_glib_none();
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_string(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                default_value.0,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_uchar")]
    pub fn new_uchar(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: u8,
        maximum: u8,
        default_value: u8,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_uchar(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_uint")]
    pub fn new_uint(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: u32,
        maximum: u32,
        default_value: u32,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_uint(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_uint64")]
    pub fn new_uint64(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: u64,
        maximum: u64,
        default_value: u64,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_uint64(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_ulong")]
    pub fn new_ulong(
        name: &str,
        nick: &str,
        blurb: &str,
        minimum: libc::c_ulong,
        maximum: libc::c_ulong,
        default_value: libc::c_ulong,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_ulong(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                minimum,
                maximum,
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_unichar")]
    pub fn new_unichar(
        name: &str,
        nick: &str,
        blurb: &str,
        default_value: char,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_unichar(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                default_value.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_value_array")]
    pub fn new_value_array(
        name: &str,
        nick: &str,
        blurb: &str,
        element_spec: &ParamSpec,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_value_array(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                element_spec.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_param_spec_variant")]
    pub fn new_variant(
        name: &str,
        nick: &str,
        blurb: &str,
        type_: &crate::VariantTy,
        default_value: Option<&crate::Variant>,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(
            is_canonical_pspec_name(name),
            "{} is not a valid canonical parameter name",
            name
        );
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_variant(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                type_.to_glib_none().0,
                default_value.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }
}

pub unsafe trait ParamSpecType:
    StaticType + FromGlibPtrFull<*mut gobject_ffi::GParamSpec> + 'static
{
}

#[link(name = "gobject-2.0")]
extern "C" {
    pub static g_param_spec_types: *const ffi::GType;
}

macro_rules! define_param_spec {
    ($rust_type:ident, $ffi_type:path, $mod_name:ident, $rust_type_offset:expr) => {
        // Can't use get_type here as this is not a boxed type but another fundamental type
        wrapper! {
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $rust_type(Shared<$ffi_type>);

            match fn {
                ref => |ptr| gobject_ffi::g_param_spec_ref_sink(ptr as *mut gobject_ffi::GParamSpec) as *mut $ffi_type,
                unref => |ptr| gobject_ffi::g_param_spec_unref(ptr as *mut gobject_ffi::GParamSpec),
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
        impl crate::value::ValueType for $rust_type {
            type Type = $rust_type;
        }

        #[doc(hidden)]
        unsafe impl<'a> crate::value::FromValue<'a> for $rust_type {
            type Checker = $crate::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a crate::Value) -> Self {
                let ptr = gobject_ffi::g_value_dup_param(value.to_glib_none().0);
                assert!(!ptr.is_null());
                from_glib_full(ptr as *mut $ffi_type)
            }
        }

        #[doc(hidden)]
        impl crate::value::ToValue for $rust_type {
            fn to_value(&self) -> crate::Value {
                unsafe {
                    let mut value = crate::Value::from_type($rust_type::static_type());
                    gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, self.to_glib_full() as *mut _);
                    value
                }
            }

            fn value_type(&self) -> crate::Type {
                $rust_type::static_type()
            }
        }

        #[doc(hidden)]
        impl crate::value::ToValueOptional for $rust_type {
            fn to_value_optional(s: Option<&Self>) -> crate::Value {
                let mut value = crate::Value::for_value_type::<Self>();
                unsafe {
                    gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, s.to_glib_full() as *mut _);
                }

                value
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

        unsafe impl ParamSpecType for $rust_type {}

        #[doc(hidden)]
        impl FromGlibPtrFull<*mut gobject_ffi::GParamSpec> for $rust_type {
            unsafe fn from_glib_full(ptr: *mut gobject_ffi::GParamSpec) -> Self {
                from_glib_full(ptr as *mut $ffi_type)
            }
        }

        impl $rust_type {
            pub fn upcast(self) -> ParamSpec {
                unsafe {
                    from_glib_full(self.to_glib_full() as *mut gobject_ffi::GParamSpec)
                }
            }

            pub fn upcast_ref(&self) -> &ParamSpec {
                &*self
            }
        }
    };
}

macro_rules! define_param_spec_default {
    ($rust_type:ident, $value_type:ty, $from_glib:expr) => {
        impl $rust_type {
            pub fn default_value(&self) -> $value_type {
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
            pub fn minimum(&self) -> $value_type {
                unsafe {
                    let ptr = self.to_glib_none().0;
                    $from_glib((*ptr).minimum)
                }
            }

            pub fn maximum(&self) -> $value_type {
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
    gobject_ffi::GParamSpecChar,
    i8,
    param_spec_char,
    0,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecUChar,
    gobject_ffi::GParamSpecUChar,
    u8,
    param_spec_uchar,
    1,
    |x| x
);

define_param_spec!(
    ParamSpecBoolean,
    gobject_ffi::GParamSpecBoolean,
    param_spec_bool,
    2
);

define_param_spec_default!(ParamSpecBoolean, bool, |x| from_glib(x));

define_param_spec_numeric!(
    ParamSpecInt,
    gobject_ffi::GParamSpecInt,
    i32,
    param_spec_int,
    3,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecUInt,
    gobject_ffi::GParamSpecUInt,
    u32,
    param_spec_uint,
    4,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecLong,
    gobject_ffi::GParamSpecLong,
    libc::c_long,
    param_spec_long,
    5,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecULong,
    gobject_ffi::GParamSpecULong,
    libc::c_ulong,
    param_spec_ulong,
    6,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecInt64,
    gobject_ffi::GParamSpecInt64,
    i64,
    param_spec_int64,
    7,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecUInt64,
    gobject_ffi::GParamSpecUInt64,
    u64,
    param_spec_uint64,
    8,
    |x| x
);

define_param_spec!(
    ParamSpecUnichar,
    gobject_ffi::GParamSpecUnichar,
    param_spec_unichar,
    9
);

define_param_spec_default!(ParamSpecUnichar, Result<char, CharTryFromError>, TryFrom::try_from);

define_param_spec!(
    ParamSpecEnum,
    gobject_ffi::GParamSpecEnum,
    param_spec_enum,
    10
);

define_param_spec_default!(ParamSpecEnum, i32, |x| x);

impl ParamSpecEnum {
    #[doc(alias = "get_enum_class")]
    pub fn enum_class(&self) -> crate::EnumClass {
        unsafe {
            let ptr = self.to_glib_none().0;

            assert!(!(*ptr).enum_class.is_null());

            crate::EnumClass::new(from_glib((*(*ptr).enum_class).g_type_class.g_type))
                .expect("Invalid enum class")
        }
    }
}

define_param_spec!(
    ParamSpecFlags,
    gobject_ffi::GParamSpecFlags,
    param_spec_flags,
    11
);

define_param_spec_default!(ParamSpecFlags, u32, |x| x);

impl ParamSpecFlags {
    #[doc(alias = "get_flags_class")]
    pub fn flags_class(&self) -> crate::FlagsClass {
        unsafe {
            let ptr = self.to_glib_none().0;

            assert!(!(*ptr).flags_class.is_null());

            crate::FlagsClass::new(from_glib((*(*ptr).flags_class).g_type_class.g_type))
                .expect("Invalid flags class")
        }
    }
}

define_param_spec_numeric!(
    ParamSpecFloat,
    gobject_ffi::GParamSpecFloat,
    f32,
    param_spec_float,
    12,
    |x| x
);

define_param_spec_numeric!(
    ParamSpecDouble,
    gobject_ffi::GParamSpecDouble,
    f64,
    param_spec_double,
    13,
    |x| x
);

define_param_spec!(
    ParamSpecString,
    gobject_ffi::GParamSpecString,
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
    gobject_ffi::GParamSpecParam,
    param_spec_param,
    15
);

define_param_spec!(
    ParamSpecBoxed,
    gobject_ffi::GParamSpecBoxed,
    param_spec_boxed,
    16
);

define_param_spec!(
    ParamSpecPointer,
    gobject_ffi::GParamSpecPointer,
    param_spec_pointer,
    17
);

define_param_spec!(
    ParamSpecValueArray,
    gobject_ffi::GParamSpecValueArray,
    param_spec_value_array,
    18
);

impl ParamSpecValueArray {
    #[doc(alias = "get_element_spec")]
    pub fn element_spec(&self) -> Option<ParamSpec> {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib_none((*ptr).element_spec)
        }
    }

    #[doc(alias = "get_fixed_n_elements")]
    pub fn fixed_n_elements(&self) -> u32 {
        unsafe {
            let ptr = self.to_glib_none().0;

            (*ptr).fixed_n_elements
        }
    }
}

define_param_spec!(
    ParamSpecObject,
    gobject_ffi::GParamSpecObject,
    param_spec_object,
    19
);

define_param_spec!(
    ParamSpecOverride,
    gobject_ffi::GParamSpecOverride,
    param_spec_override,
    20
);

impl ParamSpecOverride {
    #[doc(alias = "get_overridden")]
    pub fn overridden(&self) -> ParamSpec {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib_none((*ptr).overridden)
        }
    }
}

define_param_spec!(
    ParamSpecGType,
    gobject_ffi::GParamSpecGType,
    param_spec_gtype,
    21
);

define_param_spec!(
    ParamSpecVariant,
    gobject_ffi::GParamSpecVariant,
    param_spec_variant,
    22
);

define_param_spec_default!(
    ParamSpecVariant,
    Option<crate::Variant>,
    |x: *mut ffi::GVariant| from_glib_none(x)
);

impl ParamSpecVariant {
    #[doc(alias = "get_type")]
    pub fn type_(&self) -> Option<&crate::VariantTy> {
        unsafe {
            let ptr = self.to_glib_none().0;

            if (*ptr).type_.is_null() {
                None
            } else {
                Some(crate::VariantTy::from_ptr((*ptr).type_))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_spec_string() {
        let pspec = ParamSpec::new_string(
            "name",
            "nick",
            "blurb",
            Some("default"),
            ParamFlags::READWRITE,
        );

        assert_eq!(pspec.name(), "name");
        assert_eq!(pspec.nick(), "nick");
        assert_eq!(pspec.blurb(), "blurb");
        let default_value = pspec.default_value();
        assert_eq!(default_value.get::<&str>().unwrap(), "default");
        assert_eq!(pspec.flags(), ParamFlags::READWRITE);
        assert_eq!(pspec.value_type(), Type::STRING);
        assert_eq!(pspec.type_(), ParamSpecString::static_type());

        let pspec_ref = pspec
            .downcast_ref::<ParamSpecString>()
            .expect("Not a string param spec");
        assert_eq!(pspec_ref.default_value(), Some("default"));

        let pspec = pspec
            .downcast::<ParamSpecString>()
            .expect("Not a string param spec");
        assert_eq!(pspec.default_value(), Some("default"));
    }
}
