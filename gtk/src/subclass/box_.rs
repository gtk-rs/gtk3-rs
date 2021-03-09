// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use crate::Box;
use crate::Container;

pub trait BoxImpl: ContainerImpl {}

unsafe impl<T: BoxImpl> IsSubclassable<T> for Box {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <Container as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Container as IsSubclassable<T>>::instance_init(instance);
    }
}
