// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use crate::CellRenderer;
use crate::CellRendererPixbuf;

pub trait CellRendererPixbufImpl: CellRendererImpl {}

unsafe impl<T: CellRendererPixbufImpl> IsSubclassable<T> for CellRendererPixbuf {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <CellRenderer as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <CellRenderer as IsSubclassable<T>>::instance_init(instance);
    }
}
