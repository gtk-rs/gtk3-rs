// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Size;
use glib::translate::*;

impl Size {
    #[doc(alias = "graphene_size_init")]
    pub fn new(width: f32, height: f32) -> Size {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_size_alloc();
            ffi::graphene_size_init(alloc, width, height);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_size_init_from_size")]
    pub fn new_from_size(src: &Size) -> Size {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_size_alloc();
            ffi::graphene_size_init_from_size(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
