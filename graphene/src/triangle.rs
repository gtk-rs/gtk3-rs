// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point3D;
use crate::Triangle;
use crate::Vec3;
use glib::translate::*;

impl Triangle {
    #[doc(alias = "graphene_triangle_init_from_point3d")]
    pub fn new_from_point3d(
        a: Option<&Point3D>,
        b: Option<&Point3D>,
        c: Option<&Point3D>,
    ) -> Triangle {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_triangle_alloc();
            ffi::graphene_triangle_init_from_point3d(
                alloc,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_triangle_init_from_vec3")]
    pub fn new_from_vec3(a: Option<&Vec3>, b: Option<&Vec3>, c: Option<&Vec3>) -> Triangle {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_triangle_alloc();
            ffi::graphene_triangle_init_from_vec3(
                alloc,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
            from_glib_full(alloc)
        }
    }
}
