// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Euler;
use crate::EulerOrder;
use crate::Matrix;
use crate::Quaternion;
use crate::Vec3;
use glib::translate::*;

impl Euler {
    #[doc(alias = "graphene_euler_init")]
    pub fn new(x: f32, y: f32, z: f32) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init(alloc, x, y, z);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_euler_init_from_euler")]
    pub fn new_from_euler(src: Option<&Euler>) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init_from_euler(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_euler_init_from_matrix")]
    pub fn new_from_matrix(m: Option<&Matrix>, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init_from_matrix(alloc, m.to_glib_none().0, order.to_glib());
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_euler_init_from_quaternion")]
    pub fn new_from_quaternion(q: Option<&Quaternion>, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init_from_quaternion(alloc, q.to_glib_none().0, order.to_glib());
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_euler_init_from_vec3")]
    pub fn new_from_vec3(v: Option<&Vec3>, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init_from_vec3(alloc, v.to_glib_none().0, order.to_glib());
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_euler_init_with_order")]
    pub fn new_with_order(x: f32, y: f32, z: f32, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_euler_alloc();
            ffi::graphene_euler_init_with_order(alloc, x, y, z, order.to_glib());
            from_glib_full(alloc)
        }
    }
}
