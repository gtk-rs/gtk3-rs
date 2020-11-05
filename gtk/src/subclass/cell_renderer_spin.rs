use glib::subclass::prelude::*;

use super::cell_renderer_text::CellRendererTextImpl;
use CellRendererSpin;
use CellRendererText;

pub trait CellRendererSpinImpl: CellRendererTextImpl {}

unsafe impl<T: CellRendererSpinImpl> IsSubclassable<T> for CellRendererSpin {
    fn override_vfuncs(class: &mut ::glib::object::Class<Self>) {
        <CellRendererText as IsSubclassable<T>>::override_vfuncs(class);
    }
}
