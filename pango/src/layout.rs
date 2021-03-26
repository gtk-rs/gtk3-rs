// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::LayoutLine;

// rustdoc-stripper-ignore-next
/// The result of [`LayoutLine::x_to_index`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HitPosition {
    // rustdoc-stripper-ignore-next
    /// The UTF-8 byte offset of the grapheme closest to the position.
    ///
    /// This position is relative to the start of the [`Layout`]'s text.
    ///
    /// [`Layout`]: crate::Layout
    pub index: i32,
    // rustdoc-stripper-ignore-next
    /// The codepoint within the grapheme of the position.
    ///
    /// This will always be either `0`, or the number of `char`s (*not bytes!*)
    /// in the grapheme. This represents whether the user clicked near the start
    /// of the grapheme or near the end; this is important for things like
    /// resolving cursor positions.
    pub trailing: i32,
    // rustdoc-stripper-ignore-next
    /// Whether or not the position was within the bounds of the line.
    ///
    /// If this is `false`, then `index` and `trailing` will always resolve
    /// to either the very first or the very last position in the line; this
    /// behaviour is dependent on the line's resolved writing direction.
    pub is_inside: bool,
}

impl LayoutLine {
    // rustdoc-stripper-ignore-next
    /// The byte index of the start of this line into the text used to create
    /// the source [`Layout`].
    ///
    /// [`Layout`]: crate::Layout
    pub fn start_index(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).start_index }
    }

    // rustdoc-stripper-ignore-next
    /// The length of this line's text, in bytes.
    pub fn length(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).length }
    }

    #[doc(alias = "pango_layout_line_x_to_index")]
    pub fn x_to_index(&self, x_pos: i32) -> HitPosition {
        let mut index = 0;
        let mut trailing = 0;

        let is_inside = unsafe {
            from_glib(ffi::pango_layout_line_x_to_index(
                self.to_glib_none().0,
                x_pos,
                &mut index,
                &mut trailing,
            ))
        };

        HitPosition {
            index,
            trailing,
            is_inside,
        }
    }
}
