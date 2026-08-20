// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ListStore;

use glib::{object::IsA, subclass::prelude::*};

pub trait ListStoreImpl: ObjectImpl + ObjectSubclass<Type: IsA<ListStore>> {}

unsafe impl<T: ListStoreImpl> IsSubclassable<T> for ListStore {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }
    }
}
