use Rect;
use ffi;
use glib::translate::*;

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_rect_alloc();
            ffi::graphene_rect_init(alloc, x, y, width, height);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_rect(src: &Rect) -> Rect {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_rect_alloc();
            ffi::graphene_rect_init_from_rect(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
