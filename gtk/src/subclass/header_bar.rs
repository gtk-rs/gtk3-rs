// Take a look at the license at the top of the repository in the LICENSE file.

use glib::object::IsA;
use glib::subclass::prelude::*;

use super::container::ContainerImpl;

use crate::HeaderBar;

pub trait HeaderBarImpl: ContainerImpl + ObjectSubclass<Type: IsA<HeaderBar>> {}

unsafe impl<T: HeaderBarImpl> IsSubclassable<T> for HeaderBar {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }
    }
}
