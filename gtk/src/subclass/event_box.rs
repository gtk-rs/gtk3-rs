// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::bin::BinImpl;
use crate::Bin;
use crate::EventBox;

pub trait EventBoxImpl: BinImpl {}

unsafe impl<T: EventBoxImpl> IsSubclassable<T> for EventBox {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <Bin as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Bin as IsSubclassable<T>>::instance_init(instance);
    }
}
