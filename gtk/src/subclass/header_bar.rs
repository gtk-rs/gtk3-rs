// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use crate::Container;
use crate::HeaderBar;

pub trait HeaderBarImpl: ContainerImpl {}

unsafe impl<T: HeaderBarImpl> IsSubclassable<T> for HeaderBar {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Container as IsSubclassable<T>>::override_vfuncs(class);
    }
}
