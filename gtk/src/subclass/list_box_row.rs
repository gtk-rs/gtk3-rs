// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Bin;
use crate::ListBoxRow;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::bin::BinImpl;

pub trait ListBoxRowImpl: ListBoxRowImplExt + BinImpl {
    fn activate(&self, list_box_row: &Self::Type) {
        self.parent_activate(list_box_row)
    }
}

pub trait ListBoxRowImplExt: ObjectSubclass {
    fn parent_activate(&self, list_box_row: &Self::Type);
}

impl<T: ListBoxRowImpl> ListBoxRowImplExt for T {
    fn parent_activate(&self, list_box_row: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkListBoxRowClass;
            if let Some(f) = (*parent_class).activate {
                f(list_box_row
                    .unsafe_cast_ref::<ListBoxRow>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: ListBoxRowImpl> IsSubclassable<T> for ListBoxRow {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <Bin as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.activate = Some(list_box_row_activate::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Bin as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn list_box_row_activate<T: ListBoxRowImpl>(ptr: *mut ffi::GtkListBoxRow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBoxRow> = from_glib_borrow(ptr);

    imp.activate(wrap.unsafe_cast_ref())
}
