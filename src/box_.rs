use Box;
use Point3D;
use Vec3;
use ffi;
use glib::translate::*;

impl Box {
    pub fn new(min: Option<&Point3D>, max: Option<&Point3D>) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init(alloc, min.to_glib_none().0, max.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_box(src: &Box) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init_from_box(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_vec3(min: Option<&Vec3>, max: Option<&Vec3>) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_box_alloc();
            ffi::graphene_box_init_from_vec3(alloc, min.to_glib_none().0, max.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
