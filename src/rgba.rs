// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! RGBA Colors — RGBA colors

use glib::translate::*;
use gdk_ffi as ffi;
use gdk_ffi::GdkRGBA;

pub trait RGBA {
    fn white() -> GdkRGBA;
    fn blue() -> GdkRGBA;
    fn green() -> GdkRGBA;
    fn red() -> GdkRGBA;
    fn black() -> GdkRGBA;
    fn copy(&self) -> GdkRGBA;
    fn parse(&mut self, spec: &str) -> bool;
    fn equal(&self, other: &GdkRGBA) -> bool;
    fn hash(&self) -> u32;
    fn to_string(&self) -> Option<String>;
}

impl RGBA for GdkRGBA {
    fn white() -> GdkRGBA {
        GdkRGBA {
            red: 1f64,
            green: 1f64,
            blue: 1f64,
            alpha: 1f64
        }
    }

    fn blue() -> GdkRGBA {
        GdkRGBA {
            red: 0f64,
            green: 0f64,
            blue: 1f64,
            alpha: 1f64
        }
    }

    fn green() -> GdkRGBA {
        GdkRGBA {
            red: 0f64,
            green: 1f64,
            blue: 0f64,
            alpha: 1f64
        }
    }

    fn red() -> GdkRGBA {
        GdkRGBA {
            red: 1f64,
            green: 0f64,
            blue: 0f64,
            alpha: 1f64
        }
    }

    fn black() -> GdkRGBA {
        GdkRGBA {
            red: 0f64,
            green: 0f64,
            blue: 0f64,
            alpha: 1f64
        }
    }

    fn copy(&self) -> GdkRGBA {
        GdkRGBA {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha
        }
    }

    fn parse(&mut self, spec: &str) -> bool {
        unsafe {
            ::glib::to_bool(ffi::gdk_rgba_parse(self, spec.to_glib_none().0))
        }
    }

    fn equal(&self, other: &GdkRGBA) -> bool {
        unsafe { ::glib::to_bool(ffi::gdk_rgba_equal(self, other)) }
    }

    fn hash(&self) -> u32 {
        unsafe { ffi::gdk_rgba_hash(self) }
    }

    fn to_string(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gdk_rgba_to_string(self))
        }
    }
}
