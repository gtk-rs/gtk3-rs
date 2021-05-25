// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RadioButton;
use crate::Widget;
use glib::object::Cast;
use glib::translate::*;
use std::ptr;

impl RadioButton {
    /// Creates a new [RadioButton](crate::RadioButton). To be of any practical value, a widget should
    /// then be packed into the radio button.
    /// ## `group`
    /// an existing
    ///  radio button group, or [`None`] if you are creating a new group.
    ///
    /// # Returns
    ///
    /// a new radio button
    #[doc(alias = "gtk_radio_button_new")]
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_radio_button_new(ptr::null_mut())).unsafe_cast() }
    }

    /// Creates a new [RadioButton](crate::RadioButton) with a text label.
    /// ## `group`
    /// an existing
    ///  radio button group, or [`None`] if you are creating a new group.
    /// ## `label`
    /// the text label to display next to the radio button.
    ///
    /// # Returns
    ///
    /// a new radio button.
    #[doc(alias = "gtk_radio_button_new_with_label")]
    pub fn with_label(label: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_label(
                ptr::null_mut(),
                label.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    /// Creates a new [RadioButton](crate::RadioButton) containing a label, adding it to the same
    /// group as `group`. The label will be created using
    /// [Label::with_mnemonic](crate::Label::with_mnemonic), so underscores in `label` indicate the
    /// mnemonic for the button.
    /// ## `group`
    /// the radio button
    ///  group, or [`None`]
    /// ## `label`
    /// the text of the button, with an underscore in front of the
    ///  mnemonic character
    ///
    /// # Returns
    ///
    /// a new [RadioButton](crate::RadioButton)
    #[doc(alias = "gtk_radio_button_new_with_mnemonic")]
    pub fn with_mnemonic(label: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_mnemonic(
                ptr::null_mut(),
                label.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

impl Default for RadioButton {
    fn default() -> Self {
        Self::new()
    }
}
