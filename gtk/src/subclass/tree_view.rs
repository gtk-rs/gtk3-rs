// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use crate::Container;
use crate::TreeView;

pub trait TreeViewImpl: ContainerImpl {}

unsafe impl<T: ContainerImpl> IsSubclassable<T> for TreeView {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Container as IsSubclassable<T>>::override_vfuncs(class);
    }
}
