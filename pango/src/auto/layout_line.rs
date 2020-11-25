// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Rectangle;
use glib::translate::*;
use std::mem;
use std::ptr;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LayoutLine(Shared<ffi::PangoLayoutLine>);

    match fn {
        ref => |ptr| ffi::pango_layout_line_ref(ptr),
        unref => |ptr| ffi::pango_layout_line_unref(ptr),
        get_type => || ffi::pango_layout_line_get_type(),
    }
}

impl LayoutLine {
    pub fn get_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_line_get_extents(
                self.to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[cfg(any(feature = "v1_44", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_44")))]
    pub fn get_height(&self) -> i32 {
        unsafe {
            let mut height = mem::MaybeUninit::uninit();
            ffi::pango_layout_line_get_height(self.to_glib_none().0, height.as_mut_ptr());
            let height = height.assume_init();
            height
        }
    }

    pub fn get_pixel_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_line_get_pixel_extents(
                self.to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    pub fn get_x_ranges(&self, start_index: i32, end_index: i32) -> Vec<i32> {
        unsafe {
            let mut ranges = ptr::null_mut();
            let mut n_ranges = mem::MaybeUninit::uninit();
            ffi::pango_layout_line_get_x_ranges(
                self.to_glib_none().0,
                start_index,
                end_index,
                &mut ranges,
                n_ranges.as_mut_ptr(),
            );
            FromGlibContainer::from_glib_full_num(ranges, n_ranges.assume_init() as usize)
        }
    }

    pub fn index_to_x(&self, index_: i32, trailing: bool) -> i32 {
        unsafe {
            let mut x_pos = mem::MaybeUninit::uninit();
            ffi::pango_layout_line_index_to_x(
                self.to_glib_none().0,
                index_,
                trailing.to_glib(),
                x_pos.as_mut_ptr(),
            );
            let x_pos = x_pos.assume_init();
            x_pos
        }
    }

    pub fn x_to_index(&self, x_pos: i32) -> Option<(i32, i32)> {
        unsafe {
            let mut index_ = mem::MaybeUninit::uninit();
            let mut trailing = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::pango_layout_line_x_to_index(
                self.to_glib_none().0,
                x_pos,
                index_.as_mut_ptr(),
                trailing.as_mut_ptr(),
            ));
            let index_ = index_.assume_init();
            let trailing = trailing.assume_init();
            if ret {
                Some((index_, trailing))
            } else {
                None
            }
        }
    }
}
