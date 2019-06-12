// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>
//
// TODO: Implement custom subtyping here for things like GParamSpecInt to get
// default/min/max values and similar
use gobject_sys;
use libc;
use translate::*;
use ParamFlags;
use Value;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ParamSpec(Shared<gobject_sys::GParamSpec>);

    match fn {
        ref => |ptr| gobject_sys::g_param_spec_ref_sink(ptr),
        unref => |ptr| gobject_sys::g_param_spec_unref(ptr),
        get_type => || gobject_sys::G_TYPE_PARAM,
    }
}

unsafe impl Send for ParamSpec {}
unsafe impl Sync for ParamSpec {}

impl ParamSpec {
    pub fn get_value_type(&self) -> ::Type {
        unsafe { from_glib((*self.to_glib_none().0).value_type) }
    }

    pub fn get_owner_type(&self) -> ::Type {
        unsafe { from_glib((*self.to_glib_none().0).owner_type) }
    }

    pub fn get_flags(&self) -> ParamFlags {
        unsafe { from_glib((*self.to_glib_none().0).flags) }
    }

    pub fn get_blurb(&self) -> String {
        unsafe { from_glib_none(gobject_sys::g_param_spec_get_blurb(self.to_glib_none().0)) }
    }

    pub fn get_default_value(&self) -> Option<Value> {
        unsafe {
            from_glib_none(gobject_sys::g_param_spec_get_default_value(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_name(&self) -> String {
        unsafe { from_glib_none(gobject_sys::g_param_spec_get_name(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    pub fn get_name_quark(&self) -> ::Quark {
        unsafe {
            from_glib(gobject_sys::g_param_spec_get_name_quark(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_nick(&self) -> String {
        unsafe { from_glib_none(gobject_sys::g_param_spec_get_nick(self.to_glib_none().0)) }
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
            from_glib_full(gobject_sys::g_param_spec_boolean(
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
            from_glib_full(gobject_sys::g_param_spec_boxed(
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
            from_glib_full(gobject_sys::g_param_spec_char(
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
            from_glib_full(gobject_sys::g_param_spec_double(
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
            from_glib_full(gobject_sys::g_param_spec_enum(
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
            from_glib_full(gobject_sys::g_param_spec_flags(
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
            from_glib_full(gobject_sys::g_param_spec_float(
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
            from_glib_full(gobject_sys::g_param_spec_gtype(
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
            from_glib_full(gobject_sys::g_param_spec_int(
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
            from_glib_full(gobject_sys::g_param_spec_int64(
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
            from_glib_full(gobject_sys::g_param_spec_long(
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
            from_glib_full(gobject_sys::g_param_spec_object(
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
            from_glib_full(gobject_sys::g_param_spec_param(
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
            from_glib_full(gobject_sys::g_param_spec_pointer(
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
            from_glib_full(gobject_sys::g_param_spec_string(
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
            from_glib_full(gobject_sys::g_param_spec_uchar(
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
            from_glib_full(gobject_sys::g_param_spec_uint(
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
            from_glib_full(gobject_sys::g_param_spec_uint64(
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
            from_glib_full(gobject_sys::g_param_spec_ulong(
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
            from_glib_full(gobject_sys::g_param_spec_unichar(
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
