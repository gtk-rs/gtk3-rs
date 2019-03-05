use Euler;
use ffi;
use glib::translate::*;

impl Euler {
    pub fn new() -> Euler {
        assert_initialized_main_thread!();
        let mut res: Euler = unsafe {
            from_glib_full(ffi::graphene_euler_alloc())
        };
        res.init(0.0, 0.0, 0.0);
        res
    }
}
