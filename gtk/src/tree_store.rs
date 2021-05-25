// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TreeIter;
use crate::TreeModel;
use crate::TreeStore;
use glib::object::{Cast, IsA};
use glib::translate::*;
use glib::{ToValue, Type, Value};
use libc::c_int;

impl TreeStore {
    /// Creates a new tree store as with `n_columns` columns each of the types passed
    /// in. Note that only types derived from standard GObject fundamental types
    /// are supported.
    ///
    /// As an example, `gtk_tree_store_new (3, G_TYPE_INT, G_TYPE_STRING,
    /// GDK_TYPE_PIXBUF);` will create a new [TreeStore](crate::TreeStore) with three columns, of type
    /// gint, gchararray, and [gdk_pixbuf::Pixbuf](crate::gdk_pixbuf::Pixbuf) respectively.
    /// ## `n_columns`
    /// number of columns in the tree store
    ///
    /// # Returns
    ///
    /// a new [TreeStore](crate::TreeStore)
    #[doc(alias = "gtk_tree_store_newv")]
    pub fn new(column_types: &[Type]) -> TreeStore {
        assert_initialized_main_thread!();
        unsafe {
            let mut column_types = column_types
                .iter()
                .map(|t| t.into_glib())
                .collect::<Vec<_>>();
            from_glib_full(ffi::gtk_tree_store_newv(
                column_types.len() as c_int,
                column_types.as_mut_ptr(),
            ))
        }
    }
}

pub trait TreeStoreExtManual: 'static {
    #[doc(alias = "gtk_tree_store_insert_with_valuesv")]
    fn insert_with_values(
        &self,
        parent: Option<&TreeIter>,
        position: Option<u32>,
        columns_and_values: &[(u32, &dyn ToValue)],
    ) -> TreeIter;

    /// Reorders the children of `parent` in `self` to follow the order
    /// indicated by `new_order`. Note that this function only works with
    /// unsorted stores.
    /// ## `parent`
    /// A [TreeIter](crate::TreeIter), or [`None`]
    /// ## `new_order`
    /// an array of integers mapping the new position of each child
    ///  to its old position before the re-ordering,
    ///  i.e. `new_order``[newpos] = oldpos`.
    #[doc(alias = "gtk_tree_store_reorder")]
    fn reorder(&self, parent: &TreeIter, new_order: &[u32]);

    /// Sets the value of one or more cells in the row referenced by `iter`.
    /// The variable argument list should contain integer column numbers,
    /// each column number followed by the value to be set.
    /// The list is terminated by a -1. For example, to set column 0 with type
    /// `G_TYPE_STRING` to “Foo”, you would write
    /// `gtk_tree_store_set (store, iter, 0, "Foo", -1)`.
    ///
    /// The value will be referenced by the store if it is a `G_TYPE_OBJECT`, and it
    /// will be copied if it is a `G_TYPE_STRING` or `G_TYPE_BOXED`.
    /// ## `iter`
    /// A valid [TreeIter](crate::TreeIter) for the row being modified
    #[doc(alias = "gtk_tree_store_set")]
    #[doc(alias = "gtk_tree_store_set_valuesv")]
    fn set(&self, iter: &TreeIter, columns_and_values: &[(u32, &dyn ToValue)]);

    /// Sets the data in the cell specified by `iter` and `column`.
    /// The type of `value` must be convertible to the type of the
    /// column.
    /// ## `iter`
    /// A valid [TreeIter](crate::TreeIter) for the row being modified
    /// ## `column`
    /// column number to modify
    /// ## `value`
    /// new value for the cell
    #[doc(alias = "gtk_tree_store_set_value")]
    fn set_value(&self, iter: &TreeIter, column: u32, value: &Value);
}

impl<O: IsA<TreeStore>> TreeStoreExtManual for O {
    fn insert_with_values(
        &self,
        parent: Option<&TreeIter>,
        position: Option<u32>,
        columns_and_values: &[(u32, &dyn ToValue)],
    ) -> TreeIter {
        unsafe {
            assert!(
                position.unwrap_or(0) <= i32::max_value() as u32,
                "can't have more than {} rows",
                i32::max_value()
            );
            let n_columns = ffi::gtk_tree_model_get_n_columns(
                self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
            ) as u32;
            assert!(
                columns_and_values.len() <= n_columns as usize,
                "got values for {} columns but only {} columns exist",
                columns_and_values.len(),
                n_columns
            );
            for (column, value) in columns_and_values {
                assert!(
                    *column < n_columns,
                    "got column {} which is higher than the number of columns {}",
                    *column,
                    n_columns
                );
                let type_ = from_glib(ffi::gtk_tree_model_get_column_type(
                    self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
                    *column as c_int,
                ));
                assert!(
                    Value::type_transformable(value.value_type(), type_),
                    "column {} is of type {} but found value of type {}",
                    *column,
                    type_,
                    value.value_type()
                );
            }

            let columns = columns_and_values
                .iter()
                .map(|(c, _)| *c)
                .collect::<Vec<_>>();
            let values = columns_and_values
                .iter()
                .map(|(_, v)| v.to_value())
                .collect::<Vec<_>>();

            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_with_valuesv(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(parent.to_glib_none().0),
                position.map_or(-1, |n| n as c_int),
                mut_override(columns.as_ptr() as *const c_int),
                mut_override(values.as_ptr() as *const glib::gobject_ffi::GValue),
                columns.len() as c_int,
            );
            iter
        }
    }

    fn reorder(&self, parent: &TreeIter, new_order: &[u32]) {
        unsafe {
            let count = ffi::gtk_tree_model_iter_n_children(
                self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
                mut_override(parent.to_glib_none().0),
            );
            let safe_count = count as usize == new_order.len();
            debug_assert!(
                safe_count,
                "Incorrect `new_order` slice length. Expected `{}`, found `{}`.",
                count,
                new_order.len()
            );
            let safe_values = new_order.iter().max().map_or(true, |&max| {
                let max = max as i32;
                max >= 0 && max < count
            });
            debug_assert!(
                safe_values,
                "Some `new_order` slice values are out of range. Maximum safe value: \
                 `{}`. The slice contents: `{:?}`",
                count - 1,
                new_order
            );
            if safe_count && safe_values {
                ffi::gtk_tree_store_reorder(
                    self.as_ref().to_glib_none().0,
                    mut_override(parent.to_glib_none().0),
                    mut_override(new_order.as_ptr() as *const c_int),
                );
            }
        }
    }

    fn set(&self, iter: &TreeIter, columns_and_values: &[(u32, &dyn ToValue)]) {
        unsafe {
            let n_columns = ffi::gtk_tree_model_get_n_columns(
                self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
            ) as u32;
            assert!(
                columns_and_values.len() <= n_columns as usize,
                "got values for {} columns but only {} columns exist",
                columns_and_values.len(),
                n_columns
            );
            for (column, value) in columns_and_values {
                assert!(
                    *column < n_columns,
                    "got column {} which is higher than the number of columns {}",
                    *column,
                    n_columns
                );
                let type_ = from_glib(ffi::gtk_tree_model_get_column_type(
                    self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
                    *column as c_int,
                ));
                assert!(
                    Value::type_transformable(value.value_type(), type_),
                    "column {} is of type {} but found value of type {}",
                    *column,
                    type_,
                    value.value_type()
                );
            }

            let columns = columns_and_values
                .iter()
                .map(|(c, _)| *c)
                .collect::<Vec<_>>();
            let values = columns_and_values
                .iter()
                .map(|(_, v)| v.to_value())
                .collect::<Vec<_>>();

            ffi::gtk_tree_store_set_valuesv(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                mut_override(columns.as_ptr() as *const c_int),
                mut_override(values.as_ptr() as *const glib::gobject_ffi::GValue),
                columns.len() as c_int,
            );
        }
    }

    fn set_value(&self, iter: &TreeIter, column: u32, value: &Value) {
        unsafe {
            let columns = ffi::gtk_tree_model_get_n_columns(
                self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
            ) as u32;
            assert!(
                column < columns,
                "got column {} which is higher than the number of columns {}",
                column,
                columns
            );

            let type_ = from_glib(ffi::gtk_tree_model_get_column_type(
                self.as_ref().upcast_ref::<TreeModel>().to_glib_none().0,
                column as c_int,
            ));
            assert!(
                Value::type_transformable(value.type_(), type_),
                "column {} is of type {} but found value of type {}",
                column,
                type_,
                value.type_()
            );

            ffi::gtk_tree_store_set_value(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                column as c_int,
                mut_override(value.to_glib_none().0),
            );
        }
    }
}
