use Frustum;
use Matrix;
use Plane;
use ffi;
use glib::translate::*;

impl Frustum {
    pub fn new(p0: &Plane, p1: &Plane, p2: &Plane, p3: &Plane, p4: &Plane, p5: &Plane) -> Frustum {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_frustum_alloc();
            ffi::graphene_frustum_init(alloc, p0.to_glib_none().0, p1.to_glib_none().0, p2.to_glib_none().0, p3.to_glib_none().0, p4.to_glib_none().0, p5.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_frustum(src: &Frustum) -> Frustum {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_frustum_alloc();
            ffi::graphene_frustum_init_from_frustum(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_matrix(matrix: &Matrix) -> Frustum {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_frustum_alloc();
            ffi::graphene_frustum_init_from_matrix(alloc, matrix.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
