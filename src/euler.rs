use Euler;
use EulerOrder;
use Matrix;
use Quaternion;
use Vec3;
use ffi;
use glib::translate::*;

impl Euler {
    pub fn new(x: f32, y: f32, z: f32) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init(alloc, x, y, z);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_euler(src: Option<&Euler>) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init_from_euler(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_matrix(m: Option<&Matrix>, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init_from_matrix(alloc, m.to_glib_none().0, order.to_glib());
            from_glib_full(alloc)
        }
    }

    pub fn new_from_quaternion(q: Option<&Quaternion>, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init_from_quaternion(alloc, q.to_glib_none().0, order.to_glib());
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec3(v: Option<&Vec3>, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init_from_vec3(alloc, v.to_glib_none().0, order.to_glib());
            from_glib_full(alloc)
        }
    }

    pub fn new_with_order(x: f32, y: f32, z: f32, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init_with_order(alloc, x, y, z, order.to_glib());
            from_glib_full(alloc)
        }
    }
}
