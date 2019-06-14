use Rect;
use Vec2;
use graphene_sys;
use glib::translate::*;

impl Rect {
    pub fn get_vertices(&self) -> [Vec2; 4] {
        unsafe {
            let mut out: [graphene_sys::graphene_vec2_t; 4] = std::mem::uninitialized();
            graphene_sys::graphene_rect_get_vertices(self.to_glib_none().0, &mut out as *mut _);
            [
                from_glib_none(&out[0] as *const _),
                from_glib_none(&out[1] as *const _),
                from_glib_none(&out[2] as *const _),
                from_glib_none(&out[3] as *const _),
            ]
        }
    }

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_rect_alloc();
            graphene_sys::graphene_rect_init(alloc, x, y, width, height);
            from_glib_full(alloc)
        }
    }

    pub fn new_from_rect(src: &Rect) -> Rect {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = graphene_sys::graphene_rect_alloc();
            graphene_sys::graphene_rect_init_from_rect(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}
