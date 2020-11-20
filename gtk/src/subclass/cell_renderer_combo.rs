// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib;
use glib::subclass::prelude::*;

use super::cell_renderer_text::CellRendererTextImpl;
use crate::CellRendererCombo;
use crate::CellRendererText;

pub trait CellRendererComboImpl: CellRendererTextImpl {}

unsafe impl<T: CellRendererComboImpl> IsSubclassable<T> for CellRendererCombo {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <CellRendererText as IsSubclassable<T>>::override_vfuncs(class);
    }
}
