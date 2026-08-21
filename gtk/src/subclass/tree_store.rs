// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TreeStore;

use glib::{object::IsA, subclass::prelude::*};

pub trait TreeStoreImpl: ObjectImpl + ObjectSubclass<Type: IsA<TreeStore>> {}

unsafe impl<T: TreeStoreImpl> IsSubclassable<T> for TreeStore {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }
    }
}
