// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;
use glib::GString;
use libc::c_uint;

pub fn keyval_name(keyval: u32) -> Option<GString> {
    skip_assert_initialized!();
    unsafe { from_glib_none(gdk_sys::gdk_keyval_name(keyval as c_uint)) }
}

pub fn keyval_to_unicode(keyval: u32) -> Option<char> {
    skip_assert_initialized!();
    unsafe { ::std::char::from_u32(gdk_sys::gdk_keyval_to_unicode(keyval)).filter(|x| *x != '\0') }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(u32);

impl ::std::ops::Deref for Key {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for Key {
    fn deref_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

impl FromGlib<u32> for Key {
    fn from_glib(value: u32) -> Self {
        Key(value)
    }
}

impl ToGlib for Key {
    type GlibType = u32;

    fn to_glib(&self) -> u32 {
        **self
    }
}

impl Key {
    pub fn to_unicode(&self) -> Option<char> {
        keyval_to_unicode(**self)
    }

    pub fn name(&self) -> Option<GString> {
        keyval_name(**self)
    }
}

impl ::std::fmt::Display for Key {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Key({})", self.0)
    }
}
