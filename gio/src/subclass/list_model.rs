// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ListModel;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, IsA, ObjectExt};
use once_cell::sync::Lazy;

pub trait ListModelImpl: ObjectImpl {
    #[doc(alias = "get_item_type")]
    fn item_type(&self, list_model: &Self::Type) -> glib::Type;
    #[doc(alias = "get_n_items")]
    fn n_items(&self, list_model: &Self::Type) -> u32;
    #[doc(alias = "get_item")]
    fn item(&self, list_model: &Self::Type, position: u32) -> Option<glib::Object>;
}

pub trait ListModelImplExt: ObjectSubclass {
    fn parent_item_type(&self, list_model: &Self::Type) -> glib::Type;
    fn parent_n_items(&self, list_model: &Self::Type) -> u32;
    fn parent_item(&self, list_model: &Self::Type, position: u32) -> Option<glib::Object>;
}

impl<T: ListModelImpl> ListModelImplExt for T {
    fn parent_item_type(&self, list_model: &Self::Type) -> glib::Type {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ListModel>()
                as *const ffi::GListModelInterface;

            let func = (*parent_iface)
                .get_item_type
                .expect("no parent \"item_type\" implementation");
            let ret = func(list_model.unsafe_cast_ref::<ListModel>().to_glib_none().0);
            from_glib(ret)
        }
    }

    fn parent_n_items(&self, list_model: &Self::Type) -> u32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ListModel>()
                as *const ffi::GListModelInterface;

            let func = (*parent_iface)
                .get_n_items
                .expect("no parent \"n_items\" implementation");
            func(list_model.unsafe_cast_ref::<ListModel>().to_glib_none().0)
        }
    }

    fn parent_item(&self, list_model: &Self::Type, position: u32) -> Option<glib::Object> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ListModel>()
                as *const ffi::GListModelInterface;

            let func = (*parent_iface)
                .get_item
                .expect("no parent \"get_item\" implementation");
            let ret = func(
                list_model.unsafe_cast_ref::<ListModel>().to_glib_none().0,
                position,
            );
            from_glib_full(ret)
        }
    }
}

unsafe impl<T: ListModelImpl> IsImplementable<T> for ListModel
where
    <T as ObjectSubclass>::Type: IsA<glib::Object>,
{
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_item_type = Some(list_model_get_item_type::<T>);
        iface.get_n_items = Some(list_model_get_n_items::<T>);
        iface.get_item = Some(list_model_get_item::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
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
    let imp = instance.impl_();
    let wrap = from_glib_borrow::<_, ListModel>(list_model);

    let type_ = imp.item_type(wrap.unsafe_cast_ref()).into_glib();

    // Store the type so we can enforce that it doesn't change.
    match wrap.qdata(*LIST_ITEM_TYPE_QUARK) {
        Some(old_type) => {
            assert_eq!(
                type_,
                *old_type.as_ref(),
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
    let imp = instance.impl_();

    imp.n_items(from_glib_borrow::<_, ListModel>(list_model).unsafe_cast_ref())
}

unsafe extern "C" fn list_model_get_item<T: ListModelImpl>(
    list_model: *mut ffi::GListModel,
    position: u32,
) -> *mut glib::gobject_ffi::GObject
where
    <T as ObjectSubclass>::Type: IsA<glib::Object>,
{
    let instance = &*(list_model as *mut T::Instance);
    let imp = instance.impl_();
    let wrap = from_glib_borrow::<_, ListModel>(list_model);

    let item = imp.item(wrap.unsafe_cast_ref(), position);

    if let Some(ref i) = item {
        let type_ = imp.item_type(wrap.unsafe_cast_ref());
        assert!(
            type_.is_a(i.type_()),
            "All ListModel items should be of the same type"
        );
    };
    item.to_glib_full()
}
