// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Bin;
use ListBoxRow;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::bin::BinImpl;

pub trait ListBoxRowImpl: ListBoxRowImplExt + BinImpl {
    fn activate(&self, list_box_row: &Self::Type) {
        self.list_box_row_activate(list_box_row)
    }
}

pub trait ListBoxRowImplExt: ObjectSubclass {
    fn list_box_row_activate(&self, list_box_row: &Self::Type);
}

unsafe impl<T: ListBoxRowImpl> IsSubclassable<T> for ListBoxRow {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Bin as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.activate = Some(list_box_row_activate::<T>);
    }
}

unsafe extern "C" fn list_box_row_activate<T: ListBoxRowImpl>(ptr: *mut gtk_sys::GtkListBoxRow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBoxRow> = from_glib_borrow(ptr);

    imp.activate(wrap.unsafe_cast_ref())
}
