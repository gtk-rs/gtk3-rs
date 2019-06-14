use Plane;
use Point3D;
use Vec3;
use Vec4;
use graphene_sys;
use glib::translate::*;

impl Plane {
    pub fn new(normal: Option<&Vec3>, constant: f32) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_plane_alloc();
            graphene_sys::graphene_plane_init(alloc, normal.to_glib_none().0, constant);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_plane(src: &Plane) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_plane_alloc();
            graphene_sys::graphene_plane_init_from_plane(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_point(normal: &Vec3, point: &Point3D) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_plane_alloc();
            graphene_sys::graphene_plane_init_from_point(alloc, normal.to_glib_none().0, point.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_points(a: &Point3D, b: &Point3D, c: &Point3D) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_plane_alloc();
            graphene_sys::graphene_plane_init_from_points(alloc, a.to_glib_none().0, b.to_glib_none().0, c.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec4(src: &Vec4) -> Plane {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_plane_alloc();
            graphene_sys::graphene_plane_init_from_vec4(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
