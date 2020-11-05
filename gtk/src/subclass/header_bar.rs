use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use Container;
use HeaderBar;

pub trait HeaderBarImpl: ContainerImpl {}

unsafe impl<T: HeaderBarImpl> IsSubclassable<T> for HeaderBar {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Container as IsSubclassable<T>>::override_vfuncs(class);
    }
}
