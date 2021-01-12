// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Euler;
use crate::Matrix;
use crate::Quaternion;
use crate::Vec3;
use crate::Vec4;
use glib::translate::*;

impl Quaternion {
    #[doc(alias = "graphene_quaternion_init")]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quaternion_alloc();
            ffi::graphene_quaternion_init(alloc, x, y, z, w);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_angle_vec3")]
    pub fn new_from_angle_vec3(angle: f32, axis: &Vec3) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quaternion_alloc();
            ffi::graphene_quaternion_init_from_angle_vec3(alloc, angle, axis.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_angles")]
    pub fn new_from_angles(deg_x: f32, deg_y: f32, deg_z: f32) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quaternion_alloc();
            ffi::graphene_quaternion_init_from_angles(alloc, deg_x, deg_y, deg_z);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_euler")]
    pub fn new_from_euler(e: &Euler) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quaternion_alloc();
            ffi::graphene_quaternion_init_from_euler(alloc, e.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_matrix")]
    pub fn new_from_matrix(m: &Matrix) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quaternion_alloc();
            ffi::graphene_quaternion_init_from_matrix(alloc, m.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_quaternion")]
    pub fn new_from_quaternion(src: &Quaternion) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quaternion_alloc();
            ffi::graphene_quaternion_init_from_quaternion(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_radians")]
    pub fn new_from_radians(rad_x: f32, rad_y: f32, rad_z: f32) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quaternion_alloc();
            ffi::graphene_quaternion_init_from_radians(alloc, rad_x, rad_y, rad_z);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_vec4")]
    pub fn new_from_vec4(src: &Vec4) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quaternion_alloc();
            ffi::graphene_quaternion_init_from_vec4(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_quaternion_init_identity")]
    pub fn new_identity() -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quaternion_alloc();
            ffi::graphene_quaternion_init_identity(alloc);
            from_glib_full(alloc)
        }
    }
}
