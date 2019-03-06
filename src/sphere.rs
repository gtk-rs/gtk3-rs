use Sphere;
use Point3D;
use ffi;
use glib::translate::*;

impl Sphere {
    pub fn new(center: Option<&Point3D>, radius: f32) -> Sphere {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_sphere_alloc();
            ffi::graphene_sphere_init(alloc, center.to_glib_none().0, radius);
            from_glib_full(alloc)
        }
    }
}
