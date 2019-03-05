use Box;
use ffi;
use glib::translate::*;

impl Box {
    pub fn new() -> Box {
        assert_initialized_main_thread!();
        let mut res: Box = unsafe {
            from_glib_full(ffi::graphene_box_alloc())
        };
        res.init(None, None);
        res
    }
}
