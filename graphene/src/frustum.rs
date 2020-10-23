use glib::translate::*;
use graphene_sys;
use Frustum;
use Matrix;
use Plane;

impl Frustum {
    pub fn get_planes(&self) -> [Plane; 6] {
        unsafe {
            let mut out: [graphene_sys::graphene_plane_t; 6] = std::mem::uninitialized();
            graphene_sys::graphene_frustum_get_planes(self.to_glib_none().0, &mut out as *mut _);
            [
                from_glib_none(&out[0] as *const _),
                from_glib_none(&out[1] as *const _),
                from_glib_none(&out[2] as *const _),
                from_glib_none(&out[3] as *const _),
                from_glib_none(&out[4] as *const _),
                from_glib_none(&out[5] as *const _),
            ]
        }
    }

    pub fn new(p0: &Plane, p1: &Plane, p2: &Plane, p3: &Plane, p4: &Plane, p5: &Plane) -> Frustum {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_frustum_alloc();
            graphene_sys::graphene_frustum_init(
                alloc,
                p0.to_glib_none().0,
                p1.to_glib_none().0,
                p2.to_glib_none().0,
                p3.to_glib_none().0,
                p4.to_glib_none().0,
                p5.to_glib_none().0,
            );
            from_glib_full(alloc)
        }
    }

    pub fn new_from_frustum(src: &Frustum) -> Frustum {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_frustum_alloc();
            graphene_sys::graphene_frustum_init_from_frustum(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_matrix(matrix: &Matrix) -> Frustum {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_frustum_alloc();
            graphene_sys::graphene_frustum_init_from_matrix(alloc, matrix.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
