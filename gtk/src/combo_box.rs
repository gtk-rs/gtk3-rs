// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ComboBox;
use glib::object::IsA;
use glib::translate::*;

pub trait ComboBoxExtManual: 'static {
    /// Sets the active item of `self` to be the item at `index`.
    /// ## `index_`
    /// An index in the model passed during construction, or -1 to have
    /// no active item
    #[doc(alias = "gtk_combo_box_set_active")]
    fn set_active(&self, index_: Option<u32>);

    /// Returns the index of the currently active item, or -1 if there’s no
    /// active item. If the model is a non-flat treemodel, and the active item
    /// is not an immediate child of the root of the tree, this function returns
    /// `gtk_tree_path_get_indices (path)[0]`, where
    /// `path` is the [TreePath](crate::TreePath) of the active item.
    ///
    /// # Returns
    ///
    /// An integer which is the index of the currently active item,
    ///  or -1 if there’s no active item.
    #[doc(alias = "gtk_combo_box_get_active")]
    #[doc(alias = "get_active")]
    fn active(&self) -> Option<u32>;
}

impl<O: IsA<ComboBox>> ComboBoxExtManual for O {
    fn set_active(&self, index_: Option<u32>) {
        let index_ = match index_ {
            Some(i) => i as _,
            None => -1,
        };
        unsafe {
            ffi::gtk_combo_box_set_active(self.as_ref().to_glib_none().0, index_);
        }
    }

    fn active(&self) -> Option<u32> {
        match unsafe { ffi::gtk_combo_box_get_active(self.as_ref().to_glib_none().0) } {
            -1 => None,
            x => Some(x as _),
        }
    }
}
