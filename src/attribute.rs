// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Attribute;
use AttrClass;
use glib::translate::from_glib_full;
use glib::translate::ToGlibPtr;

impl Attribute {
    pub fn get_attr_class(&self) -> AttrClass {
        unsafe {
            from_glib_full((*self.to_glib_none().0).klass)
        }
    }
}
