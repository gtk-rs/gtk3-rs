// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{SelectionData, TreeDragSource, TreePath, ffi, prelude::*, subclass::prelude::*};

pub trait TreeDragSourceImpl: ObjectImpl + ObjectSubclass<Type: IsA<TreeDragSource>> {
    fn row_draggable(&self, path: &TreePath) -> bool {
        self.parent_row_draggable(path)
    }

    fn drag_data_get(&self, path: &TreePath, selection_data: &SelectionData) -> bool;

    fn drag_data_delete(&self, path: &TreePath) -> bool;
}

pub trait TreeDragSourceImplExt: TreeDragSourceImpl {
    // Returns true if the row can be dragged
    fn parent_row_draggable(&self, path: &TreePath) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TreeDragSource>()
                as *const ffi::GtkTreeDragSourceIface;

            if let Some(func) = (*parent_iface).row_draggable {
                from_glib(func(
                    self.obj()
                        .unsafe_cast_ref::<TreeDragSource>()
                        .to_glib_none()
                        .0,
                    mut_override(path.to_glib_none().0),
                ))
            } else {
                // Assume the row is draggable by default
                true
            }
        }
    }

    fn parent_drag_data_get(&self, path: &TreePath, selection_data: &SelectionData) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TreeDragSource>()
                as *const ffi::GtkTreeDragSourceIface;

            let func = (*parent_iface)
                .drag_data_get
                .expect("no parent \"drag_data_get\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<TreeDragSource>()
                    .to_glib_none()
                    .0,
                mut_override(path.to_glib_none().0),
                selection_data.to_glib_none().0,
            ))
        }
    }

    // True if the row was successfully deleted
    fn parent_drag_data_delete(&self, path: &TreePath) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TreeDragSource>()
                as *const ffi::GtkTreeDragSourceIface;

            let func = (*parent_iface)
                .drag_data_delete
                .expect("no parent \"drag_data_delete\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<TreeDragSource>()
                    .to_glib_none()
                    .0,
                mut_override(path.to_glib_none().0),
            ))
        }
    }
}

impl<T: TreeDragSourceImpl> TreeDragSourceImplExt for T {}

unsafe impl<T: TreeDragSourceImpl> IsImplementable<T> for TreeDragSource {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        iface.row_draggable = Some(tree_drag_source_row_draggable::<T>);
        iface.drag_data_get = Some(tree_drag_source_drag_data_get::<T>);
        iface.drag_data_delete = Some(tree_drag_source_drag_data_delete::<T>);
    }
}

unsafe extern "C" fn tree_drag_source_row_draggable<T: TreeDragSourceImpl>(
    tree_drag_source: *mut ffi::GtkTreeDragSource,
    pathptr: *mut ffi::GtkTreePath,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(tree_drag_source as *mut T::Instance);
        let imp = instance.imp();

        let path: Borrowed<TreePath> = from_glib_borrow(pathptr);

        imp.row_draggable(&path).into_glib()
    }
}

unsafe extern "C" fn tree_drag_source_drag_data_get<T: TreeDragSourceImpl>(
    tree_drag_source: *mut ffi::GtkTreeDragSource,
    pathptr: *mut ffi::GtkTreePath,
    selectiondataptr: *mut ffi::GtkSelectionData,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(tree_drag_source as *mut T::Instance);
        let imp = instance.imp();
        let path: Borrowed<TreePath> = from_glib_borrow(pathptr);
        let selection_data: Borrowed<SelectionData> = from_glib_borrow(selectiondataptr);

        imp.drag_data_get(&path, &selection_data).into_glib()
    }
}

unsafe extern "C" fn tree_drag_source_drag_data_delete<T: TreeDragSourceImpl>(
    tree_drag_source: *mut ffi::GtkTreeDragSource,
    pathptr: *mut ffi::GtkTreePath,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(tree_drag_source as *mut T::Instance);
        let imp = instance.imp();
        let path: Borrowed<TreePath> = from_glib_borrow(pathptr);
        imp.drag_data_delete(&path).into_glib()
    }
}
