use Ray;
use Point3D;
use Vec3;
use graphene_sys;
use glib::translate::*;

impl Ray {
    pub fn new(origin: Option<&Point3D>, direction: Option<&Vec3>) -> Ray {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_ray_alloc();
            graphene_sys::graphene_ray_init(alloc, origin.to_glib_none().0, direction.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_ray(src: &Ray) -> Ray {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_ray_alloc();
            graphene_sys::graphene_ray_init_from_ray(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec3(origin: Option<&Vec3>, direction: Option<&Vec3>) -> Ray {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_ray_alloc();
            graphene_sys::graphene_ray_init_from_vec3(alloc, origin.to_glib_none().0, direction.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
