use Point;
use Vec2;
use graphene_sys;
use glib::translate::*;

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_point_alloc();
            graphene_sys::graphene_point_init(alloc, x, y);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_point(src: &Point) -> Point {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_point_alloc();
            graphene_sys::graphene_point_init_from_point(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec2(src: &Vec2) -> Point {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_point_alloc();
            graphene_sys::graphene_point_init_from_vec2(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
