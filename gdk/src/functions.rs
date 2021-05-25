// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[repr(packed)]
pub struct GRange(pub i32, pub i32);

/// Obtains a clip region which contains the areas where the given
/// ranges of text would be drawn. `x_origin` and `y_origin` are the top left
/// position of the layout. `index_ranges`
/// should contain ranges of bytes in the layout’s text. The clip
/// region will include space to the left or right of the line (to the
/// layout bounding box) if you have indexes above or below the indexes
/// contained inside the line. This is to draw the selection all the way
/// to the side of the layout. However, the clip region is in line coordinates,
/// not layout coordinates.
///
/// Note that the regions returned correspond to logical extents of the text
/// ranges, not ink extents. So the drawn line may in fact touch areas out of
/// the clip region. The clip region is mainly useful for highlightling parts
/// of text, such as when text is selected.
/// ## `line`
/// a [pango::LayoutLine](crate::pango::LayoutLine)
/// ## `x_origin`
/// X pixel where you intend to draw the layout line with this clip
/// ## `y_origin`
/// baseline pixel where you intend to draw the layout line with this clip
/// ## `index_ranges`
/// array of byte indexes into the layout,
///  where even members of array are start indexes and odd elements
///  are end indexes
/// ## `n_ranges`
/// number of ranges in `index_ranges`, i.e. half the size of `index_ranges`
///
/// # Returns
///
/// a clip region containing the given ranges
#[doc(alias = "gdk_pango_layout_line_get_clip_region")]
pub fn pango_layout_line_get_clip_region(
    line: &pango::LayoutLine,
    x_origin: i32,
    y_origin: i32,
    index_ranges: &[GRange],
) -> Option<cairo::Region> {
    assert_initialized_main_thread!();

    let ptr: *const i32 = index_ranges.as_ptr() as _;
    unsafe {
        from_glib_full(ffi::gdk_pango_layout_line_get_clip_region(
            line.to_glib_none().0,
            x_origin,
            y_origin,
            mut_override(ptr),
            index_ranges.len() as i32,
        ))
    }
}

/// Obtains a clip region which contains the areas where the given ranges
/// of text would be drawn. `x_origin` and `y_origin` are the top left point
/// to center the layout. `index_ranges` should contain
/// ranges of bytes in the layout’s text.
///
/// Note that the regions returned correspond to logical extents of the text
/// ranges, not ink extents. So the drawn layout may in fact touch areas out of
/// the clip region. The clip region is mainly useful for highlightling parts
/// of text, such as when text is selected.
/// ## `layout`
/// a [pango::Layout](crate::pango::Layout)
/// ## `x_origin`
/// X pixel where you intend to draw the layout with this clip
/// ## `y_origin`
/// Y pixel where you intend to draw the layout with this clip
/// ## `index_ranges`
/// array of byte indexes into the layout, where even members of array are start indexes and odd elements are end indexes
/// ## `n_ranges`
/// number of ranges in `index_ranges`, i.e. half the size of `index_ranges`
///
/// # Returns
///
/// a clip region containing the given ranges
#[doc(alias = "gdk_pango_layout_get_clip_region")]
pub fn pango_layout_get_clip_region(
    layout: &pango::Layout,
    x_origin: i32,
    y_origin: i32,
    index_ranges: &[GRange],
) -> Option<cairo::Region> {
    assert_initialized_main_thread!();

    let ptr: *const i32 = index_ranges.as_ptr() as _;
    unsafe {
        from_glib_full(ffi::gdk_pango_layout_get_clip_region(
            layout.to_glib_none().0,
            x_origin,
            y_origin,
            ptr,
            index_ranges.len() as i32,
        ))
    }
}

/// Obtains a desktop-wide setting, such as the double-click time,
/// for the default screen. See [Screen::is_setting](crate::Screen::is_setting).
/// ## `name`
/// the name of the setting.
/// ## `value`
/// location to store the value of the setting.
///
/// # Returns
///
/// [`true`] if the setting existed and a value was stored
///  in `value`, [`false`] otherwise.
#[doc(alias = "gdk_setting_get")]
pub fn setting_get(name: &str) -> Option<glib::Value> {
    assert_initialized_main_thread!();
    unsafe {
        let mut value = glib::Value::uninitialized();
        let done: bool = from_glib(ffi::gdk_setting_get(
            name.to_glib_none().0,
            value.to_glib_none_mut().0,
        ));
        if done {
            Some(value)
        } else {
            None
        }
    }
}

/// Changes the contents of a property on a window.
/// ## `window`
/// a [Window](crate::Window)
/// ## `property`
/// the property to change
/// ## `type_`
/// the new type for the property. If `mode` is
///  [PropMode::Prepend](crate::PropMode::Prepend) or [PropMode::Append](crate::PropMode::Append), then this
///  must match the existing type or an error will occur.
/// ## `format`
/// the new format for the property. If `mode` is
///  [PropMode::Prepend](crate::PropMode::Prepend) or [PropMode::Append](crate::PropMode::Append), then this
///  must match the existing format or an error will occur.
/// ## `mode`
/// a value describing how the new data is to be combined
///  with the current data.
/// ## `data`
/// the data (a `guchar *`
///  `gushort *`, or `gulong *`,
///  depending on `format`), cast to a `guchar *`.
/// ## `nelements`
/// the number of elements of size determined by the format,
///  contained in `data`.
#[doc(alias = "gdk_property_change")]
pub fn property_change(
    window: &super::Window,
    property: &super::Atom,
    type_: &super::Atom,
    format: i32,
    mode: super::PropMode,
    data: super::ChangeData,
) {
    skip_assert_initialized!();
    let nelements = data.len();
    unsafe {
        ffi::gdk_property_change(
            window.to_glib_none().0,
            property.to_glib_none().0,
            type_.to_glib_none().0,
            format,
            mode.into_glib(),
            data.to_glib(),
            nelements as i32,
        );
    }
}
