// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point;
use crate::Vec2;
use glib::translate::*;

impl Point {
    #[doc(alias = "graphene_point_init")]
    pub fn new(x: f32, y: f32) -> Point {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_point_alloc();
            ffi::graphene_point_init(alloc, x, y);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_point_init_from_point")]
    pub fn new_from_point(src: &Point) -> Point {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_point_alloc();
            ffi::graphene_point_init_from_point(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_point_init_from_vec2")]
    pub fn new_from_vec2(src: &Vec2) -> Point {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_point_alloc();
            ffi::graphene_point_init_from_vec2(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
