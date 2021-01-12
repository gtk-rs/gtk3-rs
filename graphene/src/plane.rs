// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Plane;
use crate::Point3D;
use crate::Vec3;
use crate::Vec4;
use glib::translate::*;

impl Plane {
    #[doc(alias = "graphene_plane_init")]
    pub fn new(normal: Option<&Vec3>, constant: f32) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_plane_alloc();
            ffi::graphene_plane_init(alloc, normal.to_glib_none().0, constant);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_plane_init_from_plane")]
    pub fn new_from_plane(src: &Plane) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_plane_alloc();
            ffi::graphene_plane_init_from_plane(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_plane_init_from_point")]
    pub fn new_from_point(normal: &Vec3, point: &Point3D) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_plane_alloc();
            ffi::graphene_plane_init_from_point(
                alloc,
                normal.to_glib_none().0,
                point.to_glib_none().0,
            );
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_plane_init_from_points")]
    pub fn new_from_points(a: &Point3D, b: &Point3D, c: &Point3D) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_plane_alloc();
            ffi::graphene_plane_init_from_points(
                alloc,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_plane_init_from_vec4")]
    pub fn new_from_vec4(src: &Vec4) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_plane_alloc();
            ffi::graphene_plane_init_from_vec4(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
