use glib::subclass::prelude::*;

use super::cell_renderer_text::CellRendererTextImpl;
use CellRendererCombo;
use CellRendererText;

pub trait CellRendererComboImpl: CellRendererTextImpl {}

unsafe impl<T: CellRendererComboImpl> IsSubclassable<T> for CellRendererCombo {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <CellRendererText as IsSubclassable<T>>::override_vfuncs(class);
    }
}
