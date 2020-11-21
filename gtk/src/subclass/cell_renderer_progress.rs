use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use crate::CellRenderer;
use crate::CellRendererProgress;

pub trait CellRendererProgressImpl: CellRendererImpl {}

unsafe impl<T: CellRendererProgressImpl> IsSubclassable<T> for CellRendererProgress {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <CellRenderer as IsSubclassable<T>>::override_vfuncs(class);
    }
}
