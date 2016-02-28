// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;
use ffi::GdkRGBA;

pub trait RGBA {
    fn white() -> GdkRGBA;
    fn blue() -> GdkRGBA;
    fn green() -> GdkRGBA;
    fn red() -> GdkRGBA;
    fn black() -> GdkRGBA;
    fn copy(&self) -> GdkRGBA;
    fn parse(&mut self, spec: &str) -> bool;
    fn to_string(&self) -> Option<String>;
}

impl RGBA for GdkRGBA {
    fn white() -> GdkRGBA {
        skip_assert_initialized!();
        GdkRGBA {
            red: 1f64,
            green: 1f64,
            blue: 1f64,
            alpha: 1f64
        }
    }

    fn blue() -> GdkRGBA {
        skip_assert_initialized!();
        GdkRGBA {
            red: 0f64,
            green: 0f64,
            blue: 1f64,
            alpha: 1f64
        }
    }

    fn green() -> GdkRGBA {
        skip_assert_initialized!();
        GdkRGBA {
            red: 0f64,
            green: 1f64,
            blue: 0f64,
            alpha: 1f64
        }
    }

    fn red() -> GdkRGBA {
        skip_assert_initialized!();
        GdkRGBA {
            red: 1f64,
            green: 0f64,
            blue: 0f64,
            alpha: 1f64
        }
    }

    fn black() -> GdkRGBA {
        skip_assert_initialized!();
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
            from_glib(ffi::gdk_rgba_parse(self, spec.to_glib_none().0))
        }
    }

    fn to_string(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gdk_rgba_to_string(self))
        }
    }
}
