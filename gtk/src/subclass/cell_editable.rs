// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`CellEditable`] interface.

use glib::translate::*;

use crate::{CellEditable, ffi, prelude::*, subclass::prelude::*};

pub trait CellEditableImpl: WidgetImpl + ObjectSubclass<Type: IsA<CellEditable>> {
    fn editing_done(&self) {
        self.parent_editing_done();
    }

    fn remove_widget(&self) {
        self.parent_remove_widget();
    }

    fn start_editing(&self, event: Option<&gdk::Event>) {
        self.parent_start_editing(event);
    }
}

pub trait CellEditableImplExt: CellEditableImpl {
    fn parent_editing_done(&self) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellEditable>()
                as *const ffi::GtkCellEditableIface;

            if let Some(func) = (*parent_iface).editing_done {
                func(
                    self.obj()
                        .unsafe_cast_ref::<CellEditable>()
                        .to_glib_none()
                        .0,
                );
            }
        }
    }

    fn parent_remove_widget(&self) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellEditable>()
                as *const ffi::GtkCellEditableIface;

            if let Some(func) = (*parent_iface).remove_widget {
                func(
                    self.obj()
                        .unsafe_cast_ref::<CellEditable>()
                        .to_glib_none()
                        .0,
                );
            }
        }
    }

    fn parent_start_editing(&self, event: Option<&gdk::Event>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellEditable>()
                as *const ffi::GtkCellEditableIface;

            let func = (*parent_iface)
                .start_editing
                .expect("no parent \"start_editing\" implementation");
            func(
                self.obj()
                    .unsafe_cast_ref::<CellEditable>()
                    .to_glib_none()
                    .0,
                event.to_glib_none().0,
            );
        }
    }
}

impl<T: CellEditableImpl> CellEditableImplExt for T {}

unsafe impl<T: CellEditableImpl> IsImplementable<T> for CellEditable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        iface.editing_done = Some(cell_editable_editing_done::<T>);
        iface.remove_widget = Some(cell_editable_remove_widget::<T>);
        iface.start_editing = Some(cell_editable_start_editing::<T>);
    }
}

unsafe extern "C" fn cell_editable_editing_done<T: CellEditableImpl>(
    celleditableptr: *mut ffi::GtkCellEditable,
) {
    assert!(!celleditableptr.is_null());

    let instance = unsafe { &*(celleditableptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.editing_done();
}

unsafe extern "C" fn cell_editable_remove_widget<T: CellEditableImpl>(
    celleditableptr: *mut ffi::GtkCellEditable,
) {
    assert!(!celleditableptr.is_null());

    let instance = unsafe { &*(celleditableptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.remove_widget();
}

unsafe extern "C" fn cell_editable_start_editing<T: CellEditableImpl>(
    celleditableptr: *mut ffi::GtkCellEditable,
    event_ptr: *mut gdk::ffi::GdkEvent,
) {
    assert!(!celleditableptr.is_null());

    let instance = unsafe { &*(celleditableptr as *mut T::Instance) };
    let imp = instance.imp();

    let event: Borrowed<Option<gdk::Event>> = unsafe { from_glib_borrow(event_ptr) };
    imp.start_editing((*event).as_ref());
}
