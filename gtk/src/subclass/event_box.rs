use glib::subclass::prelude::*;

use super::bin::BinImpl;
use Bin;
use EventBox;

pub trait EventBoxImpl: BinImpl {}

unsafe impl<T: EventBoxImpl> IsSubclassable<T> for EventBox {
    fn override_vfuncs(class: &mut ::glib::object::Class<Self>) {
        <Bin as IsSubclassable<T>>::override_vfuncs(class);
    }
}
