// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point3D;
use crate::Vec3;
use glib::translate::*;

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Point3D {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_point3d_alloc();
            ffi::graphene_point3d_init(alloc, x, y, z);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_point(src: &Point3D) -> Point3D {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_point3d_alloc();
            ffi::graphene_point3d_init_from_point(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec3(v: &Vec3) -> Point3D {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_point3d_alloc();
            ffi::graphene_point3d_init_from_vec3(alloc, v.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
