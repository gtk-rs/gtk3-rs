// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RGBA;
use std::fmt;

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

    pub fn green(&self) -> f64 {
        self.0.green
    }

    pub fn blue(&self) -> f64 {
        self.0.blue
    }

    pub fn alpha(&self) -> f64 {
        self.0.alpha
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
