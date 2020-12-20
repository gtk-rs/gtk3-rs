// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Box;
use crate::Point3D;
use crate::Vec3;
use glib::translate::*;

impl Box {
    #[doc(alias = "graphene_box_get_vertices")]
    pub fn get_vertices(&self) -> [Vec3; 8] {
        unsafe {
            let mut out: [ffi::graphene_vec3_t; 8] = std::mem::uninitialized();
            ffi::graphene_box_get_vertices(self.to_glib_none().0, &mut out as *mut _);
            [
                from_glib_none(&out[0] as *const _),
                from_glib_none(&out[1] as *const _),
                from_glib_none(&out[2] as *const _),
                from_glib_none(&out[3] as *const _),
                from_glib_none(&out[4] as *const _),
                from_glib_none(&out[5] as *const _),
                from_glib_none(&out[6] as *const _),
                from_glib_none(&out[7] as *const _),
            ]
        }
    }

    #[doc(alias = "graphene_box_init_from_points")]
    pub fn init_from_points(&mut self, points: &[&Point3D]) {
        let vec: Vec<_> = points
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            ffi::graphene_box_init_from_points(self.to_glib_none_mut().0, n, vec.as_ptr());
        }
    }

    #[doc(alias = "graphene_box_init_from_vectors")]
    pub fn init_from_vectors(&mut self, vectors: &[&Vec3]) {
        let vec: Vec<_> = vectors
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            ffi::graphene_box_init_from_vectors(self.to_glib_none_mut().0, n, vec.as_ptr());
        }
    }

    #[doc(alias = "graphene_box_init")]
    pub fn new(min: Option<&Point3D>, max: Option<&Point3D>) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init(alloc, min.to_glib_none().0, max.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_box_init_from_box")]
    pub fn new_from_box(src: &Box) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init_from_box(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_box_init_from_points")]
    pub fn new_from_points(&mut self, points: &[&Point3D]) -> Box {
        assert_initialized_main_thread!();

        let vec: Vec<_> = points
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init_from_points(alloc, n, vec.as_ptr());
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_box_init_from_vec3")]
    pub fn new_from_vec3(min: Option<&Vec3>, max: Option<&Vec3>) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init_from_vec3(alloc, min.to_glib_none().0, max.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_box_init_from_vectors")]
    pub fn new_from_vectors(vectors: &[&Vec3]) -> Box {
        assert_initialized_main_thread!();

        let vec: Vec<_> = vectors
            .iter()
            .map(|e| unsafe { *e.to_glib_none().0 })
            .collect();
        let n = vec.len() as u32;

        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init_from_vectors(alloc, n, vec.as_ptr());
            from_glib_full(alloc)
        }
    }
}
