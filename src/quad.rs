use Quad;
use Point;
use Rect;
use ffi;
use glib::translate::*;

impl Quad {
    pub fn new(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> Quad {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quad_alloc();
            ffi::graphene_quad_init(alloc, p1.to_glib_none().0, p2.to_glib_none().0, p3.to_glib_none().0, p4.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_rect(r: &Rect) -> Quad {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quad_alloc();
            ffi::graphene_quad_init_from_rect(alloc, r.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
