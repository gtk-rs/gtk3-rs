// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::{c_double, c_int, c_uint};
use std::ptr;
use std::fmt;
use ::enums::{
    Extend,
    Filter,
    PatternType,
    Status,
};
use ::enums::MeshCorner;
use ffi;
use ffi::{
    cairo_pattern_t,
    cairo_surface_t,
};
use ::{
    Path,
    Matrix,
    MatrixTrait,
    Surface,
};

// Quite some changes from the C api but all suggested by the cairo devs.
// See http://cairographics.org/manual/bindings-patterns.html for more info
#[derive(Debug, Clone)]
pub enum Pattern {
    SolidPattern(SolidPattern),
    SurfacePattern(SurfacePattern),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Mesh(Mesh),
}

impl Pattern {
    user_data_methods! {
        Pattern::as_ptr,
        ffi::cairo_pattern_get_user_data,
        ffi::cairo_pattern_set_user_data,
    }
}

impl PatternTrait for Pattern {
    type PatternType = Pattern;

    fn as_ptr(&self) -> *mut cairo_pattern_t {
        match *self {
            Pattern::SolidPattern(ref solid) => solid.as_ptr(),
            Pattern::SurfacePattern(ref surface) => surface.as_ptr(),
            Pattern::LinearGradient(ref linear) => linear.as_ptr(),
            Pattern::RadialGradient(ref radial) => radial.as_ptr(),
            Pattern::Mesh(ref mesh) => mesh.as_ptr(),
        }
    }

    unsafe fn from_raw_none(pointer: *mut cairo_pattern_t) -> Pattern {
        ffi::cairo_pattern_reference(pointer);
        Self::from_raw_full(pointer)
    }

    unsafe fn from_raw_full(pointer: *mut cairo_pattern_t) -> Pattern {
        let pattern_type = PatternType::from(ffi::cairo_pattern_get_type(pointer));

        match pattern_type {
            PatternType::Solid           => Pattern::SolidPattern(SolidPattern::from_raw_full(pointer)),
            PatternType::Surface         => Pattern::SurfacePattern(SurfacePattern::from_raw_full(pointer)),
            PatternType::LinearGradient  => Pattern::LinearGradient(LinearGradient::from_raw_full(pointer)),
            PatternType::RadialGradient  => Pattern::RadialGradient(RadialGradient::from_raw_full(pointer)),
            PatternType::Mesh         => Pattern::Mesh(Mesh::from_raw_full(pointer)),
            PatternType::RasterSource => panic!("Not implemented"),
            PatternType::__Unknown(x) => panic!("Unknown value {}", x),
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pattern::{}", match *self {
            Pattern::SolidPattern(_) => "SolidPattern",
            Pattern::SurfacePattern(_) => "SurfacePattern",
            Pattern::LinearGradient(_) => "LinearGradient",
            Pattern::RadialGradient(_) => "RadialGradient",
            Pattern::Mesh(_) => "Mesh",
        })
    }
}

pub trait PatternTrait {
    type PatternType;

    fn as_ptr(&self) -> *mut cairo_pattern_t;

    fn ensure_status(&self) {
        self.status().ensure_valid();
    }

    fn status(&self) -> Status {
        unsafe {
            Status::from(ffi::cairo_pattern_status(self.as_ptr()))
        }
    }

    fn get_reference_count(&self) -> isize {
        unsafe {
            ffi::cairo_pattern_get_reference_count(self.as_ptr()) as isize
        }
    }

    fn set_extend(&self, extend: Extend) {
        unsafe {
            ffi::cairo_pattern_set_extend(self.as_ptr(), extend.into())
        }
    }

    fn get_extend(&self) -> Extend {
        unsafe {
            Extend::from(ffi::cairo_pattern_get_extend(self.as_ptr()))
        }
    }

    fn set_filter(&self, filter: Filter) {
        unsafe {
            ffi::cairo_pattern_set_filter(self.as_ptr(), filter.into())
        }
    }

    fn get_filter(&self) -> Filter {
        unsafe {
            Filter::from(ffi::cairo_pattern_get_filter(self.as_ptr()))
        }
    }

    fn set_matrix(&self, matrix: Matrix) {
        unsafe {
            ffi::cairo_pattern_set_matrix (self.as_ptr(), &matrix)
        }
    }

    fn get_matrix(&self) -> Matrix {
        let mut matrix = Matrix::null();
        unsafe {
            ffi::cairo_pattern_get_matrix(self.as_ptr(), &mut matrix);
        }
        matrix
    }

    unsafe fn from_raw_full(pointer: *mut cairo_pattern_t) -> Self::PatternType;

    unsafe fn from_raw_none(pointer: *mut cairo_pattern_t) -> Self::PatternType {
        ffi::cairo_pattern_reference(pointer);
        Self::from_raw_full(pointer)
    }
}

macro_rules! pattern_type(
    //Signals without arguments
    ($pattern_type:ident) => (

        #[derive(Debug)]
        pub struct $pattern_type {
            pointer: *mut cairo_pattern_t,
        }

        impl PatternTrait for $pattern_type {
            type PatternType = $pattern_type;

            unsafe fn from_raw_full(pointer: *mut cairo_pattern_t) -> Self::PatternType {
                $pattern_type {
                    pointer: pointer,
                }
            }

            fn as_ptr(&self) -> *mut cairo_pattern_t {
                self.pointer
            }
        }

        impl $pattern_type {
            user_data_methods! {
                $pattern_type::as_ptr,
                ffi::cairo_pattern_get_user_data,
                ffi::cairo_pattern_set_user_data,
            }
        }

        impl Clone for $pattern_type {
            fn clone(&self) -> Self {
                $pattern_type {
                    pointer: unsafe {
                        ffi::cairo_pattern_reference(self.pointer)
                    },
                }
            }
        }

        impl Drop for $pattern_type {
            fn drop(&mut self){
                unsafe {
                    ffi::cairo_pattern_destroy(self.pointer)
                }
            }
        }

        impl fmt::Display for $pattern_type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, stringify!($pattern_type))
            }
        }
    );
);

pattern_type!(SolidPattern);

impl SolidPattern {
    pub fn from_rgb(red: f64, green: f64, blue: f64) -> SolidPattern {
        unsafe {
            SolidPattern::from_raw_full(
                ffi::cairo_pattern_create_rgb(red, green, blue)
            )
        }
    }

    pub fn from_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> SolidPattern {
        unsafe {
            SolidPattern::from_raw_full(
                ffi::cairo_pattern_create_rgba(red, green, blue, alpha)
                )
        }
    }

    pub fn get_rgba(&self) -> (f64, f64, f64, f64) {
        unsafe {
            let mut red   = 0.0;
            let mut green = 0.0;
            let mut blue  = 0.0;
            let mut alpha = 0.0;

            Status::from(ffi::cairo_pattern_get_rgba(self.pointer,
                                                     &mut red,
                                                     &mut green,
                                                     &mut blue,
                                                     &mut alpha)).ensure_valid();

            (red, green, blue, alpha)
        }
    }
}


pub trait Gradient : PatternTrait {
    fn add_color_stop_rgb(&self, offset: f64, red: f64, green: f64, blue: f64) {
        unsafe {
            ffi::cairo_pattern_add_color_stop_rgb(self.as_ptr(), offset, red, green, blue)
        }
    }

    fn add_color_stop_rgba(&self, offset: f64, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe {
            ffi::cairo_pattern_add_color_stop_rgba(self.as_ptr(), offset, red, green, blue, alpha)
        }
    }

    fn get_color_stop_count(&self) -> isize {
        unsafe {
            let mut count = 0;
            let result = ffi::cairo_pattern_get_color_stop_count(self.as_ptr(), &mut count);

            Status::from(result).ensure_valid(); // Not sure if these are needed
            count as isize
        }
    }

    fn get_color_stop_rgba(&self, index: isize) -> (f64, f64, f64, f64, f64) {
        unsafe {
            let mut offset = 0.0;
            let mut red    = 0.0;
            let mut green  = 0.0;
            let mut blue   = 0.0;
            let mut alpha  = 0.0;

            Status::from(ffi::cairo_pattern_get_color_stop_rgba(self.as_ptr(),
                                                                index as c_int,
                                                                &mut offset,
                                                                &mut red,
                                                                &mut green,
                                                                &mut blue,
                                                                &mut alpha)).ensure_valid();
            (offset, red, green, blue, alpha)
        }
    }
}

pattern_type!(LinearGradient);

impl LinearGradient {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> LinearGradient {
        unsafe {
            LinearGradient::from_raw_full(
                ffi::cairo_pattern_create_linear(x0, y0, x1, y1)
            )
        }
    }

    pub fn get_linear_points(&self) -> (f64, f64, f64, f64) {
        unsafe {
            let mut x0 = 0.0;
            let mut y0 = 0.0;
            let mut x1 = 0.0;
            let mut y1 = 0.0;

            Status::from(ffi::cairo_pattern_get_linear_points(self.pointer,
                                                              &mut x0,
                                                              &mut y0,
                                                              &mut x1,
                                                              &mut y1)).ensure_valid();
            (x0, y0, x1, y1)
        }
    }
}

impl Gradient for LinearGradient{}


pattern_type!(RadialGradient);

impl RadialGradient {
    pub fn new(x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> RadialGradient {
        unsafe {
            RadialGradient::from_raw_full(
                ffi::cairo_pattern_create_radial(x0, y0, r0, x1, y1, r1)
            )
        }
    }

    pub fn get_radial_circles(&self) -> (f64, f64, f64, f64, f64, f64) {
        unsafe {
            let mut x0 = 0.0;
            let mut y0 = 0.0;
            let mut r0 = 0.0;
            let mut x1 = 0.0;
            let mut y1 = 0.0;
            let mut r1 = 0.0;

            Status::from(ffi::cairo_pattern_get_radial_circles(self.pointer,
                                                               &mut x0,
                                                               &mut y0,
                                                               &mut r0,
                                                               &mut x1,
                                                               &mut y1,
                                                               &mut r1)).ensure_valid();
            (x0, y0, r0, x1, y1, r1)
        }
    }
}

impl Gradient for RadialGradient{}


pattern_type!(SurfacePattern);

impl SurfacePattern {
    pub fn create<T: AsRef<Surface>>(surface: &T) -> SurfacePattern {
        unsafe {
            SurfacePattern::from_raw_full(
                ffi::cairo_pattern_create_for_surface(surface.as_ref().to_raw_none())
            )
        }
    }

    pub fn get_surface(&self) -> Surface {
        unsafe {
            let mut surface_ptr: *mut cairo_surface_t = ptr::null_mut();
            Status::from(ffi::cairo_pattern_get_surface(self.pointer,
                                                        &mut surface_ptr)).ensure_valid();
            Surface::from_raw_none(surface_ptr)
        }
    }
}

pattern_type!(Mesh);

impl Mesh {
    pub fn new() -> Mesh {
        unsafe {
            Mesh::from_raw_full(
                ffi::cairo_pattern_create_mesh()
            )
        }
    }

    pub fn begin_patch(&self) {
        unsafe {
            ffi::cairo_mesh_pattern_begin_patch(self.pointer)
        }
        self.ensure_status();
    }

    pub fn end_patch(&self) {
        unsafe {
            ffi::cairo_mesh_pattern_end_patch(self.pointer)
        }
        self.ensure_status();
    }

    pub fn move_to(&self, x: f64, y: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_move_to(self.pointer, x, y)
        }
        self.ensure_status();
    }

    pub fn line_to(&self, x: f64, y: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_line_to(self.pointer, x, y)
        }
        self.ensure_status();
    }

    pub fn curve_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_curve_to(self.pointer, x1, y1, x2, y2, x3, y3)
        }
        self.ensure_status();
    }

    pub fn set_control_point(&self, corner: MeshCorner, x: f64, y: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_set_control_point(self.pointer, corner.into(), x, y)
        }
        self.ensure_status();
    }

    pub fn get_control_point(&self, patch_num: usize, corner: MeshCorner) -> (f64, f64) {
        let mut x: c_double = 0.0;
        let mut y: c_double = 0.0;

        let status = unsafe {
            ffi::cairo_mesh_pattern_get_control_point(self.pointer,
                                                      patch_num as c_uint,
                                                      corner.into(),
                                                      &mut x,
                                                      &mut y)
        };
        Status::from(status).ensure_valid();
        (x, y)
    }

    pub fn set_corner_color_rgb(&self, corner: MeshCorner, red: f64, green: f64, blue: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_set_corner_color_rgb(self.pointer,
                                                         corner.into(),
                                                         red,
                                                         green,
                                                         blue)
        }
        self.ensure_status();
    }

    pub fn set_corner_color_rgba(&self, corner: MeshCorner, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_set_corner_color_rgba(self.pointer,
                                                          corner.into(),
                                                          red,
                                                          green,
                                                          blue,
                                                          alpha)
        }
        self.ensure_status();
    }

    pub fn get_corner_color_rgba(&self, patch_num: usize, corner: MeshCorner) -> (f64, f64, f64, f64) {
        let mut red: c_double = 0.0;
        let mut green: c_double = 0.0;
        let mut blue: c_double = 0.0;
        let mut alpha: c_double = 0.0;

        let status = unsafe {
            ffi::cairo_mesh_pattern_get_corner_color_rgba(self.pointer,
                                                          patch_num as c_uint,
                                                          corner.into(),
                                                          &mut red,
                                                          &mut green,
                                                          &mut blue,
                                                          &mut alpha)
        };
        Status::from(status).ensure_valid();
        (red, green, blue, alpha)
    }

    pub fn get_patch_count(&self) -> usize {
        let mut count: c_uint = 0;
        unsafe {
            Status::from(ffi::cairo_mesh_pattern_get_patch_count(self.pointer,
                                                                 &mut count)).ensure_valid();
        }
        count as usize
    }

    pub fn get_path(&self, patch_num: usize) -> Path {
        let path: Path = unsafe {
            Path::from_raw_full(
                ffi::cairo_mesh_pattern_get_path(self.pointer, patch_num as c_uint)
            )
        };
        path.ensure_status();
        path
    }
}
