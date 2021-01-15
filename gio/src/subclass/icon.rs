// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Icon;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, IsA};

pub trait IconImpl: ObjectImpl {
    fn hash(&self, icon: &Self::Type) -> u32;
    fn equal(&self, icon: &Self::Type, other_icon: &Icon) -> bool;
    fn to_tokens(&self, icon: &Self::Type) -> Option<(Vec<String>, i32)>;
    fn from_tokens(tokens: Vec<String>, version: i32) -> Result<Self::Type, glib::Error>;
    fn serialize(&self, icon: &Self::Type) -> Option<glib::Variant>;
}

unsafe impl<T: IconImpl> IsImplementable<T> for Icon
where
    <T as ObjectSubclass>::Type: IsA<glib::Object>,
{
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let icon_iface = &mut *(iface as *mut ffi::GIconIface);

        icon_iface.hash = Some(icon_hash::<T>);
        icon_iface.equal = Some(icon_equal::<T>);
        icon_iface.to_tokens = Some(icon_to_tokens::<T>);
        icon_iface.from_tokens = Some(icon_from_tokens::<T>);
        icon_iface.serialize = Some(icon_serialize::<T>);
    }
}

unsafe extern "C" fn icon_hash<T: IconImpl>(icon: *mut ffi::GIcon) -> u32 {
    let instance = &*(icon as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow::<_, Icon>(icon);

    imp.hash(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn icon_equal<T: IconImpl>(
    icon1: *mut ffi::GIcon,
    icon2: *mut ffi::GIcon,
) -> glib::ffi::gboolean {
    let instance = &*(icon1 as *mut T::Instance);
    let imp = instance.get_impl();

    imp.equal(
        from_glib_borrow::<_, Icon>(icon1).unsafe_cast_ref(),
        from_glib_borrow::<_, Icon>(icon2).unsafe_cast_ref(),
    )
    .to_glib()
}

unsafe extern "C" fn icon_to_tokens<T: IconImpl>(
    icon: *mut ffi::GIcon,
    tokensptr: *mut glib::ffi::GPtrArray,
    version: *mut libc::c_int,
) -> glib::ffi::gboolean {
    let instance = &*(icon as *mut T::Instance);
    let imp = instance.get_impl();

    let tokens_version = imp.to_tokens(from_glib_borrow::<_, Icon>(icon).unsafe_cast_ref());
    if let Some((tokens, tversion)) = tokens_version {
        *version = tversion;
        *tokensptr = tokens.to_glib_full();
        true.to_glib()
    } else {
        *tokensptr = *std::ptr::null_mut();
        false.to_glib()
    }
}

unsafe extern "C" fn icon_from_tokens<T: IconImpl>(
    tokensptr: *mut *mut libc::c_char,
    num_tokens: i32,
    version: i32,
    error: *mut *mut glib::ffi::GError,
) -> *mut ffi::GIcon {
    let tokens: Vec<String> = String::from_glib_none_num_as_vec(tokensptr, num_tokens as usize);

    let ret = T::from_tokens(tokens, version);
    match ret {
        Ok(icon) => icon.unsafe_cast_ref::<Icon>().to_glib_full(),
        Err(err) => {
            *error = err.to_glib_full() as *mut _;
            std::ptr::null_mut()
        }
    }
}

unsafe extern "C" fn icon_serialize<T: IconImpl>(
    icon: *mut ffi::GIcon,
) -> *mut glib::ffi::GVariant {
    let instance = &*(icon as *mut T::Instance);
    let imp = instance.get_impl();

    imp.serialize(from_glib_borrow::<_, Icon>(icon).unsafe_cast_ref())
        .to_glib_full()
}
