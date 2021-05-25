// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RadioMenuItem;
use crate::Widget;
use glib::object::Cast;
use glib::translate::*;
use std::ptr;

impl RadioMenuItem {
    /// Creates a new [RadioMenuItem](crate::RadioMenuItem).
    /// ## `group`
    /// the group to which the
    ///  radio menu item is to be attached, or [`None`]
    ///
    /// # Returns
    ///
    /// a new [RadioMenuItem](crate::RadioMenuItem)
    #[doc(alias = "gtk_radio_menu_item_new")]
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new(ptr::null_mut())).unsafe_cast()
        }
    }

    /// Creates a new [RadioMenuItem](crate::RadioMenuItem) whose child is a simple [Label](crate::Label).
    /// ## `group`
    ///
    ///  group the radio menu item is inside, or [`None`]
    /// ## `label`
    /// the text for the label
    ///
    /// # Returns
    ///
    /// A new [RadioMenuItem](crate::RadioMenuItem)
    #[doc(alias = "gtk_radio_menu_item_new_with_label")]
    pub fn with_label(label: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_label(
                ptr::null_mut(),
                label.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    /// Creates a new [RadioMenuItem](crate::RadioMenuItem) containing a label. The label
    /// will be created using [Label::with_mnemonic](crate::Label::with_mnemonic), so underscores
    /// in `label` indicate the mnemonic for the menu item.
    /// ## `group`
    ///
    ///  group the radio menu item is inside, or [`None`]
    /// ## `label`
    /// the text of the button, with an underscore in front of the
    ///  mnemonic character
    ///
    /// # Returns
    ///
    /// a new [RadioMenuItem](crate::RadioMenuItem)
    #[doc(alias = "gtk_radio_menu_item_new_with_mnemonic")]
    pub fn with_mnemonic(label: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_mnemonic(
                ptr::null_mut(),
                label.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

impl Default for RadioMenuItem {
    fn default() -> Self {
        Self::new()
    }
}
