use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use Container;
use Stack;

pub trait StackImpl: ContainerImpl {}

unsafe impl<T: ContainerImpl> IsSubclassable<T> for Stack {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Container as IsSubclassable<T>>::override_vfuncs(class);
    }
}
