// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use crate::Box;
use crate::Container;

pub trait BoxImpl: ContainerImpl {}

unsafe impl<T: BoxImpl> IsSubclassable<T> for Box {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Container as IsSubclassable<T>>::override_vfuncs(class);
    }
}
