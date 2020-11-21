// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::CoordType;
#[cfg(any(feature = "v2_32", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
use crate::ScrollType;
use crate::TextBoundary;
use crate::TextClipType;
use crate::TextGranularity;
use crate::TextRange;
use crate::TextRectangle;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct Text(Interface<ffi::AtkText>);

    match fn {
        get_type => || ffi::atk_text_get_type(),
    }
}

pub const NONE_TEXT: Option<&Text> = None;

pub trait TextExt: 'static {
    fn add_selection(&self, start_offset: i32, end_offset: i32) -> bool;

    fn get_bounded_ranges(
        &self,
        rect: &mut TextRectangle,
        coord_type: CoordType,
        x_clip_type: TextClipType,
        y_clip_type: TextClipType,
    ) -> Vec<TextRange>;

    fn get_caret_offset(&self) -> i32;

    fn get_character_at_offset(&self, offset: i32) -> char;

    fn get_character_count(&self) -> i32;

    fn get_character_extents(&self, offset: i32, coords: CoordType) -> (i32, i32, i32, i32);

    //fn get_default_attributes(&self) -> /*Ignored*/Option<AttributeSet>;

    fn get_n_selections(&self) -> i32;

    fn get_offset_at_point(&self, x: i32, y: i32, coords: CoordType) -> i32;

    fn get_range_extents(
        &self,
        start_offset: i32,
        end_offset: i32,
        coord_type: CoordType,
    ) -> TextRectangle;

    //fn get_run_attributes(&self, offset: i32) -> (/*Ignored*/AttributeSet, i32, i32);

    fn get_selection(&self, selection_num: i32) -> (glib::GString, i32, i32);

    fn get_string_at_offset(
        &self,
        offset: i32,
        granularity: TextGranularity,
    ) -> (Option<glib::GString>, i32, i32);

    fn get_text(&self, start_offset: i32, end_offset: i32) -> Option<glib::GString>;

    fn get_text_at_offset(
        &self,
        offset: i32,
        boundary_type: TextBoundary,
    ) -> (glib::GString, i32, i32);

    fn remove_selection(&self, selection_num: i32) -> bool;

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    fn scroll_substring_to(&self, start_offset: i32, end_offset: i32, type_: ScrollType) -> bool;

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    fn scroll_substring_to_point(
        &self,
        start_offset: i32,
        end_offset: i32,
        coords: CoordType,
        x: i32,
        y: i32,
    ) -> bool;

    fn set_caret_offset(&self, offset: i32) -> bool;

    fn set_selection(&self, selection_num: i32, start_offset: i32, end_offset: i32) -> bool;

    fn connect_text_attributes_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_text_caret_moved<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_text_insert<F: Fn(&Self, i32, i32, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_text_remove<F: Fn(&Self, i32, i32, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_text_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Text>> TextExt for O {
    fn add_selection(&self, start_offset: i32, end_offset: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_text_add_selection(
                self.as_ref().to_glib_none().0,
                start_offset,
                end_offset,
            ))
        }
    }

    fn get_bounded_ranges(
        &self,
        rect: &mut TextRectangle,
        coord_type: CoordType,
        x_clip_type: TextClipType,
        y_clip_type: TextClipType,
    ) -> Vec<TextRange> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::atk_text_get_bounded_ranges(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none_mut().0,
                coord_type.to_glib(),
                x_clip_type.to_glib(),
                y_clip_type.to_glib(),
            ))
        }
    }

    fn get_caret_offset(&self) -> i32 {
        unsafe { ffi::atk_text_get_caret_offset(self.as_ref().to_glib_none().0) }
    }

    fn get_character_at_offset(&self, offset: i32) -> char {
        unsafe {
            std::convert::TryFrom::try_from(ffi::atk_text_get_character_at_offset(
                self.as_ref().to_glib_none().0,
                offset,
            ))
            .expect("conversion from an invalid Unicode value attempted")
        }
    }

    fn get_character_count(&self) -> i32 {
        unsafe { ffi::atk_text_get_character_count(self.as_ref().to_glib_none().0) }
    }

    fn get_character_extents(&self, offset: i32, coords: CoordType) -> (i32, i32, i32, i32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::atk_text_get_character_extents(
                self.as_ref().to_glib_none().0,
                offset,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
                coords.to_glib(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            let width = width.assume_init();
            let height = height.assume_init();
            (x, y, width, height)
        }
    }

    //fn get_default_attributes(&self) -> /*Ignored*/Option<AttributeSet> {
    //    unsafe { TODO: call ffi:atk_text_get_default_attributes() }
    //}

    fn get_n_selections(&self) -> i32 {
        unsafe { ffi::atk_text_get_n_selections(self.as_ref().to_glib_none().0) }
    }

    fn get_offset_at_point(&self, x: i32, y: i32, coords: CoordType) -> i32 {
        unsafe {
            ffi::atk_text_get_offset_at_point(
                self.as_ref().to_glib_none().0,
                x,
                y,
                coords.to_glib(),
            )
        }
    }

    fn get_range_extents(
        &self,
        start_offset: i32,
        end_offset: i32,
        coord_type: CoordType,
    ) -> TextRectangle {
        unsafe {
            let mut rect = TextRectangle::uninitialized();
            ffi::atk_text_get_range_extents(
                self.as_ref().to_glib_none().0,
                start_offset,
                end_offset,
                coord_type.to_glib(),
                rect.to_glib_none_mut().0,
            );
            rect
        }
    }

    //fn get_run_attributes(&self, offset: i32) -> (/*Ignored*/AttributeSet, i32, i32) {
    //    unsafe { TODO: call ffi:atk_text_get_run_attributes() }
    //}

    fn get_selection(&self, selection_num: i32) -> (glib::GString, i32, i32) {
        unsafe {
            let mut start_offset = mem::MaybeUninit::uninit();
            let mut end_offset = mem::MaybeUninit::uninit();
            let ret = from_glib_full(ffi::atk_text_get_selection(
                self.as_ref().to_glib_none().0,
                selection_num,
                start_offset.as_mut_ptr(),
                end_offset.as_mut_ptr(),
            ));
            let start_offset = start_offset.assume_init();
            let end_offset = end_offset.assume_init();
            (ret, start_offset, end_offset)
        }
    }

    fn get_string_at_offset(
        &self,
        offset: i32,
        granularity: TextGranularity,
    ) -> (Option<glib::GString>, i32, i32) {
        unsafe {
            let mut start_offset = mem::MaybeUninit::uninit();
            let mut end_offset = mem::MaybeUninit::uninit();
            let ret = from_glib_full(ffi::atk_text_get_string_at_offset(
                self.as_ref().to_glib_none().0,
                offset,
                granularity.to_glib(),
                start_offset.as_mut_ptr(),
                end_offset.as_mut_ptr(),
            ));
            let start_offset = start_offset.assume_init();
            let end_offset = end_offset.assume_init();
            (ret, start_offset, end_offset)
        }
    }

    fn get_text(&self, start_offset: i32, end_offset: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::atk_text_get_text(
                self.as_ref().to_glib_none().0,
                start_offset,
                end_offset,
            ))
        }
    }

    fn get_text_at_offset(
        &self,
        offset: i32,
        boundary_type: TextBoundary,
    ) -> (glib::GString, i32, i32) {
        unsafe {
            let mut start_offset = mem::MaybeUninit::uninit();
            let mut end_offset = mem::MaybeUninit::uninit();
            let ret = from_glib_full(ffi::atk_text_get_text_at_offset(
                self.as_ref().to_glib_none().0,
                offset,
                boundary_type.to_glib(),
                start_offset.as_mut_ptr(),
                end_offset.as_mut_ptr(),
            ));
            let start_offset = start_offset.assume_init();
            let end_offset = end_offset.assume_init();
            (ret, start_offset, end_offset)
        }
    }

    fn remove_selection(&self, selection_num: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_text_remove_selection(
                self.as_ref().to_glib_none().0,
                selection_num,
            ))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    fn scroll_substring_to(&self, start_offset: i32, end_offset: i32, type_: ScrollType) -> bool {
        unsafe {
            from_glib(ffi::atk_text_scroll_substring_to(
                self.as_ref().to_glib_none().0,
                start_offset,
                end_offset,
                type_.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    fn scroll_substring_to_point(
        &self,
        start_offset: i32,
        end_offset: i32,
        coords: CoordType,
        x: i32,
        y: i32,
    ) -> bool {
        unsafe {
            from_glib(ffi::atk_text_scroll_substring_to_point(
                self.as_ref().to_glib_none().0,
                start_offset,
                end_offset,
                coords.to_glib(),
                x,
                y,
            ))
        }
    }

    fn set_caret_offset(&self, offset: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_text_set_caret_offset(
                self.as_ref().to_glib_none().0,
                offset,
            ))
        }
    }

    fn set_selection(&self, selection_num: i32, start_offset: i32, end_offset: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_text_set_selection(
                self.as_ref().to_glib_none().0,
                selection_num,
                start_offset,
                end_offset,
            ))
        }
    }

    fn connect_text_attributes_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn text_attributes_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkText,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Text>,
        {
            let f: &F = &*(f as *const F);
            f(&Text::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"text-attributes-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    text_attributes_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_text_caret_moved<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn text_caret_moved_trampoline<P, F: Fn(&P, i32) + 'static>(
            this: *mut ffi::AtkText,
            arg1: libc::c_int,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Text>,
        {
            let f: &F = &*(f as *const F);
            f(&Text::from_glib_borrow(this).unsafe_cast_ref(), arg1)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"text-caret-moved\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    text_caret_moved_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_text_insert<F: Fn(&Self, i32, i32, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn text_insert_trampoline<P, F: Fn(&P, i32, i32, &str) + 'static>(
            this: *mut ffi::AtkText,
            arg1: libc::c_int,
            arg2: libc::c_int,
            arg3: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Text>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Text::from_glib_borrow(this).unsafe_cast_ref(),
                arg1,
                arg2,
                &glib::GString::from_glib_borrow(arg3),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"text-insert\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    text_insert_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_text_remove<F: Fn(&Self, i32, i32, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn text_remove_trampoline<P, F: Fn(&P, i32, i32, &str) + 'static>(
            this: *mut ffi::AtkText,
            arg1: libc::c_int,
            arg2: libc::c_int,
            arg3: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Text>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Text::from_glib_borrow(this).unsafe_cast_ref(),
                arg1,
                arg2,
                &glib::GString::from_glib_borrow(arg3),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"text-remove\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    text_remove_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_text_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn text_selection_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkText,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Text>,
        {
            let f: &F = &*(f as *const F);
            f(&Text::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"text-selection-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    text_selection_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Text")
    }
}
