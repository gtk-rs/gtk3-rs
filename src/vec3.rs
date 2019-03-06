use Vec3;
use ffi;
use glib::translate::*;

impl Vec3 {
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
}
