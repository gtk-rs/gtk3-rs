// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use atk_sys;
use glib::translate::*;
use std::fmt;

#[derive(Debug)]
pub struct TextRectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl TextRectangle {
    pub fn uninitialized() -> Self {
        TextRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn to_glib_none_mut(&mut self) -> (*mut atk_sys::AtkTextRectangle, i32) {
        (self as *mut TextRectangle as usize as *mut _, 0)
    }
}

impl fmt::Display for TextRectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TextRectangle")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

#[doc(hidden)]
impl FromGlib<atk_sys::AtkTextRectangle> for TextRectangle {
    fn from_glib(value: atk_sys::AtkTextRectangle) -> Self {
        skip_assert_initialized!();
        TextRectangle {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

#[doc(hidden)]
impl ToGlib for TextRectangle {
    type GlibType = atk_sys::AtkTextRectangle;

    fn to_glib(&self) -> atk_sys::AtkTextRectangle {
        atk_sys::AtkTextRectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}
