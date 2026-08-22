// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`CellLayout`] interface.

use std::{ffi::CStr, mem::ManuallyDrop};

use glib::translate::*;

use crate::{
    CellArea, CellLayout, CellRenderer, TreeIter, TreeModel, ffi, prelude::*, subclass::prelude::*,
};

// rustdoc-stripper-ignore-next
/// A cell data func set on a [`CellLayout`]'s [`CellRenderer`].
///
/// When implementing [`CellLayoutImpl::set_cell_data_func`], you will need to store these, keyed
/// to the passed [`CellRenderer`], somewhere in your class's instance data.  Whenever your
/// implementation needs to set attribute values on the cell renderers for a particular
/// [`TreeIter`], you should use [`CellDataFunc::call`] in order to do so, if there is a
/// [`CellDataFunc`] instance present for that cell renderer.
///
/// ## Example
///
/// ```ignore
/// # use std::{cell::RefCell, collections::HashMap};
/// # use gtk::{CellRenderer, TreeIter, TreeModel, subclass::prelude::*};
/// #
/// struct MyWidget {
///    model: RefCell<TreeModel>,
///    items: RefCell<Vec<MyItem>>,
///    cells: RefCell<Vec<CellRenderer>>,
///    cell_data_funcs: RefCell<HashMap<CellRenderer, CellDataFunc>>,
///    // ... other fields
/// }
///
/// impl CellLayoutImpl for MyWidget {
///     fn set_cell_data_func(&self, cell: &CellRenderer, cell_data_func: Option<CellDataFunc>) {
///         // Store or clear the passed CellDataFunc.
///         if let Some(cell_data_func) = cell_data_func {
///             self.cell_data_funcs.borrow_mut().insert(cell.clone(), cell_data_func);
///         } else {
///             self.cell_data_funcs.borrow_mut().remove(cell);
///         }
///         // Things have changed, so redraw.
///         self.redraw();
///     }
/// }
///
/// impl MyWidget {
///     fn redraw(&self) {
///         for item in &*self.items.borrow() {
///             let iter: TreeIter = item.iter(&*self.model.borrow());
///             for cell in &*self.cells.borrow() {
///                 if let Some(cell_data_func) = self.cell_data_funcs.borrow().get(cell) {
///                     // There's a CellDataFunc for this CellRenderer, so call it so it can set
///                     // renderer attributes for the item at this iter.
///                     cell_data_func.call(&*self.obj(), cell, &*self.model.borrow(), &iter);
///                 }
///             }
///             item.draw(&*self.cells.borrow());
///         }
///     }
/// }
/// ```
pub struct CellDataFunc {
    func: ffi::GtkCellLayoutDataFunc,
    func_data: glib::ffi::gpointer,
    destroy: glib::ffi::GDestroyNotify,
}

impl CellDataFunc {
    // rustdoc-stripper-ignore-next
    /// Calls the data func on the specified cell renderer, tree model, and iter.
    ///
    /// Usually this will set up `cell`'s attribute values correctly for `iter`.
    pub fn call(
        &self,
        cell_layout: &impl IsA<CellLayout>,
        cell: &impl IsA<CellRenderer>,
        model: &impl IsA<TreeModel>,
        iter: &TreeIter,
    ) {
        if let Some(func) = self.func.as_ref() {
            unsafe {
                func(
                    cell_layout.as_ref().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                    model.as_ref().to_glib_none().0,
                    mut_override(iter.to_glib_none().0),
                    self.func_data,
                );
            }
        }
    }
}

impl Drop for CellDataFunc {
    fn drop(&mut self) {
        if let Some(destroy_ptr) = self.destroy.take() {
            unsafe {
                destroy_ptr(self.func_data);
            }
        }
    }
}

pub trait CellLayoutImpl: ObjectImpl + ObjectSubclass<Type: IsA<CellLayout>> {
    fn pack_start(&self, cell: &CellRenderer, expand: bool) {
        self.parent_pack_start(cell, expand);
    }

    fn pack_end(&self, cell: &CellRenderer, expand: bool) {
        self.parent_pack_end(cell, expand);
    }

    fn clear(&self) {
        self.parent_clear();
    }

    fn add_attribute(&self, cell: &CellRenderer, attribute: &str, column: i32) {
        self.parent_add_attribute(cell, attribute, column);
    }

    fn set_cell_data_func(&self, cell: &CellRenderer, cell_data_func: Option<CellDataFunc>) {
        self.parent_set_cell_data_func(cell, cell_data_func);
    }

    fn clear_attributes(&self, cell: &CellRenderer) {
        self.parent_clear_attributes(cell);
    }

    fn reorder(&self, cell: &CellRenderer, position: i32) {
        self.parent_reorder(cell, position);
    }

    #[doc(alias = "get_cells")]
    fn cells(&self) -> Vec<CellRenderer> {
        self.parent_cells()
    }

    #[doc(alias = "get_area")]
    fn area(&self) -> Option<CellArea> {
        self.parent_area()
    }
}

pub trait CellLayoutImplExt: CellLayoutImpl {
    fn parent_pack_start(&self, cell: &CellRenderer, expand: bool) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            let func = (*parent_iface)
                .pack_start
                .expect("no parent \"pack_start\" implementation");
            func(
                self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                cell.to_glib_none().0,
                expand.into_glib(),
            );
        }
    }

    fn parent_pack_end(&self, cell: &CellRenderer, expand: bool) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            let func = (*parent_iface)
                .pack_end
                .expect("no parent \"pack_end\" implementation");
            func(
                self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                cell.to_glib_none().0,
                expand.into_glib(),
            );
        }
    }

    fn parent_clear(&self) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            let func = (*parent_iface)
                .clear
                .expect("no parent \"clear\" implementation");
            func(self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0);
        }
    }

    fn parent_add_attribute(&self, cell: &CellRenderer, attribute: &str, column: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            let func = (*parent_iface)
                .add_attribute
                .expect("no parent \"add_attribute\" implementation");
            func(
                self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                cell.to_glib_none().0,
                attribute.to_glib_none().0,
                column,
            );
        }
    }

    fn parent_set_cell_data_func(&self, cell: &CellRenderer, cell_data_func: Option<CellDataFunc>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            let func = (*parent_iface)
                .set_cell_data_func
                .expect("no parent \"set_cell_data_func\" implementation");

            let (data_func, data_ptr, destroy_func) = if let Some(cell_data_func) = cell_data_func {
                // Wrap in ManuallyDrop because we are transferring ownership to the parent class,
                // and if we drop the `CellDataFunc` struct, then the `GDestroyNotify` will run.
                let cell_data_func = ManuallyDrop::new(cell_data_func);
                (
                    cell_data_func.func,
                    cell_data_func.func_data,
                    cell_data_func.destroy,
                )
            } else {
                (None, std::ptr::null_mut(), None)
            };

            func(
                self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                cell.to_glib_none().0,
                data_func,
                data_ptr,
                destroy_func,
            );
        }
    }

    fn parent_clear_attributes(&self, cell: &CellRenderer) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            let func = (*parent_iface)
                .clear_attributes
                .expect("no parent \"clear_attributes\" implementation");
            func(
                self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                cell.to_glib_none().0,
            );
        }
    }

    fn parent_reorder(&self, cell: &CellRenderer, position: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            let func = (*parent_iface)
                .reorder
                .expect("no parent \"reorder\" implementation");
            func(
                self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                cell.to_glib_none().0,
                position,
            );
        }
    }

    fn parent_cells(&self) -> Vec<CellRenderer> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            let func = (*parent_iface)
                .get_cells
                .expect("no parent \"get_cells\" implementation");
            FromGlibPtrContainer::from_glib_container(func(
                self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
            ))
        }
    }

    fn parent_area(&self) -> Option<CellArea> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            (*parent_iface).get_area.and_then(|func| {
                from_glib_none(func(
                    self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                ))
            })
        }
    }
}

impl<T: CellLayoutImpl> CellLayoutImplExt for T {}

unsafe impl<T: CellLayoutImpl> IsImplementable<T> for CellLayout {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        iface.pack_start = Some(cell_layout_pack_start::<T>);
        iface.pack_end = Some(cell_layout_pack_end::<T>);
        iface.clear = Some(cell_layout_clear::<T>);
        iface.add_attribute = Some(cell_layout_add_attribute::<T>);
        iface.set_cell_data_func = Some(cell_layout_set_cell_data_func::<T>);
        iface.clear_attributes = Some(cell_layout_clear_attributes::<T>);
        iface.reorder = Some(cell_layout_reorder::<T>);
        iface.get_cells = Some(cell_layout_get_cells::<T>);
        iface.get_area = Some(cell_layout_get_area::<T>);
    }
}

unsafe extern "C" fn cell_layout_pack_start<T: CellLayoutImpl>(
    cell_layout_ptr: *mut ffi::GtkCellLayout,
    cell_ptr: *mut ffi::GtkCellRenderer,
    expand: glib::ffi::gboolean,
) {
    assert!(!cell_layout_ptr.is_null());
    assert!(!cell_ptr.is_null());

    let instance = unsafe { &*(cell_layout_ptr as *mut T::Instance) };
    let imp = instance.imp();
    unsafe {
        imp.pack_start(&from_glib_borrow(cell_ptr), from_glib(expand));
    }
}

unsafe extern "C" fn cell_layout_pack_end<T: CellLayoutImpl>(
    cell_layout_ptr: *mut ffi::GtkCellLayout,
    cell_ptr: *mut ffi::GtkCellRenderer,
    expand: glib::ffi::gboolean,
) {
    assert!(!cell_layout_ptr.is_null());
    assert!(!cell_ptr.is_null());

    let instance = unsafe { &*(cell_layout_ptr as *mut T::Instance) };
    let imp = instance.imp();
    unsafe {
        imp.pack_end(&from_glib_borrow(cell_ptr), from_glib(expand));
    }
}

unsafe extern "C" fn cell_layout_clear<T: CellLayoutImpl>(
    cell_layout_ptr: *mut ffi::GtkCellLayout,
) {
    assert!(!cell_layout_ptr.is_null());

    let instance = unsafe { &*(cell_layout_ptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.clear();
}

unsafe extern "C" fn cell_layout_add_attribute<T: CellLayoutImpl>(
    cell_layout_ptr: *mut ffi::GtkCellLayout,
    cell_ptr: *mut ffi::GtkCellRenderer,
    attribute_ptr: *const glib::ffi::gchar,
    column: i32,
) {
    assert!(!cell_layout_ptr.is_null());
    assert!(!cell_ptr.is_null());
    assert!(!attribute_ptr.is_null());

    let instance = unsafe { &*(cell_layout_ptr as *mut T::Instance) };
    let imp = instance.imp();
    unsafe {
        let attribute = CStr::from_ptr(attribute_ptr).to_str().unwrap();
        imp.add_attribute(&from_glib_borrow(cell_ptr), attribute, column);
    }
}

unsafe extern "C" fn cell_layout_set_cell_data_func<T: CellLayoutImpl>(
    cell_layout_ptr: *mut ffi::GtkCellLayout,
    cell_ptr: *mut ffi::GtkCellRenderer,
    func_ptr: ffi::GtkCellLayoutDataFunc,
    func_data_ptr: glib::ffi::gpointer,
    destroy_ptr: glib::ffi::GDestroyNotify,
) {
    assert!(!cell_layout_ptr.is_null());
    assert!(!cell_ptr.is_null());

    let instance = unsafe { &*(cell_layout_ptr as *mut T::Instance) };
    let imp = instance.imp();

    let cell = unsafe { from_glib_borrow(cell_ptr) };
    let cell_data_func = func_ptr.is_some().then(|| CellDataFunc {
        func: func_ptr,
        func_data: func_data_ptr,
        destroy: destroy_ptr,
    });

    imp.set_cell_data_func(&cell, cell_data_func);
}

unsafe extern "C" fn cell_layout_clear_attributes<T: CellLayoutImpl>(
    cell_layout_ptr: *mut ffi::GtkCellLayout,
    cell_ptr: *mut ffi::GtkCellRenderer,
) {
    assert!(!cell_layout_ptr.is_null());
    assert!(!cell_ptr.is_null());

    let instance = unsafe { &*(cell_layout_ptr as *mut T::Instance) };
    let imp = instance.imp();
    unsafe {
        imp.clear_attributes(&from_glib_borrow(cell_ptr));
    }
}

unsafe extern "C" fn cell_layout_reorder<T: CellLayoutImpl>(
    cell_layout_ptr: *mut ffi::GtkCellLayout,
    cell_ptr: *mut ffi::GtkCellRenderer,
    position: i32,
) {
    assert!(!cell_layout_ptr.is_null());
    assert!(!cell_ptr.is_null());

    let instance = unsafe { &*(cell_layout_ptr as *mut T::Instance) };
    let imp = instance.imp();
    unsafe {
        imp.reorder(&from_glib_borrow(cell_ptr), position);
    }
}

unsafe extern "C" fn cell_layout_get_cells<T: CellLayoutImpl>(
    cell_layout_ptr: *mut ffi::GtkCellLayout,
) -> *mut glib::ffi::GList {
    assert!(!cell_layout_ptr.is_null());

    let instance = unsafe { &*(cell_layout_ptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.cells().to_glib_container().0
}

unsafe extern "C" fn cell_layout_get_area<T: CellLayoutImpl>(
    cell_layout_ptr: *mut ffi::GtkCellLayout,
) -> *mut ffi::GtkCellArea {
    assert!(!cell_layout_ptr.is_null());

    let instance = unsafe { &*(cell_layout_ptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.area().to_glib_none().0
}
