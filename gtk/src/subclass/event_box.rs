// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib;
use glib::subclass::prelude::*;

use super::bin::BinImpl;
use crate::Bin;
use crate::EventBox;

pub trait EventBoxImpl: BinImpl {}

unsafe impl<T: EventBoxImpl> IsSubclassable<T> for EventBox {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Bin as IsSubclassable<T>>::override_vfuncs(class);
    }
}
