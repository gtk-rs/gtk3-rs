// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Grid, subclass::container::ContainerImpl};

use glib::{object::IsA, subclass::prelude::*};

pub trait GridImpl: ContainerImpl + ObjectSubclass<Type: IsA<Grid>> {}

unsafe impl<T: GridImpl> IsSubclassable<T> for Grid {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }
    }
}
