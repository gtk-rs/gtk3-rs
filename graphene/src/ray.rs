// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point3D;
use crate::Ray;
use crate::Vec3;
use glib::translate::*;

impl Ray {
    #[doc(alias = "graphene_ray_init")]
    pub fn new(origin: Option<&Point3D>, direction: Option<&Vec3>) -> Ray {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_ray_alloc();
            ffi::graphene_ray_init(alloc, origin.to_glib_none().0, direction.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_ray_init_from_ray")]
    pub fn new_from_ray(src: &Ray) -> Ray {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_ray_alloc();
            ffi::graphene_ray_init_from_ray(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_ray_init_from_vec3")]
    pub fn new_from_vec3(origin: Option<&Vec3>, direction: Option<&Vec3>) -> Ray {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_ray_alloc();
            ffi::graphene_ray_init_from_vec3(
                alloc,
                origin.to_glib_none().0,
                direction.to_glib_none().0,
            );
            from_glib_full(alloc)
        }
    }
}
