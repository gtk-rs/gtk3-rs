// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Vec3;
use glib::translate::*;

impl Vec3 {
    pub fn init_from_float(&mut self, src: &[f32; 3]) {
        unsafe {
            ffi::graphene_vec3_init_from_float(self.to_glib_none_mut().0, src as *const _);
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec3_alloc();
            ffi::graphene_vec3_init(alloc, x, y, z);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec3(src: &Vec3) -> Vec3 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec3_alloc();
            ffi::graphene_vec3_init_from_vec3(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_float(src: &[f32; 3]) -> Vec3 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec3_alloc();
            ffi::graphene_vec3_init_from_float(alloc, src as *const _);
            from_glib_full(alloc)
        }
    }

    pub fn to_float(&self) -> [f32; 3] {
        unsafe {
            let mut out = std::mem::uninitialized();
            ffi::graphene_vec3_to_float(self.to_glib_none().0, &mut out as *mut _);
            out
        }
    }
}
