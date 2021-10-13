// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Rectangle;
use cairo::RectangleInt;
use std::convert::{AsRef, From};
use std::fmt;

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rectangle {
        skip_assert_initialized!();
        Rectangle(ffi::GdkRectangle {
            x,
            y,
            width,
            height,
        })
    }

    pub fn x(&self) -> i32 {
        self.0.x
    }

    pub fn y(&self) -> i32 {
        self.0.y
    }

    pub fn width(&self) -> i32 {
        self.0.width
    }

    pub fn height(&self) -> i32 {
        self.0.height
    }

    #[cfg(not(any(feature = "v3_20", feature = "dox")))]
    #[doc(alias = "gdk_rectangle_equal")]
    fn equal(&self, rect2: &Rectangle) -> bool {
        self.0.x == rect2.0.x
            && self.0.y == rect2.0.y
            && self.0.width == rect2.0.width
            && self.0.height == rect2.0.height
    }
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Rectangle")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

#[cfg(not(any(feature = "v3_20", feature = "dox")))]
impl PartialEq for Rectangle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl AsRef<RectangleInt> for Rectangle {
    fn as_ref(&self) -> &RectangleInt {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl From<RectangleInt> for Rectangle {
    fn from(r: RectangleInt) -> Rectangle {
        skip_assert_initialized!();
        unsafe { *(&r as *const _ as *const _) }
    }
}
