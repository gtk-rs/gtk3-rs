// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TextBuffer;
use crate::TextChildAnchor;
use crate::TextIter;
use crate::TextTag;
use glib::object::{Cast, IsA};
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use libc::{c_char, c_int};
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::{slice, str};

pub trait TextBufferExtManual: 'static {
    /// The ::apply-tag signal is emitted to apply a tag to a
    /// range of text in a [TextBuffer](crate::TextBuffer).
    /// Applying actually occurs in the default handler.
    ///
    /// Note that if your handler runs before the default handler it must not
    /// invalidate the `start` and `end` iters (or has to revalidate them).
    ///
    /// See also:
    /// [TextBufferExt::apply_tag](crate::prelude::TextBufferExt::apply_tag),
    /// `gtk_text_buffer_insert_with_tags`,
    /// [TextBufferExt::insert_range](crate::prelude::TextBufferExt::insert_range).
    /// ## `tag`
    /// the applied tag
    /// ## `start`
    /// the start of the range the tag is applied to
    /// ## `end`
    /// the end of the range the tag is applied to
    fn connect_apply_tag<F: Fn(&Self, &TextTag, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::delete-range signal is emitted to delete a range
    /// from a [TextBuffer](crate::TextBuffer).
    ///
    /// Note that if your handler runs before the default handler it must not
    /// invalidate the `start` and `end` iters (or has to revalidate them).
    /// The default signal handler revalidates the `start` and `end` iters to
    /// both point to the location where text was deleted. Handlers
    /// which run after the default handler (see `g_signal_connect_after`)
    /// do not have access to the deleted text.
    ///
    /// See also: [TextBufferExt::delete](crate::prelude::TextBufferExt::delete).
    /// ## `start`
    /// the start of the range to be deleted
    /// ## `end`
    /// the end of the range to be deleted
    fn connect_delete_range<F: Fn(&Self, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::insert-child-anchor signal is emitted to insert a
    /// [TextChildAnchor](crate::TextChildAnchor) in a [TextBuffer](crate::TextBuffer).
    /// Insertion actually occurs in the default handler.
    ///
    /// Note that if your handler runs before the default handler it must
    /// not invalidate the `location` iter (or has to revalidate it).
    /// The default signal handler revalidates it to be placed after the
    /// inserted `anchor`.
    ///
    /// See also: [TextBufferExt::insert_child_anchor](crate::prelude::TextBufferExt::insert_child_anchor).
    /// ## `location`
    /// position to insert `anchor` in `textbuffer`
    /// ## `anchor`
    /// the [TextChildAnchor](crate::TextChildAnchor) to be inserted
    fn connect_insert_child_anchor<F: Fn(&Self, &mut TextIter, &TextChildAnchor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::insert-pixbuf signal is emitted to insert a [gdk_pixbuf::Pixbuf](crate::gdk_pixbuf::Pixbuf)
    /// in a [TextBuffer](crate::TextBuffer). Insertion actually occurs in the default handler.
    ///
    /// Note that if your handler runs before the default handler it must not
    /// invalidate the `location` iter (or has to revalidate it).
    /// The default signal handler revalidates it to be placed after the
    /// inserted `pixbuf`.
    ///
    /// See also: [TextBufferExt::insert_pixbuf](crate::prelude::TextBufferExt::insert_pixbuf).
    /// ## `location`
    /// position to insert `pixbuf` in `textbuffer`
    /// ## `pixbuf`
    /// the [gdk_pixbuf::Pixbuf](crate::gdk_pixbuf::Pixbuf) to be inserted
    fn connect_insert_pixbuf<F: Fn(&Self, &mut TextIter, &gdk_pixbuf::Pixbuf) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::insert-text signal is emitted to insert text in a [TextBuffer](crate::TextBuffer).
    /// Insertion actually occurs in the default handler.
    ///
    /// Note that if your handler runs before the default handler it must not
    /// invalidate the `location` iter (or has to revalidate it).
    /// The default signal handler revalidates it to point to the end of the
    /// inserted text.
    ///
    /// See also:
    /// [TextBufferExt::insert](crate::prelude::TextBufferExt::insert),
    /// [TextBufferExt::insert_range](crate::prelude::TextBufferExt::insert_range).
    /// ## `location`
    /// position to insert `text` in `textbuffer`
    /// ## `text`
    /// the UTF-8 text to be inserted
    /// ## `len`
    /// length of the inserted text in bytes
    fn connect_insert_text<F: Fn(&Self, &mut TextIter, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::remove-tag signal is emitted to remove all occurrences of `tag` from
    /// a range of text in a [TextBuffer](crate::TextBuffer).
    /// Removal actually occurs in the default handler.
    ///
    /// Note that if your handler runs before the default handler it must not
    /// invalidate the `start` and `end` iters (or has to revalidate them).
    ///
    /// See also:
    /// [TextBufferExt::remove_tag](crate::prelude::TextBufferExt::remove_tag).
    /// ## `tag`
    /// the tag to be removed
    /// ## `start`
    /// the start of the range the tag is removed from
    /// ## `end`
    /// the end of the range the tag is removed from
    fn connect_remove_tag<F: Fn(&Self, &TextTag, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<TextBuffer>> TextBufferExtManual for O {
    fn connect_apply_tag<F: Fn(&Self, &TextTag, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn apply_tag_trampoline<
            P,
            F: Fn(&P, &TextTag, &mut TextIter, &mut TextIter) + 'static,
        >(
            this: *mut ffi::GtkTextBuffer,
            tag: *mut ffi::GtkTextTag,
            start: *mut ffi::GtkTextIter,
            end: *mut ffi::GtkTextIter,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut start_copy = from_glib_none(start);
            let mut end_copy = from_glib_none(end);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(tag),
                &mut start_copy,
                &mut end_copy,
            );

            *start = *start_copy.to_glib_none().0;
            *end = *end_copy.to_glib_none().0;
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"apply-tag\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    apply_tag_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_delete_range<F: Fn(&Self, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn delete_range_trampoline<
            P,
            F: Fn(&P, &mut TextIter, &mut TextIter) + 'static,
        >(
            this: *mut ffi::GtkTextBuffer,
            start: *mut ffi::GtkTextIter,
            end: *mut ffi::GtkTextIter,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut start_copy = from_glib_none(start);
            let mut end_copy = from_glib_none(end);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &mut start_copy,
                &mut end_copy,
            );

            *start = *start_copy.to_glib_none().0;
            *end = *end_copy.to_glib_none().0;
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"delete-range\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    delete_range_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_insert_child_anchor<F: Fn(&Self, &mut TextIter, &TextChildAnchor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn insert_child_anchor_trampoline<
            P,
            F: Fn(&P, &mut TextIter, &TextChildAnchor) + 'static,
        >(
            this: *mut ffi::GtkTextBuffer,
            location: *mut ffi::GtkTextIter,
            anchor: *mut ffi::GtkTextChildAnchor,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut location_copy = from_glib_none(location);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &mut location_copy,
                &from_glib_borrow(anchor),
            );

            *location = *location_copy.to_glib_none().0;
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"insert-child-anchor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    insert_child_anchor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_insert_pixbuf<F: Fn(&Self, &mut TextIter, &gdk_pixbuf::Pixbuf) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn insert_pixbuf_trampoline<
            P,
            F: Fn(&P, &mut TextIter, &gdk_pixbuf::Pixbuf) + 'static,
        >(
            this: *mut ffi::GtkTextBuffer,
            location: *mut ffi::GtkTextIter,
            pixbuf: *mut gdk_pixbuf::ffi::GdkPixbuf,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut location_copy = from_glib_none(location);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &mut location_copy,
                &from_glib_borrow(pixbuf),
            );

            *location = *location_copy.to_glib_none().0;
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"insert-pixbuf\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    insert_pixbuf_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_insert_text<F: Fn(&Self, &mut TextIter, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn insert_text_trampoline<T, F: Fn(&T, &mut TextIter, &str) + 'static>(
            this: *mut ffi::GtkTextBuffer,
            location: *mut ffi::GtkTextIter,
            text: *mut c_char,
            len: c_int,
            f: glib::ffi::gpointer,
        ) where
            T: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut location_copy = from_glib_none(location);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &mut location_copy,
                str::from_utf8(slice::from_raw_parts(text as *const u8, len as usize)).unwrap(),
            );

            *location = *location_copy.to_glib_none().0;
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"insert-text\0".as_ptr() as *mut _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    insert_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_remove_tag<F: Fn(&Self, &TextTag, &mut TextIter, &mut TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn remove_tag_trampoline<
            P,
            F: Fn(&P, &TextTag, &mut TextIter, &mut TextIter) + 'static,
        >(
            this: *mut ffi::GtkTextBuffer,
            tag: *mut ffi::GtkTextTag,
            start: *mut ffi::GtkTextIter,
            end: *mut ffi::GtkTextIter,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            let mut start_copy = from_glib_none(start);
            let mut end_copy = from_glib_none(end);

            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(tag),
                &mut start_copy,
                &mut end_copy,
            );

            *start = *start_copy.to_glib_none().0;
            *end = *end_copy.to_glib_none().0;
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"remove-tag\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    remove_tag_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
