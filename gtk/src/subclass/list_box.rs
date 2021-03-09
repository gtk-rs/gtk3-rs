// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::{container::ContainerImpl, widget::WidgetImpl};

use libc::c_int;

use crate::Container;
use crate::ListBox;
use crate::ListBoxRow;
use crate::MovementStep;

pub trait ListBoxImpl: ListBoxImplExt + ContainerImpl + WidgetImpl {
    fn activate_cursor_row(&self, list_box: &Self::Type) {
        self.list_box_activate_cursor_row(list_box)
    }

    fn move_cursor(&self, list_box: &Self::Type, step: MovementStep, count: i32) {
        self.list_box_move_cursor(list_box, step, count)
    }

    fn row_activated(&self, list_box: &Self::Type, row: &ListBoxRow) {
        self.list_box_row_activated(list_box, row)
    }

    fn row_selected(&self, list_box: &Self::Type, row: Option<&ListBoxRow>) {
        self.list_box_row_selected(list_box, row)
    }

    fn select_all(&self, list_box: &Self::Type) {
        self.list_box_select_all(list_box)
    }

    fn selected_rows_changed(&self, list_box: &Self::Type) {
        self.list_box_selected_rows_changed(list_box)
    }

    fn toggle_cursor_row(&self, list_box: &Self::Type) {
        self.list_box_toggle_cursor_row(list_box)
    }

    fn unselect_all(&self, list_box: &Self::Type) {
        self.list_box_unselect_all(list_box)
    }
}

pub trait ListBoxImplExt: ObjectSubclass {
    fn list_box_activate_cursor_row(&self, list_box: &Self::Type);
    fn list_box_move_cursor(&self, list_box: &Self::Type, step: MovementStep, count: i32);
    fn list_box_row_activated(&self, list_box: &Self::Type, row: &ListBoxRow);
    fn list_box_row_selected(&self, list_box: &Self::Type, row: Option<&ListBoxRow>);
    fn list_box_select_all(&self, list_box: &Self::Type);
    fn list_box_selected_rows_changed(&self, list_box: &Self::Type);
    fn list_box_toggle_cursor_row(&self, list_box: &Self::Type);
    fn list_box_unselect_all(&self, list_box: &Self::Type);
}

impl<T: ListBoxImpl> ListBoxImplExt for T {
    fn list_box_activate_cursor_row(&self, list_box: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkListBoxClass;
            if let Some(f) = (*parent_class).activate_cursor_row {
                f(list_box.unsafe_cast_ref::<ListBox>().to_glib_none().0)
            }
        }
    }

    fn list_box_move_cursor(&self, list_box: &Self::Type, step: MovementStep, count: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkListBoxClass;
            if let Some(f) = (*parent_class).move_cursor {
                f(
                    list_box.unsafe_cast_ref::<ListBox>().to_glib_none().0,
                    step.to_glib(),
                    count,
                );
            }
        }
    }

    fn list_box_row_activated(&self, list_box: &Self::Type, row: &ListBoxRow) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkListBoxClass;
            if let Some(f) = (*parent_class).row_activated {
                f(
                    list_box.unsafe_cast_ref::<ListBox>().to_glib_none().0,
                    row.to_glib_none().0,
                )
            }
        }
    }

    fn list_box_row_selected(&self, list_box: &Self::Type, row: Option<&ListBoxRow>) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkListBoxClass;
            if let Some(f) = (*parent_class).row_selected {
                f(
                    list_box.unsafe_cast_ref::<ListBox>().to_glib_none().0,
                    mut_override(row.to_glib_none().0),
                )
            }
        }
    }

    fn list_box_select_all(&self, list_box: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkListBoxClass;
            if let Some(f) = (*parent_class).select_all {
                f(list_box.unsafe_cast_ref::<ListBox>().to_glib_none().0)
            }
        }
    }

    fn list_box_selected_rows_changed(&self, list_box: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkListBoxClass;
            if let Some(f) = (*parent_class).selected_rows_changed {
                f(list_box.unsafe_cast_ref::<ListBox>().to_glib_none().0)
            }
        }
    }

    fn list_box_toggle_cursor_row(&self, list_box: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkListBoxClass;
            if let Some(f) = (*parent_class).toggle_cursor_row {
                f(list_box.unsafe_cast_ref::<ListBox>().to_glib_none().0)
            }
        }
    }

    fn list_box_unselect_all(&self, list_box: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkListBoxClass;
            if let Some(f) = (*parent_class).unselect_all {
                f(list_box.unsafe_cast_ref::<ListBox>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: ListBoxImpl> IsSubclassable<T> for ListBox {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <Container as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.activate_cursor_row = Some(list_box_activate_cursor_row::<T>);
        klass.move_cursor = Some(list_box_move_cursor::<T>);
        klass.row_activated = Some(list_box_row_activated::<T>);
        klass.row_selected = Some(list_box_row_selected::<T>);
        klass.select_all = Some(list_box_select_all::<T>);
        klass.selected_rows_changed = Some(list_box_selected_rows_changed::<T>);
        klass.toggle_cursor_row = Some(list_box_toggle_cursor_row::<T>);
        klass.unselect_all = Some(list_box_unselect_all::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Container as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn list_box_activate_cursor_row<T: ListBoxImpl>(ptr: *mut ffi::GtkListBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);

    imp.activate_cursor_row(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn list_box_move_cursor<T: ListBoxImpl>(
    ptr: *mut ffi::GtkListBox,
    step: ffi::GtkMovementStep,
    count: c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);

    imp.move_cursor(wrap.unsafe_cast_ref(), from_glib(step), count)
}

unsafe extern "C" fn list_box_row_activated<T: ListBoxImpl>(
    ptr: *mut ffi::GtkListBox,
    rowptr: *mut ffi::GtkListBoxRow,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);
    let row: Borrowed<ListBoxRow> = from_glib_borrow(rowptr);

    imp.row_activated(wrap.unsafe_cast_ref(), &row)
}

unsafe extern "C" fn list_box_row_selected<T: ListBoxImpl>(
    ptr: *mut ffi::GtkListBox,
    rowptr: *mut ffi::GtkListBoxRow,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);
    let row: Borrowed<Option<ListBoxRow>> = from_glib_borrow(rowptr);

    imp.row_selected(wrap.unsafe_cast_ref(), row.as_ref().as_ref())
}

unsafe extern "C" fn list_box_select_all<T: ListBoxImpl>(ptr: *mut ffi::GtkListBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);

    imp.select_all(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn list_box_selected_rows_changed<T: ListBoxImpl>(ptr: *mut ffi::GtkListBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);

    imp.selected_rows_changed(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn list_box_toggle_cursor_row<T: ListBoxImpl>(ptr: *mut ffi::GtkListBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);

    imp.toggle_cursor_row(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn list_box_unselect_all<T: ListBoxImpl>(ptr: *mut ffi::GtkListBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBox> = from_glib_borrow(ptr);

    imp.unselect_all(wrap.unsafe_cast_ref())
}
