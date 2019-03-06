use Vec2;
use ffi;
use glib::translate::*;

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec2_alloc();
            ffi::graphene_vec2_init(alloc, x, y);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec2(src: &Vec2) -> Vec2 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec2_alloc();
            ffi::graphene_vec2_init_from_vec2(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
