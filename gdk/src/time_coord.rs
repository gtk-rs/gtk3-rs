// Take a look at the license at the top of the repository in the LICENSE file.

glib::wrapper! {
    #[doc(alias = "GdkTimeCoord")]
    pub struct TimeCoord(BoxedInline<ffi::GdkTimeCoord>);
}

impl TimeCoord {
    pub fn time(&self) -> u32 {
        self.0.time
    }

    pub fn axes(&self) -> &[f64; ffi::GDK_MAX_TIMECOORD_AXES as usize] {
        &self.0.axes
    }
}
