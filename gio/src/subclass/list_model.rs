// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ListModel;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, IsA, ObjectExt};
use once_cell::sync::Lazy;

pub trait ListModelImpl: ObjectImpl {
    fn get_item_type(&self, list_model: &Self::Type) -> glib::Type;
    fn get_n_items(&self, list_model: &Self::Type) -> u32;
    fn get_item(&self, list_model: &Self::Type, position: u32) -> Option<glib::Object>;
}

unsafe impl<T: ListModelImpl> IsImplementable<T> for ListModel
where
    <T as ObjectSubclass>::Type: IsA<glib::Object>,
{
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let list_model_iface = &mut *(iface as *mut ffi::GListModelInterface);

        list_model_iface.get_item_type = Some(list_model_get_item_type::<T>);
        list_model_iface.get_n_items = Some(list_model_get_n_items::<T>);
        list_model_iface.get_item = Some(list_model_get_item::<T>);
    }
}

static LIST_ITEM_TYPE_QUARK: Lazy<glib::Quark> =
    Lazy::new(|| glib::Quark::from_string("gtk-rs-subclass-list-model-item-type"));

unsafe extern "C" fn list_model_get_item_type<T: ListModelImpl>(
    list_model: *mut ffi::GListModel,
) -> glib::ffi::GType
where
    <T as ObjectSubclass>::Type: IsA<glib::Object>,
{
    let instance = &*(list_model as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow::<_, ListModel>(list_model);

    let type_ = imp.get_item_type(wrap.unsafe_cast_ref()).to_glib();

    // Store the type so we can enforce that it doesn't change.
    match wrap.get_qdata(*LIST_ITEM_TYPE_QUARK) {
        Some(old_type) => {
            assert_eq!(
                type_, *old_type,
                "ListModel's get_item_type cannot be changed"
            );
        }
        None => {
            wrap.set_qdata(*LIST_ITEM_TYPE_QUARK, type_);
        }
    }
    type_
}

unsafe extern "C" fn list_model_get_n_items<T: ListModelImpl>(
    list_model: *mut ffi::GListModel,
) -> u32
where
    <T as ObjectSubclass>::Type: IsA<glib::Object>,
{
    let instance = &*(list_model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_n_items(from_glib_borrow::<_, ListModel>(list_model).unsafe_cast_ref())
}

unsafe extern "C" fn list_model_get_item<T: ListModelImpl>(
    list_model: *mut ffi::GListModel,
    position: u32,
) -> *mut glib::gobject_ffi::GObject
where
    <T as ObjectSubclass>::Type: IsA<glib::Object>,
{
    let instance = &*(list_model as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow::<_, ListModel>(list_model);

    let item = imp.get_item(wrap.unsafe_cast_ref(), position);

    if let Some(ref i) = item {
        let type_ = imp.get_item_type(wrap.unsafe_cast_ref());
        assert!(
            type_.is_a(i.get_type()),
            "All ListModel items should be of the same type"
        );
    };
    item.to_glib_full()
}
