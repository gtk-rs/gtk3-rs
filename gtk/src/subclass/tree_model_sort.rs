// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TreeModelSort;

use glib::{object::IsA, subclass::prelude::*};

pub trait TreeModelSortImpl: ObjectImpl + ObjectSubclass<Type: IsA<TreeModelSort>> {}

unsafe impl<T: TreeModelSortImpl> IsSubclassable<T> for TreeModelSort {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }
    }
}
