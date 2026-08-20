// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{
    object::{Cast, IsA},
    subclass::prelude::*,
    translate::*,
};

use super::container::ContainerImpl;

use crate::{MovementStep, TreeIter, TreePath, TreeView, TreeViewColumn, ffi};

pub trait TreeViewImpl: ContainerImpl + ObjectSubclass<Type: IsA<TreeView>> {
    fn row_activated(&self, path: &TreePath, column: Option<&TreeViewColumn>) {
        self.parent_row_activated(path, column);
    }

    fn test_expand_row(&self, iter: &TreeIter, path: &TreePath) -> bool {
        self.parent_test_expand_row(iter, path)
    }

    fn test_collapse_row(&self, iter: &TreeIter, path: &TreePath) -> bool {
        self.parent_test_collapse_row(iter, path)
    }

    fn row_expanded(&self, iter: &TreeIter, path: &TreePath) {
        self.parent_row_expanded(iter, path);
    }

    fn row_collapsed(&self, iter: &TreeIter, path: &TreePath) {
        self.parent_row_collapsed(iter, path);
    }

    fn columns_changed(&self) {
        self.parent_columns_changed();
    }

    fn cursor_changed(&self) {
        self.parent_cursor_changed();
    }

    fn move_cursor(&self, step: MovementStep, count: i32) -> bool {
        self.parent_move_cursor(step, count)
    }

    fn select_all(&self) -> bool {
        self.parent_select_all()
    }

    fn unselect_all(&self) -> bool {
        self.parent_unselect_all()
    }

    fn select_cursor_row(&self, start_editing: bool) -> bool {
        self.parent_select_cursor_row(start_editing)
    }

    fn toggle_cursor_row(&self) -> bool {
        self.parent_toggle_cursor_row()
    }

    fn expand_collapse_cursor_row(&self, logical: bool, expand: bool, open_all: bool) -> bool {
        self.parent_expand_collapse_cursor_row(logical, expand, open_all)
    }

    fn select_cursor_parent(&self) -> bool {
        self.parent_select_cursor_parent()
    }

    fn start_interactive_search(&self) -> bool {
        self.parent_start_interactive_search()
    }
}

pub trait TreeViewImplExt: TreeViewImpl {
    fn parent_row_activated(&self, path: &TreePath, column: Option<&TreeViewColumn>) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).row_activated {
                f(
                    self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    path.to_glib_none().0,
                    column.to_glib_none().0,
                )
            }
        }
    }

    fn parent_test_expand_row(&self, iter: &TreeIter, path: &TreePath) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).test_expand_row {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    mut_override(iter.to_glib_none().0),
                    path.to_glib_none().0,
                ))
            } else {
                false
            }
        }
    }

    fn parent_test_collapse_row(&self, iter: &TreeIter, path: &TreePath) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).test_collapse_row {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    mut_override(iter.to_glib_none().0),
                    path.to_glib_none().0,
                ))
            } else {
                false
            }
        }
    }

    fn parent_row_expanded(&self, iter: &TreeIter, path: &TreePath) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).row_expanded {
                f(
                    self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    mut_override(iter.to_glib_none().0),
                    path.to_glib_none().0,
                )
            }
        }
    }

    fn parent_row_collapsed(&self, iter: &TreeIter, path: &TreePath) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).row_collapsed {
                f(
                    self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    mut_override(iter.to_glib_none().0),
                    path.to_glib_none().0,
                )
            }
        }
    }

    fn parent_columns_changed(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).columns_changed {
                f(self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0);
            }
        }
    }

    fn parent_cursor_changed(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).cursor_changed {
                f(self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0);
            }
        }
    }

    fn parent_move_cursor(&self, step: MovementStep, count: i32) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).move_cursor {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    step.into_glib(),
                    count,
                ))
            } else {
                false
            }
        }
    }

    fn parent_select_all(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).select_all {
                from_glib(f(self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_unselect_all(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).unselect_all {
                from_glib(f(self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_select_cursor_row(&self, start_editing: bool) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).select_cursor_row {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    start_editing.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_toggle_cursor_row(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).toggle_cursor_row {
                from_glib(f(self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_expand_collapse_cursor_row(
        &self,
        logical: bool,
        expand: bool,
        open_all: bool,
    ) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).expand_collapse_cursor_row {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0,
                    logical.into_glib(),
                    expand.into_glib(),
                    open_all.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_select_cursor_parent(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).select_cursor_parent {
                from_glib(f(self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_start_interactive_search(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeViewClass;
            if let Some(f) = (*parent_class).start_interactive_search {
                from_glib(f(self.obj().unsafe_cast_ref::<TreeView>().to_glib_none().0))
            } else {
                false
            }
        }
    }
}

impl<T: TreeViewImpl> TreeViewImplExt for T {}

unsafe impl<T: TreeViewImpl> IsSubclassable<T> for TreeView {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        let klass = class.as_mut();
        klass.row_activated = Some(tree_view_row_activated::<T>);
        klass.test_expand_row = Some(tree_view_test_expand_row::<T>);
        klass.test_collapse_row = Some(tree_view_test_collapse_row::<T>);
        klass.row_expanded = Some(tree_view_row_expanded::<T>);
        klass.row_collapsed = Some(tree_view_row_collapsed::<T>);
        klass.columns_changed = Some(tree_view_columns_changed::<T>);
        klass.cursor_changed = Some(tree_view_cursor_changed::<T>);
        klass.move_cursor = Some(tree_view_move_cursor::<T>);
        klass.select_all = Some(tree_view_select_all::<T>);
        klass.unselect_all = Some(tree_view_unselect_all::<T>);
        klass.select_cursor_row = Some(tree_view_select_cursor_row::<T>);
        klass.toggle_cursor_row = Some(tree_view_toggle_cursor_row::<T>);
        klass.expand_collapse_cursor_row = Some(tree_view_expand_collapse_cursor_row::<T>);
        klass.select_cursor_parent = Some(tree_view_select_cursor_parent::<T>);
        klass.start_interactive_search = Some(tree_view_start_interactive_search::<T>);
    }
}

unsafe extern "C" fn tree_view_row_activated<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    pathptr: *mut ffi::GtkTreePath,
    columnptr: *mut ffi::GtkTreeViewColumn,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let column: Option<Borrowed<TreeViewColumn>> = if columnptr.is_null() {
            None
        } else {
            Some(from_glib_borrow(columnptr))
        };
        imp.row_activated(&from_glib_borrow(pathptr), column.as_deref());
    }
}

unsafe extern "C" fn tree_view_test_expand_row<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    iterptr: *mut ffi::GtkTreeIter,
    pathptr: *mut ffi::GtkTreePath,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.test_expand_row(&from_glib_borrow(iterptr), &from_glib_borrow(pathptr))
            .into_glib()
    }
}

unsafe extern "C" fn tree_view_test_collapse_row<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    iterptr: *mut ffi::GtkTreeIter,
    pathptr: *mut ffi::GtkTreePath,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.test_collapse_row(&from_glib_borrow(iterptr), &from_glib_borrow(pathptr))
            .into_glib()
    }
}

unsafe extern "C" fn tree_view_row_expanded<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    iterptr: *mut ffi::GtkTreeIter,
    pathptr: *mut ffi::GtkTreePath,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.row_expanded(&from_glib_borrow(iterptr), &from_glib_borrow(pathptr));
    }
}

unsafe extern "C" fn tree_view_row_collapsed<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    iterptr: *mut ffi::GtkTreeIter,
    pathptr: *mut ffi::GtkTreePath,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.row_collapsed(&from_glib_borrow(iterptr), &from_glib_borrow(pathptr));
    }
}

unsafe extern "C" fn tree_view_columns_changed<T: TreeViewImpl>(ptr: *mut ffi::GtkTreeView) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.columns_changed();
    }
}

unsafe extern "C" fn tree_view_cursor_changed<T: TreeViewImpl>(ptr: *mut ffi::GtkTreeView) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.cursor_changed();
    }
}

unsafe extern "C" fn tree_view_move_cursor<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    step: ffi::GtkMovementStep,
    count: glib::ffi::gint,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.move_cursor(from_glib(step), count).into_glib()
    }
}

unsafe extern "C" fn tree_view_select_all<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.select_all().into_glib()
    }
}

unsafe extern "C" fn tree_view_unselect_all<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.unselect_all().into_glib()
    }
}

unsafe extern "C" fn tree_view_select_cursor_row<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    start_editing: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.select_cursor_row(from_glib(start_editing)).into_glib()
    }
}

unsafe extern "C" fn tree_view_toggle_cursor_row<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.toggle_cursor_row().into_glib()
    }
}

unsafe extern "C" fn tree_view_expand_collapse_cursor_row<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
    logical: glib::ffi::gboolean,
    expand: glib::ffi::gboolean,
    open_all: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.expand_collapse_cursor_row(from_glib(logical), from_glib(expand), from_glib(open_all))
            .into_glib()
    }
}

unsafe extern "C" fn tree_view_select_cursor_parent<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.select_cursor_parent().into_glib()
    }
}

unsafe extern "C" fn tree_view_start_interactive_search<T: TreeViewImpl>(
    ptr: *mut ffi::GtkTreeView,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        imp.start_interactive_search().into_glib()
    }
}
