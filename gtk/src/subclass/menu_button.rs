// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::toggle_button::ToggleButtonImpl;
use crate::MenuButton;

pub trait MenuButtonImpl: MenuButtonImplExt + ToggleButtonImpl {}

pub trait MenuButtonImplExt: ObjectSubclass {}

impl<T: MenuButtonImpl> MenuButtonImplExt for T {}

unsafe impl<T: MenuButtonImpl> IsSubclassable<T> for MenuButton {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
    }
}
