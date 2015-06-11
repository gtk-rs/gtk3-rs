// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! The cairo drawing context

use glib::translate::ToGlibPtr;
use c_vec::CVec;
use std::mem::transmute;
use libc::{c_double, c_int};
use ::paths::Path;
use ::fonts::{TextExtents, TextCluster, FontExtents, ScaledFont, FontOptions, FontFace, Glyph};
use ::matrices::{Matrix, MatrixTrait};
use ffi::enums::{
    FontSlant,
    FontWeight,
    TextClusterFlags
};
use Rectangle;
use ffi;

use ffi::{
    cairo_t,
    cairo_surface_t,
    cairo_rectangle_list_t,
};
use ffi::enums::{Status, Antialias, LineCap, LineJoin, FillRule};
use ::patterns::{wrap_pattern, Pattern};

pub struct RectangleVec {
    ptr: *mut cairo_rectangle_list_t,
    pub rectangles: CVec<Rectangle>,
}

impl Drop for RectangleVec {
    /// Unconditionally frees rectangle_list and all associated references. After this call,
    /// the rectangle_list pointer must not be dereferenced.
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_rectangle_list_destroy(self.ptr);
        }
    }
}

#[repr(C)]
pub struct Context(*mut cairo_t);

// Temporarily fix issue #210
// For more info, see discussion at https://github.com/jeremyletang/rgtk/issues/210
// impl Drop for Context {
//     fn drop(&mut self) {
//         unsafe {
//             ffi::cairo_destroy(self.get_ptr())
//         }
//     }
// }

impl Context {
    pub fn get_ptr(&self) -> *mut cairo_t {
        let Context(ptr) = *self;
        ptr
    }

    pub fn wrap(ptr: *mut cairo_t) -> Context {
        unsafe {
            Context(ffi::cairo_reference(ptr))
        }
    }

    /// Increases the reference count on self by one. This prevents self from being destroyed
    /// until a matching call to drop() is made.
    /// 
    /// The number of references to a cairo_t can be get using Context::get_reference_count().
    pub fn reference(&self) -> Context {
        unsafe {
            Context(ffi::cairo_reference(self.get_ptr()))
        }
    }

    pub fn ensure_status(&self) {
        self.status().ensure_valid();
    }

    /// Creates a new Context with all graphics state parameters set to default values
    /// and with target as a target surface. The target surface should be constructed
    /// with a backend-specific function such as cairo_image_surface_create() (or any other
    /// Context::backend_surface_create() variant).
    /// 
    /// This function references target , so you can immediately call Surface::drop() on
    /// it if you don't need to maintain a separate reference to it.
    pub fn new(target: *mut cairo_surface_t) -> Context {
        unsafe {
            Context(ffi::cairo_create(target))
        }
    }

    /// Checks whether an error has previously occurred for this context.
    pub fn status (&self) -> Status {
        unsafe {
            ffi::cairo_status(self.get_ptr())
        }
    }

    /// Makes a copy of the current state of self and saves it on an internal stack of
    /// saved states for self. When Context::restore() is called, self will be restored to
    /// the saved state. Multiple calls to Context::save() and Context::restore() can be nested;
    /// each call to Context::restore() restores the state from the matching paired
    /// Context::save().
    /// 
    /// It isn't necessary to clear all saved states before a Context is freed. If the
    /// reference count of a Context drops to zero in response to a call to Context::drop(), any
    /// saved states will be freed along with the Context.
    pub fn save (&self) {
        unsafe {
            ffi::cairo_save(self.get_ptr())
        }
        self.ensure_status()
    }

    /// Restores self to the state saved by a preceding call to Context::save() and removes
    /// that state from the stack of saved states.
    pub fn restore (&self) {
        unsafe {
            ffi::cairo_restore(self.get_ptr())
        }
        self.ensure_status()
    }

    //fn ffi::cairo_get_target (cr: *mut cairo_t) -> *mut cairo_surface_t;

    /// Temporarily redirects drawing to an intermediate surface known as a group. The
    /// redirection lasts until the group is completed by a call to Context::pop_group()
    /// or Context::pop_group_to_source(). These calls provide the result of any drawing
    /// to the group as a pattern, (either as an explicit object, or set as the source
    /// pattern).
    /// 
    /// This group functionality can be convenient for performing intermediate compositing.
    /// One common use of a group is to render objects as opaque within the group, (so that
    /// they occlude each other), and then blend the result with translucence onto the
    /// destination.
    /// 
    /// Groups can be nested arbitrarily deep by making balanced calls to Context::push_group()
    /// / Context::pop_group(). Each call pushes  /pops the new target group onto/from a stack.
    /// 
    /// The Context::push_group() function calls Context::save() so that any changes to the
    /// graphics state will not be visible outside the group, (the pop_group functions call
    /// Context::restore()).
    /// 
    /// By default the intermediate group will have a content type of Content::ColorAlpha.
    /// Other content types can be chosen for the group by using
    /// Context::push_group_with_content() instead.
    /// 
    /// As an example, here is how one might fill and stroke a path with translucence, but
    /// without any portion of the fill being visible under the stroke:
    /// 
    /// ```
    /// cr.push_group();
    /// cr.set_source (fill_pattern);
    /// cr.fill_preserve();
    /// cr.set_source();
    /// cr.stroke();
    /// cr.pop_group_to_source();
    /// cr.paint_with_alpha(alpha);
    /// ```
    pub fn push_group(&self) {
        unsafe {
            ffi::cairo_push_group(self.get_ptr())
        }
    }

    /*
    pub fn push_group_with_content(&self, content: Content){
        unsafe {
            ffi::cairo_push_group_with_content(self.get_ptr(), content)
        }
    }*/

    /// Terminates the redirection begun by a call to Context::push_group() or
    /// Context::push_group_with_content() and returns a new pattern containing the results
    /// of all drawing operations performed to the group.
    /// 
    /// The Context::pop_group() function calls Context::restore(), (balancing a call to
    /// Context::save() by the push_group function), so that any changes to the graphics
    /// state will not be visible outside the group.
    pub fn pop_group(&self) -> Box<Pattern> {
        unsafe {
            wrap_pattern(ffi::cairo_pop_group(self.get_ptr()))
        }
    }

    /// Terminates the redirection begun by a call to Context::push_group() or
    /// Context::push_group_with_content() and installs the resulting pattern as the
    /// source pattern in the given cairo context.
    /// 
    /// The behavior of this function is equivalent to the sequence of operations:
    /// 
    /// ```
    /// let mut group = context.pop_group();
    /// context.set_source(group);
    /// ```
    /// 
    /// but is more convenient as their is no need for a variable to store the short-lived
    /// pointer to the pattern.
    /// 
    /// The Context::pop_group() function calls Context::restore(), (balancing a call to
    /// Context::save() by the push_group function), so that any changes to the graphics state
    /// will not be visible outside the group.
    pub fn pop_group_to_source(&self) {
        unsafe {
            ffi::cairo_pop_group_to_source(self.get_ptr())
        }
    }

    //fn ffi::cairo_get_group_target (cr: *mut cairo_t) -> *mut cairo_surface_t;

    /// Sets the source pattern within self to an opaque color. This opaque color will then be
    /// used for any subsequent drawing operation until a new source pattern is set.
    /// 
    /// The color components are floating point numbers in the range 0 to 1.
    /// 
    /// If the values passed in are outside that range, they will be clamped.
    /// 
    /// The default source pattern is opaque black, (that is, it is equivalent to
    /// Context::set_source_rgb(0.0, 0.0, 0.0)).
    pub fn set_source_rgb(&self, red: f64, green: f64, blue: f64) {
        unsafe {
            ffi::cairo_set_source_rgb(self.get_ptr(), red, green, blue)
        }
    }

    /// Sets the source pattern within self to a translucent color. This color will then be used
    /// for any subsequent drawing operation until a new source pattern is set.
    /// 
    /// The color and alpha components are floating point numbers in the range 0 to 1. If the
    /// values passed in are outside that range, they will be clamped.
    /// 
    /// The default source pattern is opaque black, (that is, it is equivalent to
    /// Context::set_source_rgba(0.0, 0.0, 0.0, 1.0)).
    pub fn set_source_rgba(&self, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe {
            ffi::cairo_set_source_rgba(self.get_ptr(), red, green, blue, alpha)
        }
    }

    /// Sets the source pattern within self to source. This pattern will then be used for any
    /// subsequent drawing operation until a new source pattern is set.
    /// 
    /// Note: The pattern's transformation matrix will be locked to the user space in effect
    /// at the time of Context::set_source(). This means that further modifications of the
    /// current transformation matrix will not affect the source pattern. See
    /// Pattern::set_matrix().
    /// 
    /// The default source pattern is a solid pattern that is opaque black, (that is, it is
    /// equivalent to Context::set_source_rgb(0.0, 0.0, 0.0)).
    pub fn set_source(&self, source: &Pattern) {
        unsafe {
            ffi::cairo_set_source(self.get_ptr(), source.get_ptr());
        }
        self.ensure_status();
    }

    /// Gets the current source pattern for self.
    pub fn get_source(&self) -> Box<Pattern> {
        unsafe {
            wrap_pattern(ffi::cairo_get_source(self.get_ptr()))
        }
    }

    //fn ffi::cairo_set_source_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t, x: c_double, y: c_double);

    /// Set the antialiasing mode of the rasterizer used for drawing shapes. This value
    /// is a hint, and a particular backend may or may not support a particular value.
    /// At the current time, no backend supports CAIRO_ANTIALIAS_SUBPIXEL when drawing
    /// shapes.
    /// 
    /// Note that this option does not affect text rendering, instead see
    /// FontOptions::set_antialias().
    pub fn set_antialias(&self, antialias : Antialias) {
        unsafe {
            ffi::cairo_set_antialias(self.get_ptr(), antialias)
        }
        self.ensure_status()
    }

    /// Gets the current shape antialiasing mode, as set by Context::set_antialias().
    pub fn get_antialias(&self) -> Antialias {
        unsafe {
            ffi::cairo_get_antialias(self.get_ptr())
        }
    }

    /// Sets the dash pattern to be used by cairo_stroke(). A dash pattern is specified by dashes,
    /// an array of positive values. Each value provides the length of alternate "on" and "off"
    /// portions of the stroke. The offset specifies an offset into the pattern at which the
    /// stroke begins.
    /// 
    /// Each "on" segment will have caps applied as if the segment were a separate sub-path. In
    /// particular, it is valid to use an "on" length of 0.0 with Line::CapRound or Line::CapSquare
    /// in order to distributed dots or squares along a path.
    /// 
    /// Note: The length values are in user-space units as evaluated at the time of stroking.
    /// This is not necessarily the same as the user space at the time of Context::set_dash().
    /// 
    /// If num_dashes is 0 dashing is disabled.
    /// 
    /// If num_dashes is 1 a symmetric pattern is assumed with alternating on and off portions
    /// of the size specified by the single value in dashes .
    /// 
    /// If any value in dashes is negative, or if all values are 0, then self will be put into
    /// an error state with a status of Status::InvalidDash.
    pub fn set_dash(&self, dashes: &[f64], offset: f64) {
        unsafe {
            ffi::cairo_set_dash(self.get_ptr(), dashes.as_ptr(), dashes.len() as i32, offset)
        }
        self.ensure_status(); //Possible invalid dashes value
    }

    /// This function returns the length of the dash array in self (0 if dashing is not
    /// currently in effect).
    pub fn get_dash_count(&self) -> i32 {
        unsafe {
            ffi::cairo_get_dash_count(self.get_ptr())
        }
    }

    /// Gets the current dash array. If not NULL, dashes should be big enough to hold at
    /// least the number of values returned by Context::get_dash_count().
    pub fn get_dash(&self) -> (Vec<f64>, f64) {
        let dash_count = self.get_dash_count() as usize;
        let mut dashes: Vec<f64> = Vec::with_capacity(dash_count);
        let mut offset: f64 = 0.0;

        unsafe {
            ffi::cairo_get_dash(self.get_ptr(), dashes.as_mut_ptr(), &mut offset);
            dashes.set_len(dash_count);
            (dashes, offset)
        }
    }

    pub fn get_dash_dashes(&self) -> Vec<f64> {
        let (dashes, _) = self.get_dash();
        dashes
    }

    pub fn get_dash_offset(&self) -> f64 {
        let (_, offset) = self.get_dash();
        offset
    }

    /// Set the current fill rule within the cairo context. The fill rule is used to determine
    /// which regions are inside or outside a complex (potentially self-intersecting) path.
    /// The current fill rule affects both Context::fill() and Context::clip(). See FillRule
    /// enum for details on the semantics of each available fill rule.
    /// 
    /// The default fill rule is FillRule::Winding.
    pub fn set_fill_rule(&self, fill_rule : FillRule) {
        unsafe {
            ffi::cairo_set_fill_rule(self.get_ptr(), fill_rule);
        }
        self.ensure_status();
    }

    /// Gets the current fill rule, as set by Context::set_fill_rule().
    pub fn get_fill_rule(&self) -> FillRule {
        unsafe {
            ffi::cairo_get_fill_rule(self.get_ptr())
        }
    }

    /// Sets the current line cap style within the cairo context. See LineCap enum
    /// for details about how the available line cap styles are drawn.
    /// 
    /// As with the other stroke parameters, the current line cap style is examined
    /// by Context::stroke(), Context::stroke_extents(), and Context::::stroke_to_path(),
    /// but does not have any effect during path construction.
    /// 
    /// The default line cap style is LineCap::Butt.
    pub fn set_line_cap(&self, arg: LineCap){
        unsafe {
            ffi::cairo_set_line_cap(self.get_ptr(), arg)
        }
        self.ensure_status();
    }

    /// Gets the current line cap style, as set by Context::set_line_cap().
    pub fn get_line_cap(&self) -> LineCap {
        unsafe {
            ffi::cairo_get_line_cap(self.get_ptr())
        }
    }

    /// Sets the current line join style within the cairo context. See LineJoin enum
    /// for details about how the available line join styles are drawn.
    /// 
    /// As with the other stroke parameters, the current line join style is examined by
    /// Context::stroke(), cairo_stroke_extents(), and Context::stroke_to_path(), but
    /// does not have any effect during path construction.
    /// 
    /// The default line join style is LineJoin::Miter.
    pub fn set_line_join(&self, arg: LineJoin) {
        unsafe {
            ffi::cairo_set_line_join(self.get_ptr(), arg)
        }
        self.ensure_status();
    }

    /// Gets the current line join style, as set by Context::set_line_join().
    pub fn get_line_join(&self) -> LineJoin {
        unsafe {
            ffi::cairo_get_line_join(self.get_ptr())
        }
    }

    /// Sets the current line width within the cairo context. The line width value specifies
    /// the diameter of a pen that is circular in user space, (though device-space pen may be
    /// an ellipse in general due to scaling/shear/rotation of the CTM).
    /// 
    /// Note: When the description above refers to user space and CTM it refers to the user
    /// space and CTM in effect at the time of the stroking operation, not the user space and
    /// CTM in effect at the time of the call to Context::set_line_width(). The simplest usage
    /// makes both of these spaces identical. That is, if there is no change to the CTM between
    /// a call to Context::set_line_width() and the stroking operation, then one can just pass
    /// user-space values to Context::set_line_width() and ignore this note.
    /// 
    /// As with the other stroke parameters, the current line width is examined by
    /// Context::stroke(), Context::stroke_extents(), and Context::stroke_to_path(), but does
    /// not have any effect during path construction.
    /// 
    /// The default line width value is 2.0.
    pub fn set_line_width(&self, arg: f64) {
        unsafe {
            ffi::cairo_set_line_width(self.get_ptr(), arg)
        }
        self.ensure_status();
    }

    /// This function returns the current line width value exactly as set by
    /// Context::set_line_width(). Note that the value is unchanged even if the CTM has changed
    /// between the calls to Context::set_line_width() and Context::get_line_width().
    pub fn get_line_width(&self) -> f64 {
        unsafe {
            ffi::cairo_get_line_width(self.get_ptr())
        }
    }

    /// Sets the current miter limit within the cairo context.
    /// 
    /// If the current line join style is set to LineJoin::Miter (see Context::set_line_join()),
    /// the miter limit is used to determine whether the lines should be joined with a bevel
    /// instead of a miter. Cairo divides the length of the miter by the line width. If the result
    /// is greater than the miter limit, the style is converted to a bevel.
    /// 
    /// As with the other stroke parameters, the current line miter limit is examined by
    /// Context::stroke(), Context::stroke_extents(), and Context::stroke_to_path(), but does
    /// not have any effect during path construction.
    /// 
    /// The default miter limit value is 10.0, which will convert joins with interior angles less
    /// than 11 degrees to bevels instead of miters. For reference, a miter limit of 2.0 makes the
    /// miter cutoff at 60 degrees, and a miter limit of 1.414 makes the cutoff at 90 degrees.
    /// 
    /// A miter limit for a desired angle can be computed as: miter limit = 1/sin(angle/2)
    pub fn set_miter_limit(&self, arg: f64) {
        unsafe {
            ffi::cairo_set_miter_limit(self.get_ptr(), arg)
        }
        self.ensure_status();
    }

    /// Gets the current miter limit, as set by Contextset_miter_limit().
    pub fn get_miter_limit(&self) -> f64 {
        unsafe {
            ffi::cairo_get_miter_limit(self.get_ptr())
        }
    }

    /// Sets the tolerance used when converting paths into trapezoids. Curved segments of
    /// the path will be subdivided until the maximum deviation between the original path
    /// and the polygonal approximation is less than tolerance . The default value is 0.1.
    /// A larger value will give better performance, a smaller value, better appearance.
    /// (Reducing the value from the default value of 0.1 is unlikely to improve appearance
    /// significantly.) The accuracy of paths within Cairo is limited by the precision of
    /// its internal arithmetic, and the prescribed tolerance is restricted to the smallest
    /// representable internal value.
    pub fn set_tolerance(&self, arg: f64) {
        unsafe {
            ffi::cairo_set_tolerance(self.get_ptr(), arg)
        }
        self.ensure_status();
    }

    /// Gets the current tolerance value, as set by Context::set_tolerance().
    pub fn get_tolerance(&self) -> f64 {
        unsafe {
            ffi::cairo_get_tolerance(self.get_ptr())
        }
    }

    /// Establishes a new clip region by intersecting the current clip region with the current
    /// path as it would be filled by Context::fill() and according to the current fill rule
    /// (see Context::set_fill_rule()).
    /// 
    /// After Context::clip(), the current path will be cleared from the cairo context.
    /// 
    /// The current clip region affects all drawing operations by effectively masking out any
    /// changes to the surface that are outside the current clip region.
    /// 
    /// Calling Context::clip() can only make the clip region smaller, never larger. But the
    /// current clip is part of the graphics state, so a temporary restriction of the clip
    /// region can be achieved by calling Context::clip() within a Context::save() /
    /// Context::restore() pair. The only other means of increasing the size of the clip
    /// region is Context::reset_clip().
    pub fn clip(&self) {
        unsafe {
            ffi::cairo_clip(self.get_ptr())
        }
    }

    /// Establishes a new clip region by intersecting the current clip region with the current
    /// path as it would be filled by Context::fill() and according to the current fill rule
    /// (see Context::set_fill_rule()).
    /// 
    /// Unlike Context::clip(), Context::clip_preserve() preserves the path within the cairo
    /// context.
    /// 
    /// The current clip region affects all drawing operations by effectively masking out any
    /// changes to the surface that are outside the current clip region.
    /// 
    /// Calling Context::clip_preserve() can only make the clip region smaller, never larger.
    /// But the current clip is part of the graphics state, so a temporary restriction of the
    /// clip region can be achieved by calling Context::clip_preserve() within a
    /// Context::save()/cairo_restore() pair. The only other means of increasing the size of
    /// the clip region is Context::reset_clip().
    pub fn clip_preserve(&self) {
        unsafe {
            ffi::cairo_clip_preserve(self.get_ptr())
        }
    }

    /// Computes a bounding box in user coordinates covering the area inside the current clip.
    pub fn clip_extents(&self) -> (f64, f64, f64, f64) {
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe {
            ffi::cairo_clip_extents(self.get_ptr(), &mut x1, &mut y1, &mut x2, &mut y2);
        }
        (x1, y1, x2, y2)
    }

    /// Tests whether the given point is inside the area that would be visible through the current
    /// clip, i.e. the area that would be filled by a Context::paint() operation.
    /// 
    /// See Context::clip(), and Context::clip_preserve().
    pub fn in_clip(&self, x:f64, y:f64) -> bool {
        unsafe {
            ffi::cairo_in_clip(self.get_ptr(), x, y).as_bool()
        }
    }

    /// Reset the current clip region to its original, unrestricted state. That is, set the clip
    /// region to an infinitely large shape containing the target surface. Equivalently, if
    /// infinity is too hard to grasp, one can imagine the clip region being reset to the exact
    /// bounds of the target surface.
    /// 
    /// Note that code meant to be reusable should not call Context::reset_clip() as it will
    /// cause results unexpected by higher-level code which calls Context::clip(). Consider
    /// using Context::save() and Context::restore() around Context::clip() as a more robust
    /// means of temporarily restricting the clip region.
    pub fn reset_clip(&self) {
        unsafe {
            ffi::cairo_reset_clip(self.get_ptr())
        }
        self.ensure_status()
    }

    /// Gets the current clip region as a list of rectangles in user coordinates.
    /// 
    /// The status in the list may be Status::ClipNotRepresentable to indicate that
    /// the clip region cannot be represented as a list of user-space rectangles.
    /// The status may have other values to indicate other errors.
    pub fn copy_clip_rectangle_list(&self) -> RectangleVec {
        unsafe {
            let rectangle_list = ffi::cairo_copy_clip_rectangle_list(self.get_ptr());

            (*rectangle_list).status.ensure_valid();

            RectangleVec {
                ptr: rectangle_list,
                rectangles: CVec::new((*rectangle_list).rectangles,
                                      (*rectangle_list).num_rectangles as usize),
            }
        }
    }

    /// A drawing operator that fills the current path according to the current fill
    /// rule, (each sub-path is implicitly closed before being filled). After
    /// Context::fill(), the current path will be cleared from the cairo context. See
    /// Context::set_fill_rule() and Context::fill_preserve().
    pub fn fill(&self) {
        unsafe {
            ffi::cairo_fill(self.get_ptr())
        }
    }

    /// A drawing operator that fills the current path according to the current fill rule,
    /// (each sub-path is implicitly closed before being filled). Unlike Context::fill(),
    /// Context::fill_preserve() preserves the path within the cairo context.
    /// 
    /// See Context::set_fill_rule() and Context::fill().
    pub fn fill_preserve(&self) {
        unsafe {
            ffi::cairo_fill_preserve(self.get_ptr())
        }
    }

    /// Computes a bounding box in user coordinates covering the area that would be affected,
    /// (the "inked" area), by a Context::fill() operation given the current path and fill
    /// parameters. If the current path is empty, returns an empty rectangle ((0,0), (0,0)).
    /// Surface dimensions and clipping are not taken into account.
    /// 
    /// Contrast with Context::path_extents(), which is similar, but returns non-zero extents
    /// for some paths with no inked area, (such as a simple line segment).
    /// 
    /// Note that Context::fill_extents() must necessarily do more work to compute the precise
    /// inked areas in light of the fill rule, so Context::path_extents() may be more desirable
    /// for sake of performance if the non-inked path extents are desired.
    /// 
    /// See Context::fill(), Context::set_fill_rule() and Context::fill_preserve().
    pub fn fill_extents(&self) -> (f64, f64, f64, f64) {
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe {
            ffi::cairo_fill_extents(self.get_ptr(), &mut x1, &mut y1, &mut x2, &mut y2);
        }
        (x1, y1, x2, y2)
    }

    /// Tests whether the given point is inside the area that would be affected by a
    /// Context::fill() operation given the current path and filling parameters. Surface
    /// dimensions and clipping are not taken into account.
    /// 
    /// See Context::fill(), Context::set_fill_rule() and Context::fill_preserve().
    pub fn in_fill(&self, x:f64, y:f64) -> bool {
        unsafe {
            ffi::cairo_in_fill(self.get_ptr(), x, y).as_bool()
        }
    }

    /// A drawing operator that paints the current source using the alpha channel of
    /// pattern as a mask. (Opaque areas of pattern are painted with the source, transparent
    /// areas are not painted.)
    pub fn mask(&self, pattern: &Pattern) {
        unsafe {
            ffi::cairo_mask(self.get_ptr(), pattern.get_ptr())
        }
    }

    //fn ffi::cairo_mask_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t, surface_x: c_double, surface_y: c_double);

    /// A drawing operator that paints the current source everywhere within the current clip region.
    pub fn paint(&self) {
        unsafe {
            ffi::cairo_paint(self.get_ptr())
        }
    }

    /// A drawing operator that paints the current source everywhere within the current clip region
    /// using a mask of constant alpha value alpha . The effect is similar to Context::paint(), but
    /// the drawing is faded out using the alpha value.
    pub fn paint_with_alpha(&self, alpha: f64) {
        unsafe {
            ffi::cairo_paint_with_alpha(self.get_ptr(), alpha)
        }
    }

    /// A drawing operator that strokes the current path according to the current line width, line
    /// join, line cap, and dash settings. After Context::stroke(), the current path will be cleared
    /// from the cairo context. See Context::set_line_width(), Context::set_line_join(),
    /// Context::set_line_cap(), Context::set_dash(), and Context::stroke_preserve().
    /// 
    /// Note: Degenerate segments and sub-paths are treated specially and provide a useful result.
    /// These can result in two different situations:
    /// 
    /// 1. Zero-length "on" segments set in Context::set_dash(). If the cap style is LineCap::Round
    /// or LineCap::Square then these segments will be drawn as circular dots or squares respectively.
    /// In the case of LineCap::Square, the orientation of the squares is determined by the direction
    /// of the underlying path.
    /// 
    /// 2. A sub-path created by Context::move_to() followed by either a Context::close_path() or one
    /// or more calls to Context::line_to() to the same coordinate as the Context::move_to(). If the
    /// cap style is LineCap::Round then these sub-paths will be drawn as circular dots. Note that in
    /// the case of LineCap::Square a degenerate sub-path will not be drawn at all, (since the correct
    /// orientation is indeterminate).
    /// 
    /// In no case will a cap style of LineCap::Butt cause anything to be drawn in the case of either
    /// degenerate segments or sub-paths.
    pub fn stroke(&self) {
        unsafe {
            ffi::cairo_stroke(self.get_ptr())
        }
    }

    /// A drawing operator that strokes the current path according to the current line width, line
    /// join, line cap, and dash settings. Unlike Context::stroke(), Context::stroke_preserve()
    /// preserves the path within the cairo context.
    /// 
    /// See Context::set_line_width(), Context::set_line_join(), Context::set_line_cap(),
    /// Context::set_dash(), and Context::stroke_preserve().
    pub fn stroke_preserve(&self) {
        unsafe {
            ffi::cairo_stroke_preserve(self.get_ptr())
        }
    }

    /// Computes a bounding box in user coordinates covering the area that would be affected,
    /// (the "inked" area), by a Context::stroke() operation given the current path and stroke
    /// parameters. If the current path is empty, returns an empty rectangle ((0,0), (0,0)).
    /// Surface dimensions and clipping are not taken into account.
    /// 
    /// Note that if the line width is set to exactly zero, then Context::stroke_extents() will
    /// return an empty rectangle. Contrast with cairo_path_extents() which can be used to compute
    /// the non-empty bounds as the line width approaches zero.
    /// 
    /// Note that cairo_stroke_extents() must necessarily do more work to compute the precise inked
    /// areas in light of the stroke parameters, so cairo_path_extents() may be more desirable for
    /// sake of performance if non-inked path extents are desired.
    /// 
    /// See Context::stroke(), Context::set_line_width(), Context::set_line_join(),
    /// Context::set_line_cap(), Context::set_dash(), and Context::stroke_preserve().
    pub fn stroke_extents(&self) -> (f64, f64, f64, f64) {
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;

        unsafe {
            ffi::cairo_stroke_extents(self.get_ptr(), &mut x1, &mut y1, &mut x2, &mut y2);
        }
        (x1, y1, x2, y2)
    }

    /// Tests whether the given point is inside the area that would be affected by a 
    /// Context::stroke() operation given the current path and stroking parameters. Surface
    /// dimensions and clipping are not taken into account.
    /// 
    /// See Context::stroke(), Context::set_line_width(), Context::set_line_join(),
    /// Context::set_line_cap(), Context::set_dash(), and Context::stroke_preserve().
    pub fn in_stroke(&self, x:f64, y:f64) -> bool {
        unsafe {
            ffi::cairo_in_stroke(self.get_ptr(), x, y).as_bool()
        }
    }

    /// Emits the current page for backends that support multiple pages, but doesn't clear
    /// it, so, the contents of the current page will be retained for the next page too. Use
    /// Context::show_page() if you want to get an empty page after the emission.
    /// 
    /// This is a convenience function that simply calls Surface::copy_page() on self's
    /// target.
    pub fn copy_page(&self) {
        unsafe {
            ffi::cairo_copy_page(self.get_ptr())
        }
    }

    /// Emits and clears the current page for backends that support multiple pages. Use
    /// Context::copy_page() if you don't want to clear the page.
    /// 
    /// This is a convenience function that simply calls Surface::show_page() on self's
    /// target.
    pub fn show_page(&self) {
        unsafe {
            ffi::cairo_show_page(self.get_ptr())
        }
    }

    /// Returns the current reference count of self.
    pub fn get_reference_count(&self) -> u32 {
        unsafe {
            ffi::cairo_get_reference_count(self.get_ptr())
        }
    }

    // transformations stuff

    /// Modifies the current transformation matrix (CTM) by translating the user-space
    /// origin by (tx , ty ). This offset is interpreted as a user-space coordinate
    /// according to the CTM in place before the new call to cairo_translate(). In other
    /// words, the translation of the user-space origin takes place after any existing
    /// transformation.
     pub fn translate(&self, tx: f64, ty: f64) {
        unsafe {
            ffi::cairo_translate(self.get_ptr(), tx, ty)
        }
    }

    /// Modifies the current transformation matrix (CTM) by scaling the X and Y user-space
    /// axes by sx and sy respectively. The scaling of the axes takes place after any
    /// existing transformation of user space.
    pub fn scale(&self, sx: f64, sy: f64) {
        unsafe {
            ffi::cairo_scale(self.get_ptr(), sx, sy)
        }
    }

    /// Modifies the current transformation matrix (CTM) by rotating the user-space axes by
    /// angle radians. The rotation of the axes takes places after any existing transformation
    /// of user space. The rotation direction for positive angles is from the positive X axis
    /// toward the positive Y axis.
    pub fn rotate(&self, angle: f64) {
        unsafe {
            ffi::cairo_rotate(self.get_ptr(), angle)
        }
    }

    //pub fn cairo_transform(cr: *cairo_t, matrix: *cairo_matrix_t);

    //pub fn cairo_set_matrix(cr: *cairo_t, matrix: *cairo_matrix_t);

    //pub fn cairo_get_matrix(cr: *cairo_t, matrix: *cairo_matrix_t);

    /// Resets the current transformation matrix (CTM) by setting it equal to the identity
    /// matrix. That is, the user-space and device-space axes will be aligned and one user-space
    /// unit will transform to one device-space unit.
    pub fn identity_matrix(&self) {
        unsafe {
            ffi::cairo_identity_matrix(self.get_ptr())
        }
    }

    /// Transform a coordinate from user space to device space by multiplying the given point
    /// by the current transformation matrix (CTM).
    pub fn user_to_device(&self, x: f64, y: f64) -> (f64, f64) {
        unsafe {
            let x_ptr: *mut c_double = transmute(Box::new(x));
            let y_ptr: *mut c_double = transmute(Box::new(y));

            ffi::cairo_user_to_device(self.get_ptr(), x_ptr, y_ptr);

            let x_box: Box<f64> = transmute(x_ptr);
            let y_box: Box<f64> = transmute(y_ptr);

            (*x_box, *y_box)
        }
    }

    /// Transform a distance vector from user space to device space. This function is similar
    /// to Context::user_to_device() except that the translation components of the CTM will
    /// be ignored when transforming (dx ,dy ).
    pub fn user_to_device_distance(&self, dx: f64, dy: f64) -> (f64, f64) {
        unsafe {
            let dx_ptr: *mut c_double = transmute(Box::new(dx));
            let dy_ptr: *mut c_double = transmute(Box::new(dy));

            ffi::cairo_user_to_device_distance(self.get_ptr(), dx_ptr, dy_ptr);

            let dx_box: Box<f64> = transmute(dx_ptr);
            let dy_box: Box<f64> = transmute(dy_ptr);

            (*dx_box, *dy_box)
        }
    }

    /// Transform a coordinate from device space to user space by multiplying the given point
    /// by the inverse of the current transformation matrix (CTM).
    pub fn device_to_user(&self, x: f64, y: f64) -> (f64, f64) {
        unsafe {
            let x_ptr: *mut c_double = transmute(Box::new(x));
            let y_ptr: *mut c_double = transmute(Box::new(y));

            ffi::cairo_device_to_user(self.get_ptr(), x_ptr, y_ptr);

            let x_box: Box<f64> = transmute(x_ptr);
            let y_box: Box<f64> = transmute(y_ptr);

            (*x_box, *y_box)
        }
    }

    /// Transform a distance vector from device space to user space. This function is similar
    /// to Context::device_to_user() except that the translation components of the inverse CTM
    /// will be ignored when transforming (dx ,dy ).
    pub fn device_to_user_distance(&self, dx: f64, dy: f64) -> (f64, f64) {
        unsafe {
            let dx_ptr: *mut c_double = transmute(Box::new(dx));
            let dy_ptr: *mut c_double = transmute(Box::new(dy));

            ffi::cairo_device_to_user_distance(self.get_ptr(), dx_ptr, dy_ptr);

            let dx_box: Box<f64> = transmute(dx_ptr);
            let dy_box: Box<f64> = transmute(dy_ptr);

            (*dx_box, *dy_box)
        }
    }

    // font stuff

    /// Note: The Context::select_font_face() function call is part of what the cairo designers
    /// call the "toy" text API. It is convenient for short demos and simple programs, but it
    /// is not expected to be adequate for serious text-using applications.
    /// 
    /// Selects a family and style of font from a simplified description as a family name, slant
    /// and weight. Cairo provides no operation to list available family names on the system (this
    /// is a "toy", remember), but the standard CSS2 generic family names, ("serif", "sans-serif",
    /// "cursive", "fantasy", "monospace"), are likely to work as expected.
    /// 
    /// If family starts with the string "cairo :", or if no native font backends are compiled in,
    /// cairo will use an internal font family. The internal font family recognizes many modifiers
    /// in the family string, most notably, it recognizes the string "monospace". That is, the
    /// family name "cairo :monospace" will use the monospace version of the internal font family.
    /// 
    /// For "real" font selection, see the font-backend-specific font_face_create functions for the
    /// font backend you are using. (For example, if you are using the freetype-based cairo-ft font
    /// backend, see Font::create_for_ft_face() or Font::create_for_pattern().) The resulting font
    /// face could then be used with Context::scaled_font_create() and Context::set_scaled_font().
    /// 
    /// Similarly, when using the "real" font support, you can call directly into the underlying
    /// font system, (such as fontconfig or freetype), for operations such as listing available
    /// fonts, etc.
    /// 
    /// It is expected that most applications will need to use a more comprehensive font handling
    /// and text layout library, (for example, pango), in conjunction with cairo.
    /// 
    /// If text is drawn without a call to Context::select_font_face(), (nor Context::set_font_face()
    /// nor Context::set_scaled_font()), the default family is platform-specific, but is essentially
    /// "sans-serif". Default slant is FontSlant::Normal, and default weight is FontWeight::Normal.
    /// 
    /// This function is equivalent to a call to cairo_toy_font_face_create() followed by
    /// Context::set_font_face().
    pub fn select_font_face(&self, family: &str, slant: FontSlant, weight: FontWeight){
        unsafe {
            ffi::cairo_select_font_face(self.get_ptr(), family.to_glib_none().0, slant, weight)
        }
    }

    /// Sets the current font matrix to a scale by a factor of size , replacing any font matrix
    /// previously set with Context::set_font_size() or Context::set_font_matrix(). This results
    /// in a font size of size user space units. (More precisely, this matrix will result in the
    /// font's em-square being a size by size square in user space.)
    /// 
    /// If text is drawn without a call to Context::set_font_size(), (nor
    /// Context::set_font_matrix() nor Context::set_scaled_font()), the default font size is 10.0.
    pub fn set_font_size(&self, size: f64){
        unsafe {
            ffi::cairo_set_font_size(self.get_ptr(), size)
        }
    }

    /// Sets the current font matrix to matrix . The font matrix gives a transformation from the
    /// design space of the font (in this space, the em-square is 1 unit by 1 unit) to user space.
    /// Normally, a simple scale is used (see Context::set_font_size()), but a more complex font
    /// matrix can be used to shear the font or stretch it unequally along the two axes.
    // FIXME probably needs a heap allocation
    pub fn set_font_matrix(&self, matrix: Matrix){
        unsafe {
            ffi::cairo_set_font_matrix(self.get_ptr(), &matrix)
        }
    }

    /// Stores the current font matrix into matrix . See Context::set_font_matrix().
    pub fn get_font_matrix(&self) -> Matrix {
        let mut matrix = <Matrix as MatrixTrait>::null();
        unsafe {
            ffi::cairo_get_font_matrix(self.get_ptr(), &mut matrix);
        }
        matrix
    }

    /// Sets a set of custom font rendering options for the Context. Rendering options are
    /// derived by merging these options with the options derived from underlying surface;
    /// if the value in options has a default value (like Antialias::Default), then the value
    /// from the surface is used.
    pub fn set_font_options(&self, options: FontOptions){
        unsafe {
            ffi::cairo_set_font_options(self.get_ptr(), options.get_ptr())
        }
    }

    /// Retrieves font rendering options set via Context::set_font_options. Note that the returned
    /// options do not include any options derived from the underlying surface; they are literally
    /// the options passed to Context::set_font_options().
    pub fn get_font_options(&self) -> FontOptions {
        let out = FontOptions::new();
        unsafe {
            ffi::cairo_get_font_options(self.get_ptr(), out.get_ptr());
        }
        out
    }

    /// Replaces the current FontFace object in the Context with font_face. The replaced
    /// font face in the cairo_t will be destroyed if there are no other references to it.
    pub fn set_font_face(&self, font_face: FontFace){
        unsafe {
            ffi::cairo_set_font_face(self.get_ptr(), font_face.get_ptr())
        }
    }

    /// Gets the current font face for a Context object.
    pub fn get_font_face(&self) -> FontFace {
        unsafe {
            FontFace(ffi::cairo_get_font_face(self.get_ptr()))
        }
    }

    /// Replaces the current font face, font matrix, and font options in the Context with
    /// those of the ScaledFont object. Except for some translation, the current CTM of the
    /// Context should be the same as that of the ScaledFont object, which can be accessed
    /// using Context::scaled_font_get_ctm().
    pub fn set_scaled_font(&self, scaled_font: ScaledFont){
        unsafe {
            ffi::cairo_set_scaled_font(self.get_ptr(), scaled_font.get_ptr())
        }
    }

    /// Gets the current scaled font for a Context.
    pub fn get_scaled_font(&self) -> ScaledFont {
        unsafe {
            ScaledFont(ffi::cairo_get_scaled_font(self.get_ptr()))
        }
    }

    pub fn show_text(&self, text: &str){
        unsafe {
            ffi::cairo_show_text(self.get_ptr(), text.to_glib_none().0)
        }
    }

    pub fn show_glyphs(&self, glyphs: &[Glyph]){
        unsafe {
            ffi::cairo_show_glyphs(self.get_ptr(), glyphs.as_ptr(), glyphs.len() as c_int)
        }
    }

    pub fn show_text_glyphs(&self,
                            text: &str,
                            glyphs: &[Glyph],
                            clusters: &[TextCluster],
                            cluster_flags: TextClusterFlags){
        unsafe {
            ffi::cairo_show_text_glyphs(self.get_ptr(),
                                        text.to_glib_none().0,
                                        -1 as c_int, //NUL terminated
                                        glyphs.as_ptr(),
                                        glyphs.len() as c_int,
                                        clusters.as_ptr(),
                                        clusters.len() as c_int,
                                        cluster_flags)
    }
    }

    pub fn font_extents(&self) -> FontExtents {
        let mut extents = FontExtents {
            ascent: 0.0,
            descent: 0.0,
            height: 0.0,
            max_x_advance: 0.0,
            max_y_advance: 0.0,
        };

        unsafe {
            ffi::cairo_font_extents(self.get_ptr(), &mut extents);
        }

        extents
    }

    pub fn text_extents(&self, text: &str) -> TextExtents {
        let mut extents = TextExtents {
            x_bearing: 0.0,
            y_bearing: 0.0,
            width: 0.0,
            height: 0.0,
            x_advance: 0.0,
            y_advance: 0.0,
        };

        unsafe {
            ffi::cairo_text_extents(self.get_ptr(), text.to_glib_none().0, &mut extents);
        }
        extents
    }

    pub fn glyph_extents(&self, glyphs: &[Glyph]) -> TextExtents {
        let mut extents = TextExtents {
            x_bearing: 0.0,
            y_bearing: 0.0,
            width: 0.0,
            height: 0.0,
            x_advance: 0.0,
            y_advance: 0.0,
        };

        unsafe {
            ffi::cairo_glyph_extents(self.get_ptr(), glyphs.as_ptr(), glyphs.len() as c_int, &mut extents);
        }

        extents
    }

    // paths stuff

     pub fn copy_path(&self) -> Path {
        unsafe {
            Path::wrap(ffi::cairo_copy_path(self.get_ptr()))
        }
    }

    pub fn copy_path_flat(&self) -> Path {
        unsafe {
            Path::wrap(ffi::cairo_copy_path_flat(self.get_ptr()))
        }
    }

    pub fn append_path(&self, path: &Path){
        unsafe {
            ffi::cairo_append_path(self.get_ptr(), path.get_ptr())
        }
    }

    pub fn has_current_point(&self) -> bool {
        unsafe {
            ffi::cairo_has_current_point(self.get_ptr()).as_bool()
        }
    }

    pub fn get_current_point(&self) -> (f64, f64) {
        unsafe {
            let x = transmute(Box::new(0.0f64));
            let y = transmute(Box::new(0.0f64));
            ffi::cairo_get_current_point(self.get_ptr(), x, y);
            (*x, *y)
        }
    }

    pub fn new_path(&self){
        unsafe {
            ffi::cairo_new_path(self.get_ptr())
        }
    }

    pub fn new_sub_path(&self){
        unsafe {
            ffi::cairo_new_sub_path(self.get_ptr())
        }
    }

    pub fn close_path(&self){
        unsafe {
            ffi::cairo_close_path(self.get_ptr())
        }
    }

    pub fn arc(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64){
        unsafe {
            ffi::cairo_arc(self.get_ptr(), xc, yc, radius, angle1, angle2)
        }
    }

    pub fn arc_negative(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64){
        unsafe {
            ffi::cairo_arc_negative(self.get_ptr(), xc, yc, radius, angle1, angle2)
        }
    }

    pub fn curve_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64){
        unsafe {
            ffi::cairo_curve_to(self.get_ptr(), x1, y1, x2, y2, x3, y3)
        }
    }

    pub fn line_to(&self, x: f64, y: f64){
        unsafe {
            ffi::cairo_line_to(self.get_ptr(), x, y)
        }
    }

    pub fn move_to(&self, x: f64, y: f64){
        unsafe {
            ffi::cairo_move_to(self.get_ptr(), x, y)
        }
    }

    pub fn rectangle(&self, x: f64, y: f64, width: f64, height: f64){
        unsafe {
            ffi::cairo_rectangle(self.get_ptr(), x, y, width, height)
        }
    }

    pub fn text_path(&self, str_: &str){
        unsafe {
            ffi::cairo_text_path(self.get_ptr(), str_.to_glib_none().0)
        }
    }

    //fn ffi::cairo_glyph_path(cr: *mut cairo_t, glyphs: *mut cairo_glyph_t, num_glyphs: isize);

    pub fn rel_curve_to(&self, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64){
        unsafe {
            ffi::cairo_rel_curve_to(self.get_ptr(), dx1, dy1, dx2, dy2, dx3, dy3)
        }
    }

    pub fn rel_line_to(&self, dx: f64, dy: f64){
        unsafe {
            ffi::cairo_rel_line_to(self.get_ptr(), dx, dy)
        }
    }

    pub fn rel_move_to(&self, dx: f64, dy: f64){
        unsafe {
            ffi::cairo_rel_move_to(self.get_ptr(), dx, dy)
        }
    }
}
