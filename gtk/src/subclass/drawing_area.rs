use glib::subclass::prelude::*;

use super::widget::WidgetImpl;
use crate::DrawingArea;
use crate::Widget;

pub trait DrawingAreaImpl: WidgetImpl {}

unsafe impl<T: DrawingAreaImpl> IsSubclassable<T> for DrawingArea {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);
    }
}
