// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::cell_renderer_text::CellRendererTextImpl;
use crate::CellRendererSpin;
use crate::CellRendererText;

pub trait CellRendererSpinImpl: CellRendererTextImpl {}

unsafe impl<T: CellRendererSpinImpl> IsSubclassable<T> for CellRendererSpin {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <CellRendererText as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <CellRendererText as IsSubclassable<T>>::instance_init(instance);
    }
}
