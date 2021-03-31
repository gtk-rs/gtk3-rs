// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Color;
use glib::translate::*;

impl Color {
    pub fn red(&self) -> u16 {
        unsafe { *self.to_glib_none().0 }.red
    }

    pub fn green(&self) -> u16 {
        unsafe { *self.to_glib_none().0 }.green
    }

    pub fn blue(&self) -> u16 {
        unsafe { *self.to_glib_none().0 }.blue
    }
}
