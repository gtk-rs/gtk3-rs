// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::widget::WidgetImpl;
use crate::DrawingArea;
use crate::Widget;

pub trait DrawingAreaImpl: WidgetImpl {}

unsafe impl<T: DrawingAreaImpl> IsSubclassable<T> for DrawingArea {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Widget as IsSubclassable<T>>::instance_init(instance);
    }
}
