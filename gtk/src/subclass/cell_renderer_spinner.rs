use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use CellRenderer;
use CellRendererSpinner;

pub trait CellRendererSpinnerImpl: CellRendererImpl {}

unsafe impl<T: CellRendererSpinnerImpl> IsSubclassable<T> for CellRendererSpinner {
    fn override_vfuncs(class: &mut ::glib::object::Class<Self>) {
        <CellRenderer as IsSubclassable<T>>::override_vfuncs(class);
    }
}
