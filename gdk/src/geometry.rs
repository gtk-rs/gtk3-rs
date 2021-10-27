// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Gravity;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkGeometry")]
    pub struct Geometry(BoxedInline<ffi::GdkGeometry>);
}

impl Geometry {
    pub fn min_width(&self) -> i32 {
        self.0.min_width
    }
    pub fn min_height(&self) -> i32 {
        self.0.min_height
    }
    pub fn max_width(&self) -> i32 {
        self.0.max_width
    }
    pub fn max_height(&self) -> i32 {
        self.0.max_height
    }
    pub fn base_width(&self) -> i32 {
        self.0.base_width
    }
    pub fn base_height(&self) -> i32 {
        self.0.base_height
    }
    pub fn width_inc(&self) -> i32 {
        self.0.width_inc
    }
    pub fn height_inc(&self) -> i32 {
        self.0.height_inc
    }
    pub fn min_aspect(&self) -> f64 {
        self.0.min_aspect
    }
    pub fn max_aspect(&self) -> f64 {
        self.0.max_aspect
    }
    pub fn win_gravity(&self) -> Gravity {
        unsafe { from_glib(self.0.win_gravity) }
    }
}

impl fmt::Debug for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Geometry")
            .field("min_width", &self.min_width())
            .field("min_height", &self.min_height())
            .field("max_width", &self.max_width())
            .field("max_height", &self.max_height())
            .field("base_width", &self.base_width())
            .field("base_height", &self.base_height())
            .field("width_inc", &self.width_inc())
            .field("height_inc", &self.height_inc())
            .field("min_aspect", &self.min_aspect())
            .field("max_aspect", &self.max_aspect())
            .field("win_gravity", &self.win_gravity())
            .finish()
    }
}
