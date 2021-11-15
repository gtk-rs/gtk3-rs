// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::Uninitialized;

use crate::RGBA;
use glib::translate::*;
use std::fmt;
use std::str::FromStr;

impl RGBA {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> RGBA {
        skip_assert_initialized!();
        RGBA(ffi::GdkRGBA {
            red,
            green,
            blue,
            alpha,
        })
    }

    pub fn red(&self) -> f64 {
        self.0.red
    }

    pub fn set_red(&mut self, red: f64) {
        self.0.red = red;
    }

    pub fn green(&self) -> f64 {
        self.0.green
    }

    pub fn set_green(&mut self, green: f64) {
        self.0.green = green;
    }

    pub fn blue(&self) -> f64 {
        self.0.blue
    }

    pub fn set_blue(&mut self, blue: f64) {
        self.0.blue = blue;
    }

    pub fn alpha(&self) -> f64 {
        self.0.alpha
    }

    pub fn set_alpha(&mut self, alpha: f64) {
        self.0.alpha = alpha;
    }

    #[doc(alias = "gdk_rgba_parse")]
    pub fn parse(s: &str) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let mut res = RGBA::uninitialized();
            glib::result_from_gboolean!(
                ffi::gdk_rgba_parse(res.to_glib_none_mut().0, s.to_glib_none().0),
                "Can't parse RGBA"
            )
            .map(|_| res)
        }
    }

    pub const BLACK: RGBA = RGBA(ffi::GdkRGBA {
        red: 0f64,
        green: 0f64,
        blue: 0f64,
        alpha: 1f64,
    });

    pub const BLUE: RGBA = RGBA(ffi::GdkRGBA {
        red: 0f64,
        green: 0f64,
        blue: 1f64,
        alpha: 1f64,
    });

    pub const GREEN: RGBA = RGBA(ffi::GdkRGBA {
        red: 0f64,
        green: 1f64,
        blue: 0f64,
        alpha: 1f64,
    });

    pub const RED: RGBA = RGBA(ffi::GdkRGBA {
        red: 1f64,
        green: 0f64,
        blue: 0f64,
        alpha: 1f64,
    });

    pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
        red: 1f64,
        green: 1f64,
        blue: 1f64,
        alpha: 1f64,
    });
}

impl fmt::Debug for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RGBA")
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .field("alpha", &self.alpha())
            .finish()
    }
}

impl FromStr for RGBA {
    type Err = glib::BoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();
        RGBA::parse(s)
    }
}
