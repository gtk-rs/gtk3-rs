// Take a look at the license at the top of the repository in the LICENSE file.

glib::wrapper! {
    #[doc(alias = "GdkWaylandGLContext")]
    pub struct WaylandGLContext(Object<ffi::GdkWaylandGLContext>) @extends gdk::GLContext;

    match fn {
        type_ => || ffi::gdk_wayland_gl_context_get_type(),
    }
}
