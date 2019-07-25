use glib::translate::*;
use graphene_sys;
use Point3D;
use Triangle;
use Vec3;

impl Triangle {
    pub fn new_from_point3d(
        a: Option<&Point3D>,
        b: Option<&Point3D>,
        c: Option<&Point3D>,
    ) -> Triangle {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_triangle_alloc();
            graphene_sys::graphene_triangle_init_from_point3d(
                alloc,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec3(a: Option<&Vec3>, b: Option<&Vec3>, c: Option<&Vec3>) -> Triangle {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_triangle_alloc();
            graphene_sys::graphene_triangle_init_from_vec3(
                alloc,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
            from_glib_full(alloc)
        }
    }
}
