// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{Value, translate::*};

use crate::{TreeIter, TreeModel, TreeModelFilter, ffi, prelude::*, subclass::prelude::*};

pub trait TreeModelFilterImpl: ObjectImpl + ObjectSubclass<Type: IsA<TreeModelFilter>> {
    fn visible(&self, child_model: &TreeModel, child_iter: &TreeIter) -> bool {
        self.parent_visible(child_model, child_iter)
    }

    fn modify(&self, child_model: &TreeModel, iter: &TreeIter, column: i32) -> Value {
        self.parent_modify(child_model, iter, column)
    }
}

pub trait TreeModelFilterImplExt: TreeModelFilterImpl {
    // Whether the row indicated by iter is visible
    fn parent_visible(&self, child_model: &TreeModel, child_iter: &TreeIter) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeModelFilterClass;
            if let Some(f) = (*parent_class).visible {
                from_glib(f(
                    self.obj()
                        .unsafe_cast_ref::<TreeModelFilter>()
                        .to_glib_none()
                        .0,
                    child_model.to_glib_none().0,
                    mut_override(child_iter.to_glib_none().0),
                ))
            } else {
                true // always visible if not set
            }
        }
    }

    fn parent_modify(&self, child_model: &TreeModel, iter: &TreeIter, column: i32) -> Value {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTreeModelFilterClass;
            let f = (*parent_class)
                .modify
                .expect("No parent class impl for \"modify\"");
            let mut value = Value::uninitialized();
            f(
                self.obj()
                    .unsafe_cast_ref::<TreeModelFilter>()
                    .to_glib_none()
                    .0,
                child_model.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                value.to_glib_none_mut().0,
                column,
            );
            value
        }
    }
}

impl<T: TreeModelFilterImpl> TreeModelFilterImplExt for T {}

unsafe impl<T: TreeModelFilterImpl> IsSubclassable<T> for TreeModelFilter {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        let klass = class.as_mut();
        klass.visible = Some(tree_model_filter_visible::<T>);
        klass.modify = Some(tree_model_filter_modify::<T>);
    }
}

unsafe extern "C" fn tree_model_filter_visible<T: TreeModelFilterImpl>(
    ptr: *mut ffi::GtkTreeModelFilter,
    child_modelptr: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let child_model: Borrowed<TreeModel> = from_glib_borrow(child_modelptr);
        let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

        imp.visible(&child_model, &iter).into_glib()
    }
}

unsafe extern "C" fn tree_model_filter_modify<T: TreeModelFilterImpl>(
    ptr: *mut ffi::GtkTreeModelFilter,
    child_modelptr: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
    valueptr: *mut glib::gobject_ffi::GValue,
    column: i32,
) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let child_model: Borrowed<TreeModel> = from_glib_borrow(child_modelptr);
        let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

        let v = imp.modify(&child_model, &iter, column);

        // `valueptr` has not been initialized, so no need to unset first.  Then consume `v` and
        // transfer ownership of its bits to `valueptr`.
        std::ptr::write(valueptr, v.into_raw());
    }
}
