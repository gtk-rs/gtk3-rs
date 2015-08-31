// Copyright 2013-2015, The RGtk Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Sources for drawing

#![cfg_attr(not(cairo_1_12), allow(unused_imports))]

use libc::{c_double, c_int, c_uint};
use std::mem::transmute;
use ffi::enums::{
    Extend,
    Filter,
    Status,
    PatternType
};
use ffi;
use ffi::{
    cairo_pattern_t
};
use ::{
    Path
};

//Quite some changes from the C api but all suggested by the cairo devs.
//See http://cairographics.org/manual/bindings-patterns.html for more info


//TODO Does anyone know a way to do this without dynamic dispatch -- @mthq
pub fn wrap_pattern<'a>(ptr: *mut cairo_pattern_t) -> Box<Pattern + 'a> {
    let pattern_type = unsafe{ ffi::cairo_pattern_get_type(ptr) };

    match pattern_type {
        PatternType::Solid            => Box::new(SolidPattern::wrap(ptr))   as Box<Pattern>,
        PatternType::Surface          => Box::new(SurfacePattern::wrap(ptr)) as Box<Pattern>,
        PatternType::LinearGradient   => Box::new(LinearGradient::wrap(ptr)) as Box<Pattern>,
        PatternType::RadialGradient   => Box::new(RadialGradient::wrap(ptr)) as Box<Pattern>,
        #[cfg(cairo_1_12)]
        PatternType::Mesh             => Box::new(Mesh::wrap(ptr))           as Box<Pattern>,
        #[cfg(cairo_1_12)]
        PatternType::RasterSource     => panic!("Not implemented")
    }
}

pub trait Pattern {
    #[doc(hidden)]
    fn get_ptr(&self) -> *mut cairo_pattern_t;

    fn ensure_status(&self) {
        self.status().ensure_valid();
    }

    /// Checks whether an error has previously occurred for this pattern.
    fn status(&self) -> Status {
        unsafe {
            ffi::cairo_pattern_status(self.get_ptr())
        }
    }

    /// Returns the current reference count of self.
    fn get_reference_count(&self) -> isize {
        unsafe {
            ffi::cairo_pattern_get_reference_count(self.get_ptr()) as isize
        }
    }

    /// Sets the mode to be used for drawing outside the area of a pattern. See cairo_extend_t for
    /// details on the semantics of each extend strategy.
    /// 
    /// The default extend mode is Extend::None for surface patterns and Extend::Pad for gradient
    /// patterns.
    fn set_extend(&self, extend: Extend) {
        unsafe {
            ffi::cairo_pattern_set_extend(self.get_ptr(), extend)
        }
    }

    /// Gets the current extend mode for a pattern. See Extend enum for details on the semantics
    /// of each extend strategy.
    fn get_extend(&self) -> Extend {
        unsafe {
            ffi::cairo_pattern_get_extend(self.get_ptr())
        }
    }

    /// Sets the filter to be used for resizing when using this pattern. See Filter enum for
    /// details on each filter.
    /// 
    /// Note that you might want to control filtering even when you do not have an explicit
    /// Pattern object, (for example when using cairo_set_source_surface()). In these
    /// cases, it is convenient to use cairo_get_source() to get access to the pattern that cairo
    /// creates implicitly. For example:
    /// 
    /// ```ignore
    /// Context::set_source_surface(image, x, y);
    /// p.set_filter(Filter::nearest);
    /// ```
    fn set_filter(&self, filter: Filter) {
        unsafe {
            ffi::cairo_pattern_set_filter(self.get_ptr(), filter)
        }
    }

    /// Gets the current filter for a pattern. See Filter enum for details on each filter.
    fn get_filter(&self) -> Filter {
        unsafe {
            ffi::cairo_pattern_get_filter(self.get_ptr())
        }
    }

    //fn cairo_pattern_set_matrix(pattern: *mut cairo_pattern_t, matrix: *mut cairo_matrix_t);

    //fn cairo_pattern_get_matrix(pattern: *mut cairo_pattern_t, matrix: *mut cairo_matrix_t);
}

macro_rules! pattern_type(
    //Signals without arguments
    ($pattern_type:ident) => (

        pub struct $pattern_type {
            pointer: *mut cairo_pattern_t
        }

        impl $pattern_type {
            pub fn wrap(pointer: *mut cairo_pattern_t) -> $pattern_type {
                $pattern_type{
                    pointer: pointer
                }
            }

            pub fn reference(&self) -> $pattern_type {
                $pattern_type{
                    pointer: unsafe {
                        ffi::cairo_pattern_reference(self.pointer)
                    }
                }
            }
        }

        impl Pattern for $pattern_type {
            fn get_ptr(&self) -> *mut cairo_pattern_t{
                self.pointer
            }
        }

        impl Drop for $pattern_type {
            fn drop(&mut self){
                unsafe {
                    ffi::cairo_pattern_destroy(self.pointer)
                }
            }
        }
    );
);

pattern_type!(SolidPattern);

impl SolidPattern {
    /// Creates a new SolidPattern corresponding to an opaque color. The color components
    /// are floating point numbers in the range 0 to 1.
    /// 
    /// Note : If the values passed in are outside
    /// that range, they will be clamped.
    pub fn from_rgb(red: f64, green: f64, blue: f64) -> SolidPattern {
        SolidPattern::wrap(unsafe {
            ffi::cairo_pattern_create_rgb(red, green, blue)
        })
    }

    /// Creates a new SolidPattern corresponding to a translucent color. The color components
    /// are floating point numbers in the range 0 to
    /// 
    /// Note : If the values passed in are outside that range, they will be clamped.
    pub fn from_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> SolidPattern {
        SolidPattern::wrap(unsafe {
            ffi::cairo_pattern_create_rgba(red, green, blue, alpha)
        })
    }

    /// Gets the solid color for a solid color pattern.
    pub fn get_rgba(&self) -> (f64, f64, f64, f64) {
        unsafe {
            let red  : *mut c_double = transmute(Box::new(0.0f64));
            let green: *mut c_double = transmute(Box::new(0.0f64));
            let blue : *mut c_double = transmute(Box::new(0.0f64));
            let alpha: *mut c_double = transmute(Box::new(0.0f64));

            ffi::cairo_pattern_get_rgba(self.pointer, red, green, blue, alpha).ensure_valid();

            (*red, *green, *blue, *alpha)
        }
    }
}


pub trait Gradient : Pattern {
    /// Adds an opaque color stop to a gradient pattern. The offset specifies the
    /// location along the gradient's control vector. For example, a linear gradient's
    /// control vector is from (x0,y0) to (x1,y1) while a radial gradient's control
    /// vector is from any point on the start circle to the corresponding point on the
    /// end circle.
    /// 
    /// The color is specified in the same way as in Context::set_source_rgba().
    /// 
    /// If two (or more) stops are specified with identical offset values, they will be
    /// sorted according to the order in which the stops are added, (stops added earlier
    /// will compare less than stops added later). This can be useful for reliably making
    /// sharp color transitions instead of the typical blend.
    /// 
    /// Note: If the pattern is not a gradient pattern, (eg. a linear or radial pattern),
    /// then the pattern will be put into an error status with a status of
    /// StatusPattern::TypeMismatch.
    fn add_color_stop_rgb(&self, offset: f64, red: f64, green: f64, blue: f64) {
        unsafe {
            ffi::cairo_pattern_add_color_stop_rgb(self.get_ptr(), offset, red, green, blue)
        }
    }

    /// Adds a translucent color stop to a gradient pattern. The offset specifies the
    /// location along the gradient's control vector. For example, a linear gradient's
    /// control vector is from (x0,y0) to (x1,y1) while a radial gradient's control vector
    /// is from any point on the start circle to the corresponding point on the end circle.
    /// 
    /// The color is specified in the same way as in Context::set_source_rgba().
    /// 
    /// If two (or more) stops are specified with identical offset values, they will be
    /// sorted according to the order in which the stops are added, (stops added earlier will
    /// compare less than stops added later). This can be useful for reliably making sharp
    /// color transitions instead of the typical blend.
    /// 
    /// Note: If the pattern is not a gradient pattern, (eg. a linear or radial pattern), then
    /// the pattern will be put into an error status with a status of StatusPattern::TypeMismatch.
    fn add_color_stop_rgba(&self, offset: f64, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe {
            ffi::cairo_pattern_add_color_stop_rgba(self.get_ptr(), offset, red, green, blue, alpha)
        }
    }

    /// Gets the number of color stops specified in the given gradient pattern.
    fn get_color_stop_count(&self) -> isize {
        unsafe {
            let count : *mut c_int = transmute(Box::new(0i32));
            let result = ffi::cairo_pattern_get_color_stop_count(self.get_ptr(), count);

            result.ensure_valid(); // Not sure if these are needed
            count as isize
        }
    }

    /// Gets the color and offset information at the given index for a gradient pattern.
    /// Values of index range from 0 to n-1 where n is the number returned by
    /// Pattern::get_color_stop_count().
    fn get_color_stop_rgba(&self, index: isize) -> (f64, f64, f64, f64, f64) {
        unsafe {
            let offset: *mut c_double = transmute(Box::new(0.0f64));
            let red   : *mut c_double = transmute(Box::new(0.0f64));
            let green : *mut c_double = transmute(Box::new(0.0f64));
            let blue  : *mut c_double = transmute(Box::new(0.0f64));
            let alpha : *mut c_double = transmute(Box::new(0.0f64));

            ffi::cairo_pattern_get_color_stop_rgba(self.get_ptr(), index as c_int, offset, red, green, blue, alpha).ensure_valid();
            (*offset, *red, *green, *blue, *alpha)
        }
    }
}

pattern_type!(LinearGradient);

impl LinearGradient {
    /// Create a new linear gradient Pattern object along the line defined by
    /// (x0, y0) and (x1, y1). Before using the gradient pattern, a number of color
    /// stops should be defined using Pattern::add_color_stop_rgb() or
    /// Pattern::add_color_stop_rgba().
    /// 
    /// Note: The coordinates here are in pattern space. For a new pattern, pattern
    /// space is identical to user space, but the relationship between the spaces can
    /// be changed with Pattern::set_matrix().
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> LinearGradient {
        LinearGradient::wrap(unsafe {
            ffi::cairo_pattern_create_linear(x0, y0, x1, y1)
        })
    }

    /// Gets the gradient endpoints for a linear gradient.
    pub fn get_linear_points(&self) -> (f64, f64, f64, f64) {
        unsafe {
            let x0 : *mut c_double = transmute(Box::new(0.0f64));
            let y0 : *mut c_double = transmute(Box::new(0.0f64));
            let x1 : *mut c_double = transmute(Box::new(0.0f64));
            let y1 : *mut c_double = transmute(Box::new(0.0f64));

            ffi::cairo_pattern_get_linear_points(self.pointer, x0, y0, x1, y1).ensure_valid();
            (*x0, *y0, *x1, *y1)
        }
    }
}

impl Gradient for LinearGradient{}


pattern_type!(RadialGradient);

impl RadialGradient {
    /// Creates a new radial gradient Pattern between the two circles
    /// defined by (cx0, cy0, radius0) and (cx1, cy1, radius1). Before
    /// using the gradient pattern, a number of color stops should be
    /// defined using Pattern::add_color_stop_rgb() or Pattern::add_color_stop_rgba().
    /// 
    /// Note: The coordinates here are in pattern space. For a new pattern, pattern
    /// space is identical to user space, but the relationship between the spaces can
    /// be changed with Pattern::set_matrix().
    pub fn new(x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> RadialGradient {
        RadialGradient::wrap(unsafe{
            ffi::cairo_pattern_create_radial(x0, y0, r0, x1, y1, r1)
        })
    }

    /// Gets the gradient endpoint circles for a radial gradient, each specified as a center
    /// coordinate and a radius.
    pub fn get_radial_circles(&self) -> (f64,f64,f64,f64) {
        unsafe{
            let x0 : *mut c_double = transmute(Box::new(0.0f64));
            let y0 : *mut c_double = transmute(Box::new(0.0f64));
            let r0 : *mut c_double = transmute(Box::new(0.0f64));
            let x1 : *mut c_double = transmute(Box::new(0.0f64));
            let y1 : *mut c_double = transmute(Box::new(0.0f64));
            let r1 : *mut c_double = transmute(Box::new(0.0f64));

            ffi::cairo_pattern_get_radial_circles(self.pointer, x0, y0, r0, x1, y1, r1).ensure_valid();
            (*x0, *y0, *x1, *y1)
        }
    }
}


pattern_type!(SurfacePattern);

impl SurfacePattern {
    //pub fn cairo_pattern_create_for_surface(surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;

    //pub fn cairo_pattern_get_surface(pattern: *mut cairo_pattern_t, surface: **mut cairo_surface_t) -> Status;
}

#[cfg(cairo_1_12)]
#[derive(Clone, PartialEq, PartialOrd, Copy)]
pub enum MeshCorner {
    MeshCorner0,
    MeshCorner1,
    MeshCorner2,
    MeshCorner3
}

#[cfg(cairo_1_12)]
pattern_type!(Mesh);

#[cfg(cairo_1_12)]
impl Mesh {
    /// Create a new mesh pattern.
    /// 
    /// Mesh patterns are tensor-product patch meshes (type 7 shadings in PDF). Mesh
    /// patterns may also be used to create other types of shadings that are special
    /// cases of tensor-product patch meshes such as Coons patch meshes (type 6 shading
    /// in PDF) and Gouraud-shaded triangle meshes (type 4 and 5 shadings in PDF).
    /// 
    /// Mesh patterns consist of one or more tensor-product patches, which should be
    /// defined before using the mesh pattern. Using a mesh pattern with a partially
    /// defined patch as source or mask will put the context in an error status with
    /// a status of Status::InvalidMeshConstruction.
    /// 
    /// A tensor-product patch is defined by 4 Bézier curves (side 0, 1, 2, 3) and by
    /// 4 additional control points (P0, P1, P2, P3) that provide further control over
    /// the patch and complete the definition of the tensor-product patch. The corner
    /// C0 is the first point of the patch.
    /// 
    /// Degenerate sides are permitted so straight lines may be used. A zero length
    /// line on one side may be used to create 3 sided patches.
    /// 
    ///       C1     Side 1       C2
    ///        +---------------+
    ///        |               |
    ///        |  P1       P2  |
    ///        |               |
    /// Side 0 |               | Side 2
    ///        |               |
    ///        |               |
    ///        |  P0       P3  |
    ///        |               |
    ///        +---------------+
    ///     C0     Side 3        C3
    ///
    /// Each patch is constructed by first calling Mesh::begin_patch(),
    /// then Mesh::move_to() to specify the first point in the patch (C0).
    /// Then the sides are specified with calls to Mesh::curve_to() and
    /// cairo_mesh_pattern_line_to().
    /// 
    /// The four additional control points (P0, P1, P2, P3) in a patch can
    /// be specified with Mesh::set_control_point().
    /// 
    /// At each corner of the patch (C0, C1, C2, C3) a color may be specified with
    /// Mesh::set_corner_color_rgb() or Mesh::set_corner_color_rgba(). Any corner
    /// whose color is not explicitly specified defaults to transparent black.
    /// 
    /// A Coons patch is a special case of the tensor-product patch where the control
    /// points are implicitly defined by the sides of the patch. The default value for
    /// any control point not specified is the implicit value for a Coons patch, i.e.
    /// if no control points are specified the patch is a Coons patch.
    /// 
    /// A triangle is a special case of the tensor-product patch where the control points
    /// are implicitly defined by the sides of the patch, all the sides are lines and one
    /// of them has length 0, i.e. if the patch is specified using just 3 lines, it is a
    /// triangle. If the corners connected by the 0-length side have the same color, the
    /// patch is a Gouraud-shaded triangle.
    /// 
    /// Patches may be oriented differently to the above diagram. For example the first
    /// point could be at the top left. The diagram only shows the relationship between
    /// the sides, corners and control points. Regardless of where the first point is
    /// located, when specifying colors, corner 0 will always be the first point, corner
    /// 1 the point between side 0 and side 1 etc.
    /// 
    /// Calling Mesh::end_patch() completes the current patch. If less than 4 sides have
    /// been defined, the first missing side is defined as a line from the current point
    /// to the first point of the patch (C0) and the other sides are degenerate lines from
    /// C0 to C0. The corners between the added sides will all be coincident with C0 of
    /// the patch and their color will be set to be the same as the color of C0.
    /// 
    /// Additional patches may be added with additional calls to
    /// Mesh::begin_patch()/Mesh::end_patch().
    /// 
    /// ```ignore
    /// let mut pattern = Mesh::new();
    /// /* Add a Coons patch */
    /// pattern.begin_patch();
    /// pattern.move_to(0, 0);
    /// pattern.curve_to(30, -30,  60,  30, 100, 0);
    /// pattern.curve_to(60,  30, 130,  60, 100, 100);
    /// pattern.curve_to(60,  70,  30, 130,   0, 100);
    /// pattern.curve_to(30,  70, -30,  30,   0, 0);
    /// pattern.set_corner_color_rgb(0, 1, 0, 0);
    /// pattern.set_corner_color_rgb(1, 0, 1, 0);
    /// pattern.set_corner_color_rgb(2, 0, 0, 1);
    /// pattern.set_corner_color_rgb(3, 1, 1, 0);
    /// pattern.end_patch();
    /// 
    /// /* Add a Gouraud-shaded triangle */
    /// pattern.begin_patch()
    /// pattern.move_to(100, 100);
    /// pattern.line_to(130, 130);
    /// pattern.line_to(130,  70);
    /// pattern.set_corner_color_rgb(0, 1, 0, 0);
    /// pattern.set_corner_color_rgb(1, 0, 1, 0);
    /// pattern.set_corner_color_rgb(2, 0, 0, 1);
    /// pattern.end_patch();
    /// ```
    /// 
    /// When two patches overlap, the last one that has been added is drawn over the first
    /// one.
    /// 
    /// When a patch folds over itself, points are sorted depending on their parameter
    /// coordinates inside the patch. The v coordinate ranges from 0 to 1 when moving from
    /// side 3 to side 1; the u coordinate ranges from 0 to 1 when going from side 0 to side
    /// 
    /// Points with higher v coordinate hide points with lower v coordinate. When two points
    /// have the same v coordinate, the one with higher u coordinate is above. This means
    /// that points nearer to side 1 are above points nearer to side 3; when this is not
    /// sufficient to decide which point is above (for example when both points belong to
    /// side 1 or side 3) points nearer to side 2 are above points nearer to side 0.
    /// 
    /// For a complete definition of tensor-product patches, see the PDF specification (ISO32000),
    /// which describes the parametrization in detail.
    /// 
    /// Note: The coordinates are always in pattern space. For a new pattern, pattern space is
    /// identical to user space, but the relationship between the spaces can be changed with
    /// Pattern::set_matrix().
    pub fn new() -> Mesh {
        Mesh::wrap(unsafe {
            ffi::cairo_pattern_create_mesh()
        })
    }

    /// Begin a patch in a mesh pattern.
    /// 
    /// After calling this function, the patch shape should be defined with Mesh::move_to(),
    /// Mesh::line_to() and Mesh::curve_to().
    /// 
    /// After defining the patch, Mesh::end_patch() must be called before using pattern as
    /// a source or mask.
    /// 
    /// Note: If pattern is not a mesh pattern then pattern will be put into an error status
    /// with a status of Status::PatternTypeMismatch. If pattern already has a current patch,
    /// it will be put into an error status with a status of Status::InvalidMeshConstruction.
    pub fn begin_patch(&self) {
        unsafe {
            ffi::cairo_mesh_pattern_begin_patch(self.pointer)
        }
        self.ensure_status();
    }

    /// Indicates the end of the current patch in a mesh pattern.
    /// 
    /// If the current patch has less than 4 sides, it is closed with a straight line from the
    /// current point to the first point of the patch as if Mesh::line_to() was used.
    /// 
    /// Note: If pattern is not a mesh pattern then pattern will be put into an error status
    /// with a status of Status::PatternTypeMismatch. If pattern has no current patch or the
    /// current patch has no current point, pattern will be put into an error status with a
    /// status of Status::InvalidMeshConstruction.
    pub fn end_patch(&self) {
        unsafe {
            ffi::cairo_mesh_pattern_end_patch(self.pointer)
        }
        self.ensure_status();
    }

    /// Define the first point of the current patch in a mesh pattern.
    /// 
    /// After this call the current point will be (x , y ).
    /// 
    /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with
    /// a status of Status::PatternTypeMismatch. If pattern has no current patch or the current
    /// patch already has at least one side, pattern will be put into an error status with a status
    /// of Status::InvalidMeshConstruction.
    pub fn move_to(&self, x: f64, y: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_move_to(self.pointer, x, y)
        }
        self.ensure_status();
    }

    /// Adds a line to the current patch from the current point to position (x , y ) in
    /// pattern-space coordinates.
    /// 
    /// If there is no current point before the call to cairo_mesh_pattern_line_to() this function
    /// will behave as Mesh::move_to(pattern , x , y ).
    /// 
    /// After this call the current point will be (x , y ).
    /// 
    /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with
    /// a status of Status::PatternTypeMismatch. If pattern has no current patch or the current
    /// patch already has 4 sides, pattern will be put into an error status with a status of
    /// Status::InvalidMeshConstruction.
    pub fn line_to(&self, x: f64, y: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_line_to(self.pointer, x, y)
        }
        self.ensure_status();
    }

    /// Adds a cubic Bézier spline to the current patch from the current point to position
    /// (x3 , y3 ) in pattern-space coordinates, using (x1 , y1 ) and (x2 , y2 ) as the control
    /// points.
    /// 
    /// If the current patch has no current point before the call to Mesh::curve_to(), this
    /// function will behave as if preceded by a call to Mesh::move_to(pattern , x1 , y1 ).
    /// 
    /// After this call the current point will be (x3 , y3 ).
    /// 
    /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with
    /// a status of Status::PatternTypeMismatch. If pattern has no current patch or the current
    /// patch already has 4 sides, pattern will be put into an error status with a status of
    /// Status::InvalidMeshConstruction.
    pub fn curve_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_curve_to(self.pointer, x1, y1, x2, y2, x3, y3)
        }
        self.ensure_status();
    }

    /// Set an internal control point of the current patch.
    /// 
    /// Valid values for point_num are from 0 to 3 and identify the control points as explained in
    /// Mesh::new().
    /// 
    /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with a
    /// status of Status::PatternTypeMismatch. If point_num is not valid, pattern will be put into
    /// an error status with a status of Status::InvalidIndex. If pattern has no current patch,
    /// pattern will be put into an error status with a status of Status::InvalidMeshConstruction.
    pub fn set_control_point(&self, corner: MeshCorner, x: f64, y: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_set_control_point(self.pointer, corner as c_uint, x, y)
        }
        self.ensure_status();
    }

    /// Gets the control point point_num of patch patch_num for a mesh pattern.
    /// 
    /// patch_num can range from 0 to n-1 where n is the number returned by Mesh::get_patch_count().
    /// 
    /// Valid values for point_num are from 0 to 3 and identify the control points as explained
    /// in Mesh::new().
    pub fn get_control_point(&self, patch_num: usize, corner: MeshCorner) -> (f64, f64) {
        let mut x: c_double = 0.0;
        let mut y: c_double = 0.0;

        let status = unsafe {
            ffi::cairo_mesh_pattern_get_control_point(self.pointer, patch_num as c_uint, corner as c_uint, &mut x, &mut y)
        };
        status.ensure_valid();
        (x, y)
    }

    /// Sets the color of a corner of the current patch in a mesh pattern.
    /// 
    /// The color is specified in the same way as in Context::set_source_rgb().
    /// 
    /// Valid values for corner_num are from 0 to 3 and identify the corners as explained in
    /// Mesh::new().
    /// 
    /// Note: If pattern is not a mesh pattern then pattern will be put into an error status
    /// with a status of Status::PatternTypeMismatch. If corner_num is not valid, pattern will
    /// be put into an error status with a status of Status::InvalidIndex. If pattern has no
    /// current patch, pattern will be put into an error status with a status of
    /// Status::InvalidMeshConstruction.
    pub fn set_corner_color_rgb(&self, corner: MeshCorner, red: f64, green: f64, blue: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_set_corner_color_rgb(self.pointer, corner as c_uint, red, green, blue)
        }
        self.ensure_status();
    }

    /// Sets the color of a corner of the current patch in a mesh pattern.
    /// 
    /// The color is specified in the same way as in Context::set_source_rgba().
    /// 
    /// Valid values for corner_num are from 0 to 3 and identify the corners as explained
    /// in Mesh::new().
    /// 
    /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with a
    /// status of Status::PatternTypeMismatch. If corner_num is not valid, pattern will be put into
    /// an error status with a status of Status::InvalidIndex. If pattern has no current patch,
    /// pattern will be put into an error status with a status of Status::InvalidMeshConstruction.
    pub fn set_corner_color_rgba(&self, corner: MeshCorner, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_set_corner_color_rgba(self.pointer, corner as c_uint, red, green, blue, alpha)
        }
        self.ensure_status();
    }

    /// Gets the color information in corner corner_num of patch patch_num for a mesh pattern.
    /// 
    /// patch_num can range from 0 to n-1 where n is the number returned by Mesh::get_patch_count().
    /// 
    /// Valid values for corner_num are from 0 to 3 and identify the corners as explained in
    /// Mesh::new().
    pub fn get_corner_color_rgba(&self, patch_num: usize, corner: MeshCorner) -> (f64, f64, f64, f64) {
        let mut red: c_double = 0.0;
        let mut green: c_double = 0.0;
        let mut blue: c_double = 0.0;
        let mut alpha: c_double = 0.0;

        let status = unsafe {
            ffi::cairo_mesh_pattern_get_corner_color_rgba(self.pointer, patch_num as c_uint, corner as c_uint, &mut red, &mut green, &mut blue, &mut alpha)
        };
        status.ensure_valid();
        (red, green, blue, alpha)
    }

    /// Gets the number of patches specified in the given mesh pattern.
    /// 
    /// The number only includes patches which have been finished by calling Mesh::end_patch().
    /// For example it will be 0 during the definition of the first patch.
    pub fn get_patch_count(&self) -> usize {
        let mut count: c_uint = 0;
        unsafe {
            ffi::cairo_mesh_pattern_get_patch_count(self.pointer, &mut count).ensure_valid();
        }
        count as usize
    }

    /// Gets path defining the patch patch_num for a mesh pattern.
    /// 
    /// patch_num can range from 0 to n-1 where n is the number returned by Mesh::get_patch_count().
    pub fn get_path(&self, patch_num: usize) -> Path {
        let path: Path = Path::wrap(unsafe {
            ffi::cairo_mesh_pattern_get_path(self.pointer, patch_num as c_uint)
        });
        path.ensure_status();
        path
    }
}
