// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use crate::CellRenderer;
use crate::CellRendererProgress;

pub trait CellRendererProgressImpl: CellRendererImpl {}

unsafe impl<T: CellRendererProgressImpl> IsSubclassable<T> for CellRendererProgress {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <CellRenderer as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <CellRenderer as IsSubclassable<T>>::instance_init(instance);
    }
}
