// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Vec2;
use crate::Vec3;
use crate::Vec4;
use glib::translate::*;

impl Vec4 {
    #[doc(alias = "graphene_vec4_init_from_float")]
    pub fn init_from_float(&mut self, src: &[f32; 4]) {
        unsafe {
            ffi::graphene_vec4_init_from_float(self.to_glib_none_mut().0, src as *const _);
        }
    }

    #[doc(alias = "graphene_vec4_init")]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec4_alloc();
            ffi::graphene_vec4_init(alloc, x, y, z, w);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_vec4_init_from_vec2")]
    pub fn new_from_vec2(src: &Vec2, z: f32, w: f32) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec4_alloc();
            ffi::graphene_vec4_init_from_vec2(alloc, src.to_glib_none().0, z, w);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_vec4_init_from_vec3")]
    pub fn new_from_vec3(src: &Vec3, w: f32) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec4_alloc();
            ffi::graphene_vec4_init_from_vec3(alloc, src.to_glib_none().0, w);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_vec4_init_from_vec4")]
    pub fn new_from_vec4(src: &Vec4) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec4_alloc();
            ffi::graphene_vec4_init_from_vec4(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_vec4_init_from_float")]
    pub fn new_from_float(src: &[f32; 4]) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec4_alloc();
            ffi::graphene_vec4_init_from_float(alloc, src as *const _);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_vec4_to_float")]
    pub fn to_float(&self) -> [f32; 4] {
        unsafe {
            let mut out = std::mem::uninitialized();
            ffi::graphene_vec4_to_float(self.to_glib_none().0, &mut out as *mut _);
            out
        }
    }
}
