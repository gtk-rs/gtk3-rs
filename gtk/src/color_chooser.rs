// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ColorChooser;
use crate::Orientation;
use gdk::RGBA;
use glib::object::IsA;
use glib::translate::*;
use libc::c_int;

pub trait ColorChooserExtManual: 'static {
    /// Adds a palette to the color chooser. If `orientation` is horizontal,
    /// the colors are grouped in rows, with `colors_per_line` colors
    /// in each row. If `horizontal` is [`false`], the colors are grouped
    /// in columns instead.
    ///
    /// The default color palette of [ColorChooserWidget](crate::ColorChooserWidget) has
    /// 27 colors, organized in columns of 3 colors. The default gray
    /// palette has 9 grays in a single row.
    ///
    /// The layout of the color chooser widget works best when the
    /// palettes have 9-10 columns.
    ///
    /// Calling this function for the first time has the
    /// side effect of removing the default color and gray palettes
    /// from the color chooser.
    ///
    /// If `colors` is [`None`], removes all previously added palettes.
    /// ## `orientation`
    /// [Orientation::Horizontal](crate::Orientation::Horizontal) if the palette should
    ///  be displayed in rows, [Orientation::Vertical](crate::Orientation::Vertical) for columns
    /// ## `colors_per_line`
    /// the number of colors to show in each row/column
    /// ## `colors`
    /// the colors of the palette, or [`None`]
    #[doc(alias = "gtk_color_chooser_add_palette")]
    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]);
}

impl<O: IsA<ColorChooser>> ColorChooserExtManual for O {
    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]) {
        unsafe {
            ffi::gtk_color_chooser_add_palette(
                self.as_ref().to_glib_none().0,
                orientation.into_glib(),
                colors_per_line,
                colors.len() as c_int,
                colors.as_ptr() as *mut gdk::ffi::GdkRGBA,
            )
        }
    }
}
