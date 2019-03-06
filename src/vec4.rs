use Vec4;
use Vec2;
use Vec3;
use ffi;
use glib::translate::*;

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec4_alloc();
            ffi::graphene_vec4_init(alloc, x, y, z, w);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec2(src: &Vec2, z: f32, w: f32) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec4_alloc();
            ffi::graphene_vec4_init_from_vec2(alloc, src.to_glib_none().0, z, w);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec3(src: &Vec3, w: f32) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec4_alloc();
            ffi::graphene_vec4_init_from_vec3(alloc, src.to_glib_none().0, w);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec4(src: &Vec4) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec4_alloc();
            ffi::graphene_vec4_init_from_vec4(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
