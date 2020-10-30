use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use Container;
use TreeView;

pub trait TreeViewImpl: ContainerImpl {}

unsafe impl<T: ContainerImpl> IsSubclassable<T> for TreeView {
    fn override_vfuncs(class: &mut ::glib::object::Class<Self>) {
        <Container as IsSubclassable<T>>::override_vfuncs(class);
    }
}
