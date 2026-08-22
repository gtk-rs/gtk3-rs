// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Editable`] interface.

use std::ffi::CStr;

use glib::translate::*;

use crate::{Editable, ffi, prelude::*, subclass::prelude::*};

pub trait EditableImpl: ObjectImpl + ObjectSubclass<Type: IsA<Editable>> {
    fn insert_text(&self, new_text: &str, position: &mut i32) {
        self.parent_insert_text(new_text, position);
    }

    fn delete_text(&self, start_pos: i32, end_pos: i32) {
        self.parent_delete_text(start_pos, end_pos);
    }

    fn changed(&self) {
        self.parent_changed();
    }

    fn do_insert_text(&self, new_text: &str, position: &mut i32) {
        self.parent_do_insert_text(new_text, position);
    }

    fn do_delete_text(&self, start_pos: i32, end_pos: i32) {
        self.parent_do_delete_text(start_pos, end_pos);
    }

    #[doc(alias = "get_chars")]
    fn chars(&self, start_pos: i32, end_pos: i32) -> Option<glib::GString> {
        self.parent_chars(start_pos, end_pos)
    }

    fn set_selection_bounds(&self, start_pos: i32, end_pos: i32) {
        self.parent_set_selection_bounds(start_pos, end_pos);
    }

    #[doc(alias = "get_selection_bounds")]
    fn selection_bounds(&self) -> Option<(i32, i32)> {
        self.parent_selection_bounds()
    }

    fn set_position(&self, position: i32) {
        self.parent_set_position(position);
    }

    #[doc(alias = "get_position")]
    fn position(&self) -> i32 {
        self.parent_position()
    }
}

pub trait EditableImplExt: EditableImpl {
    fn parent_insert_text(&self, new_text: &str, position: &mut i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).insert_text {
                func(
                    self.obj().unsafe_cast_ref::<Editable>().to_glib_none().0,
                    new_text.as_ptr() as *const _,
                    new_text.len() as i32,
                    position,
                );
            }
        }
    }

    fn parent_delete_text(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).delete_text {
                func(
                    self.obj().unsafe_cast_ref::<Editable>().to_glib_none().0,
                    start_pos,
                    end_pos,
                );
            }
        }
    }

    fn parent_changed(&self) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).changed {
                func(self.obj().unsafe_cast_ref::<Editable>().to_glib_none().0);
            }
        }
    }

    fn parent_do_insert_text(&self, new_text: &str, position: &mut i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            let func = (*parent_iface)
                .do_insert_text
                .expect("no parent \"do_insert_text\" implementation");
            func(
                self.obj().unsafe_cast_ref::<Editable>().to_glib_none().0,
                new_text.as_ptr() as *const _,
                new_text.len() as i32,
                position,
            );
        }
    }

    fn parent_do_delete_text(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            let func = (*parent_iface)
                .do_delete_text
                .expect("no parent \"do_delete_text\" implementation");
            func(
                self.obj().unsafe_cast_ref::<Editable>().to_glib_none().0,
                start_pos,
                end_pos,
            );
        }
    }

    fn parent_chars(&self, start_pos: i32, end_pos: i32) -> Option<glib::GString> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            let func = (*parent_iface)
                .get_chars
                .expect("no parent \"get_chars\" implementation");
            from_glib_full(func(
                self.obj().unsafe_cast_ref::<Editable>().to_glib_none().0,
                start_pos,
                end_pos,
            ))
        }
    }

    fn parent_set_selection_bounds(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            let func = (*parent_iface)
                .set_selection_bounds
                .expect("no parent \"set_selection_bounds\" implementation");
            func(
                self.obj().unsafe_cast_ref::<Editable>().to_glib_none().0,
                start_pos,
                end_pos,
            );
        }
    }

    fn parent_selection_bounds(&self) -> Option<(i32, i32)> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            let func = (*parent_iface)
                .get_selection_bounds
                .expect("no parent \"get_selection_bounds\" implementation");
            let mut start_pos = 0;
            let mut end_pos = 0;
            from_glib::<_, bool>(func(
                self.obj().unsafe_cast_ref::<Editable>().to_glib_none().0,
                &mut start_pos,
                &mut end_pos,
            ))
            .then_some((start_pos, end_pos))
        }
    }

    fn parent_set_position(&self, position: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            let func = (*parent_iface)
                .set_position
                .expect("no parent \"set_position\" implementation");
            func(
                self.obj().unsafe_cast_ref::<Editable>().to_glib_none().0,
                position,
            );
        }
    }

    fn parent_position(&self) -> i32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            let func = (*parent_iface)
                .get_position
                .expect("no parent \"get_position\" implementation");
            func(self.obj().unsafe_cast_ref::<Editable>().to_glib_none().0)
        }
    }
}

impl<T: EditableImpl> EditableImplExt for T {}

unsafe impl<T: EditableImpl> IsImplementable<T> for Editable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        iface.insert_text = Some(editable_insert_text::<T>);
        iface.delete_text = Some(editable_delete_text::<T>);
        iface.changed = Some(editable_changed::<T>);
        iface.do_insert_text = Some(editable_do_insert_text::<T>);
        iface.do_delete_text = Some(editable_do_delete_text::<T>);
        iface.get_chars = Some(editable_get_chars::<T>);
        iface.set_selection_bounds = Some(editable_set_selection_bounds::<T>);
        iface.get_selection_bounds = Some(editable_get_selection_bounds::<T>);
        iface.set_position = Some(editable_set_position::<T>);
        iface.get_position = Some(editable_get_position::<T>);
    }
}

unsafe extern "C" fn editable_insert_text<T: EditableImpl>(
    editableptr: *mut ffi::GtkEditable,
    new_text_ptr: *const glib::ffi::gchar,
    new_text_length: i32,
    position_ptr: *mut i32,
) {
    assert!(!editableptr.is_null());
    assert!(!new_text_ptr.is_null());
    assert!(!position_ptr.is_null());

    let instance = unsafe { &*(editableptr as *mut T::Instance) };
    let imp = instance.imp();

    let new_text = if new_text_length < 0 {
        unsafe { CStr::from_ptr(new_text_ptr).to_str().unwrap() }
    } else {
        let s = unsafe {
            std::slice::from_raw_parts(new_text_ptr as *const _, new_text_length as usize)
        };
        str::from_utf8(s).unwrap()
    };

    let position = unsafe { &mut *position_ptr };
    imp.insert_text(new_text, position);
}

unsafe extern "C" fn editable_delete_text<T: EditableImpl>(
    editableptr: *mut ffi::GtkEditable,
    start_pos: i32,
    end_pos: i32,
) {
    assert!(!editableptr.is_null());

    let instance = unsafe { &*(editableptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.delete_text(start_pos, end_pos);
}

unsafe extern "C" fn editable_changed<T: EditableImpl>(editableptr: *mut ffi::GtkEditable) {
    assert!(!editableptr.is_null());

    let instance = unsafe { &*(editableptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.changed();
}

unsafe extern "C" fn editable_do_insert_text<T: EditableImpl>(
    editableptr: *mut ffi::GtkEditable,
    new_text_ptr: *const glib::ffi::gchar,
    new_text_length: i32,
    position_ptr: *mut i32,
) {
    assert!(!editableptr.is_null());
    assert!(!new_text_ptr.is_null());
    assert!(!position_ptr.is_null());

    let instance = unsafe { &*(editableptr as *mut T::Instance) };
    let imp = instance.imp();

    let new_text = if new_text_length < 0 {
        unsafe { CStr::from_ptr(new_text_ptr).to_str().unwrap() }
    } else {
        let s = unsafe {
            std::slice::from_raw_parts(new_text_ptr as *const _, new_text_length as usize)
        };
        str::from_utf8(s).unwrap()
    };

    let position = unsafe { &mut *position_ptr };
    imp.do_insert_text(new_text, position);
}

unsafe extern "C" fn editable_do_delete_text<T: EditableImpl>(
    editableptr: *mut ffi::GtkEditable,
    start_pos: i32,
    end_pos: i32,
) {
    assert!(!editableptr.is_null());

    let instance = unsafe { &*(editableptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.do_delete_text(start_pos, end_pos);
}

unsafe extern "C" fn editable_get_chars<T: EditableImpl>(
    editableptr: *mut ffi::GtkEditable,
    start_pos: i32,
    end_pos: i32,
) -> *mut glib::ffi::gchar {
    assert!(!editableptr.is_null());

    let instance = unsafe { &*(editableptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.chars(start_pos, end_pos)
        .map(|chars| chars.into_glib_ptr())
        .unwrap_or(std::ptr::null_mut())
}

unsafe extern "C" fn editable_set_selection_bounds<T: EditableImpl>(
    editableptr: *mut ffi::GtkEditable,
    start_pos: i32,
    end_pos: i32,
) {
    assert!(!editableptr.is_null());

    let instance = unsafe { &*(editableptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.set_selection_bounds(start_pos, end_pos);
}

unsafe extern "C" fn editable_get_selection_bounds<T: EditableImpl>(
    editableptr: *mut ffi::GtkEditable,
    start_pos_ptr: *mut i32,
    end_pos_ptr: *mut i32,
) -> glib::ffi::gboolean {
    assert!(!editableptr.is_null());

    let instance = unsafe { &*(editableptr as *mut T::Instance) };
    let imp = instance.imp();

    if let Some((start_pos, end_pos)) = imp.selection_bounds() {
        if !start_pos_ptr.is_null() {
            unsafe {
                *start_pos_ptr = start_pos;
            }
        }
        if !end_pos_ptr.is_null() {
            unsafe {
                *end_pos_ptr = end_pos;
            }
        }
        glib::ffi::GTRUE
    } else {
        glib::ffi::GFALSE
    }
}

unsafe extern "C" fn editable_set_position<T: EditableImpl>(
    editableptr: *mut ffi::GtkEditable,
    position: i32,
) {
    assert!(!editableptr.is_null());

    let instance = unsafe { &*(editableptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.set_position(position);
}

unsafe extern "C" fn editable_get_position<T: EditableImpl>(
    editableptr: *mut ffi::GtkEditable,
) -> i32 {
    assert!(!editableptr.is_null());

    let instance = unsafe { &*(editableptr as *mut T::Instance) };
    let imp = instance.imp();
    imp.position()
}
