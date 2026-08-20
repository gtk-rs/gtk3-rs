// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{SelectionData, TreeDragDest, TreePath, ffi, prelude::*, subclass::prelude::*};

pub trait TreeDragDestImpl: ObjectImpl + ObjectSubclass<Type: IsA<TreeDragDest>> {
    fn drag_data_received(&self, dest: &TreePath, selection_data: &SelectionData) -> bool;

    fn row_drop_possible(&self, dest: &TreePath, selection_data: &SelectionData) -> bool {
        self.parent_row_drop_possible(dest, selection_data)
    }
}

pub trait TreeDragDestImplExt: TreeDragDestImpl {
    fn parent_drag_data_received(&self, dest: &TreePath, selection_data: &SelectionData) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TreeDragDest>()
                as *const ffi::GtkTreeDragDestIface;

            let func = (*parent_iface)
                .drag_data_received
                .expect("no parent \"drag_data_received\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<TreeDragDest>()
                    .to_glib_none()
                    .0,
                mut_override(dest.to_glib_none().0),
                selection_data.to_glib_none().0,
            ))
        }
    }

    fn parent_row_drop_possible(&self, dest: &TreePath, selection_data: &SelectionData) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TreeDragDest>()
                as *const ffi::GtkTreeDragDestIface;

            if let Some(func) = (*parent_iface).row_drop_possible {
                from_glib(func(
                    self.obj()
                        .unsafe_cast_ref::<TreeDragDest>()
                        .to_glib_none()
                        .0,
                    mut_override(dest.to_glib_none().0),
                    selection_data.to_glib_none().0,
                ))
            } else {
                // same answer gtk_tree_drag_dest_row_drop_possible() gives for a NULL vfunc
                false
            }
        }
    }
}

impl<T: TreeDragDestImpl> TreeDragDestImplExt for T {}

unsafe impl<T: TreeDragDestImpl> IsImplementable<T> for TreeDragDest {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        iface.drag_data_received = Some(tree_drag_dest_drag_data_received::<T>);
        iface.row_drop_possible = Some(tree_drag_dest_row_drop_possible::<T>);
    }
}

unsafe extern "C" fn tree_drag_dest_drag_data_received<T: TreeDragDestImpl>(
    tree_drag_dest: *mut ffi::GtkTreeDragDest,
    destptr: *mut ffi::GtkTreePath,
    selectiondataptr: *mut ffi::GtkSelectionData,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(tree_drag_dest as *mut T::Instance);
        let imp = instance.imp();

        let dest: Borrowed<TreePath> = from_glib_borrow(destptr);
        let selection_data: Borrowed<SelectionData> = from_glib_borrow(selectiondataptr);

        imp.drag_data_received(&dest, &selection_data).into_glib()
    }
}

unsafe extern "C" fn tree_drag_dest_row_drop_possible<T: TreeDragDestImpl>(
    tree_drag_dest: *mut ffi::GtkTreeDragDest,
    destptr: *mut ffi::GtkTreePath,
    selectiondataptr: *mut ffi::GtkSelectionData,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(tree_drag_dest as *mut T::Instance);
        let imp = instance.imp();
        let dest: Borrowed<TreePath> = from_glib_borrow(destptr);
        let selection_data: Borrowed<SelectionData> = from_glib_borrow(selectiondataptr);

        imp.row_drop_possible(&dest, &selection_data).into_glib()
    }
}
