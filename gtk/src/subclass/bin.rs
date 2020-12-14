// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use crate::Bin;
use crate::Container;

pub trait BinImpl: ContainerImpl {}

unsafe impl<T: BinImpl> IsSubclassable<T> for Bin {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Container as IsSubclassable<T>>::override_vfuncs(class);
    }
}
