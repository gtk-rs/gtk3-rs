// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point3D;
use crate::Sphere;
use crate::Vec3;
use glib::translate::*;

impl Sphere {
    #[doc(alias = "graphene_sphere_init_from_points")]
    pub fn init_from_points(&mut self, points: &[&Point3D], center: Option<&Point3D>) {
        let vec: Vec<_> = points
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            ffi::graphene_sphere_init_from_points(
                self.to_glib_none_mut().0,
                n,
                vec.as_ptr(),
                center.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "graphene_sphere_init_from_vectors")]
    pub fn init_from_vectors(&mut self, vectors: &[&Vec3], center: Option<&Point3D>) {
        let vec: Vec<_> = vectors
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            ffi::graphene_sphere_init_from_vectors(
                self.to_glib_none_mut().0,
                n,
                vec.as_ptr(),
                center.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "graphene_sphere_init")]
    pub fn new(center: Option<&Point3D>, radius: f32) -> Sphere {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_sphere_alloc();
            ffi::graphene_sphere_init(alloc, center.to_glib_none().0, radius);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_sphere_init_from_points")]
    pub fn new_from_points(points: &[&Point3D], center: Option<&Point3D>) -> Sphere {
        assert_initialized_main_thread!();

        let vec: Vec<_> = points
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            let alloc = ffi::graphene_sphere_alloc();
            ffi::graphene_sphere_init_from_points(alloc, n, vec.as_ptr(), center.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_sphere_init_from_vectors")]
    pub fn new_from_vectors(vectors: &[&Vec3], center: Option<&Point3D>) -> Sphere {
        assert_initialized_main_thread!();

        let vec: Vec<_> = vectors
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            let alloc = ffi::graphene_sphere_alloc();
            ffi::graphene_sphere_init_from_vectors(alloc, n, vec.as_ptr(), center.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
