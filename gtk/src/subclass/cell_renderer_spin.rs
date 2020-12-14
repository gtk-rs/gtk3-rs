// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::cell_renderer_text::CellRendererTextImpl;
use crate::CellRendererSpin;
use crate::CellRendererText;

pub trait CellRendererSpinImpl: CellRendererTextImpl {}

unsafe impl<T: CellRendererSpinImpl> IsSubclassable<T> for CellRendererSpin {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <CellRendererText as IsSubclassable<T>>::override_vfuncs(class);
    }
}
