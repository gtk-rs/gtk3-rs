// Take a look at the license at the top of the repository in the LICENSE file.

use crate::enums::MeshCorner;
use crate::enums::{Extend, Filter, PatternType};
use crate::error::Error;
use crate::ffi::{cairo_pattern_t, cairo_surface_t};
use crate::utils::status_to_result;
use crate::{Matrix, Path, Surface};
use libc::{c_double, c_int, c_uint};
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;
use std::ptr;

// See https://cairographics.org/manual/bindings-patterns.html for more info
#[derive(Debug)]
pub struct Pattern {
    pointer: *mut cairo_pattern_t,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pattern")
    }
}

impl Pattern {
    user_data_methods! {
        ffi::cairo_pattern_get_user_data,
        ffi::cairo_pattern_set_user_data,
    }

    pub fn to_raw_none(&self) -> *mut cairo_pattern_t {
        self.pointer
    }

    pub unsafe fn from_raw_none(pointer: *mut cairo_pattern_t) -> Pattern {
        ffi::cairo_pattern_reference(pointer);
        Self::from_raw_full(pointer)
    }

    pub unsafe fn from_raw_full(pointer: *mut cairo_pattern_t) -> Pattern {
        Pattern { pointer }
    }

    pub fn get_type(&self) -> PatternType {
        unsafe { ffi::cairo_pattern_get_type(self.pointer).into() }
    }

    pub fn get_reference_count(&self) -> isize {
        unsafe { ffi::cairo_pattern_get_reference_count(self.pointer) as isize }
    }

    pub fn set_extend(&self, extend: Extend) {
        unsafe { ffi::cairo_pattern_set_extend(self.pointer, extend.into()) }
    }

    pub fn get_extend(&self) -> Extend {
        unsafe { Extend::from(ffi::cairo_pattern_get_extend(self.pointer)) }
    }

    pub fn set_filter(&self, filter: Filter) {
        unsafe { ffi::cairo_pattern_set_filter(self.pointer, filter.into()) }
    }

    pub fn get_filter(&self) -> Filter {
        unsafe { Filter::from(ffi::cairo_pattern_get_filter(self.pointer)) }
    }

    pub fn set_matrix(&self, matrix: Matrix) {
        unsafe { ffi::cairo_pattern_set_matrix(self.pointer, matrix.ptr()) }
    }

    pub fn get_matrix(&self) -> Matrix {
        let mut matrix = Matrix::null();
        unsafe {
            ffi::cairo_pattern_get_matrix(self.pointer, matrix.mut_ptr());
        }
        matrix
    }

    fn status(&self) -> Result<(), Error> {
        let status = unsafe { ffi::cairo_pattern_status(self.pointer) };
        status_to_result(status)
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        Pattern {
            pointer: unsafe { ffi::cairo_pattern_reference(self.pointer) },
        }
    }
}

impl Drop for Pattern {
    fn drop(&mut self) {
        unsafe { ffi::cairo_pattern_destroy(self.pointer) }
    }
}

macro_rules! convert {
    ($source: ident => $dest: ident = $( $variant: ident )|+ $( ($intermediate: ident) )*) => {
        impl TryFrom<$source> for $dest {
            type Error = $source;

            fn try_from(pattern: $source) -> Result<Self, $source> {
                if $( pattern.get_type() == PatternType::$variant )||+ {
                    $(
                        let pattern = $intermediate(pattern);
                    )*
                    Ok($dest(pattern))
                }
                else {
                    Err(pattern)
                }
            }
        }
    };
}

macro_rules! pattern_type(
    //Signals without arguments
    ($pattern_type:ident $( = $variant: ident)*) => (

        #[derive(Debug, Clone)]
        pub struct $pattern_type(Pattern);

        impl Deref for $pattern_type {
            type Target = Pattern;

            fn deref(&self) -> &Pattern {
                &self.0
            }
        }

        impl fmt::Display for $pattern_type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, stringify!($pattern_type))
            }
        }

        $(
            convert!(Pattern => $pattern_type = $variant);
        )*
    );
);

pattern_type!(SolidPattern = Solid);

impl SolidPattern {
    pub fn from_rgb(red: f64, green: f64, blue: f64) -> SolidPattern {
        unsafe {
            SolidPattern(Pattern::from_raw_full(ffi::cairo_pattern_create_rgb(
                red, green, blue,
            )))
        }
    }

    pub fn from_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> SolidPattern {
        unsafe {
            SolidPattern(Pattern::from_raw_full(ffi::cairo_pattern_create_rgba(
                red, green, blue, alpha,
            )))
        }
    }

    pub fn get_rgba(&self) -> (f64, f64, f64, f64) {
        unsafe {
            let mut red = 0.0;
            let mut green = 0.0;
            let mut blue = 0.0;
            let mut alpha = 0.0;

            let status = ffi::cairo_pattern_get_rgba(
                self.pointer,
                &mut red,
                &mut green,
                &mut blue,
                &mut alpha,
            );
            status_to_result(status).expect("Failed to get_rgba");

            (red, green, blue, alpha)
        }
    }
}

pattern_type!(Gradient);
convert!(Pattern => Gradient = LinearGradient | RadialGradient);

impl Gradient {
    pub fn add_color_stop_rgb(&self, offset: f64, red: f64, green: f64, blue: f64) {
        unsafe { ffi::cairo_pattern_add_color_stop_rgb(self.pointer, offset, red, green, blue) }
    }

    pub fn add_color_stop_rgba(&self, offset: f64, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe {
            ffi::cairo_pattern_add_color_stop_rgba(self.pointer, offset, red, green, blue, alpha)
        }
    }

    pub fn get_color_stop_count(&self) -> isize {
        unsafe {
            let mut count = 0;
            let status = ffi::cairo_pattern_get_color_stop_count(self.pointer, &mut count);

            status_to_result(status).expect("Failed to get_color_stop_count");
            count as isize
        }
    }

    pub fn get_color_stop_rgba(&self, index: isize) -> (f64, f64, f64, f64, f64) {
        unsafe {
            let mut offset = 0.0;
            let mut red = 0.0;
            let mut green = 0.0;
            let mut blue = 0.0;
            let mut alpha = 0.0;

            let status = ffi::cairo_pattern_get_color_stop_rgba(
                self.pointer,
                index as c_int,
                &mut offset,
                &mut red,
                &mut green,
                &mut blue,
                &mut alpha,
            );
            status_to_result(status).expect("Failed to get_color_stop_rgba");
            (offset, red, green, blue, alpha)
        }
    }
}

macro_rules! gradient_type {
    ($gradient_type: ident) => {
        #[derive(Debug, Clone)]
        pub struct $gradient_type(Gradient);

        impl Deref for $gradient_type {
            type Target = Gradient;

            fn deref(&self) -> &Gradient {
                &self.0
            }
        }

        impl fmt::Display for $gradient_type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, stringify!($gradient_type))
            }
        }

        convert!(Pattern => $gradient_type = $gradient_type (Gradient));
        convert!(Gradient => $gradient_type = $gradient_type);
    }
}

gradient_type!(LinearGradient);

impl LinearGradient {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> LinearGradient {
        unsafe {
            LinearGradient(Gradient(Pattern::from_raw_full(
                ffi::cairo_pattern_create_linear(x0, y0, x1, y1),
            )))
        }
    }

    pub fn get_linear_points(&self) -> (f64, f64, f64, f64) {
        unsafe {
            let mut x0 = 0.0;
            let mut y0 = 0.0;
            let mut x1 = 0.0;
            let mut y1 = 0.0;

            let status = ffi::cairo_pattern_get_linear_points(
                self.pointer,
                &mut x0,
                &mut y0,
                &mut x1,
                &mut y1,
            );
            status_to_result(status).expect("Failed to get linear points");
            (x0, y0, x1, y1)
        }
    }
}

gradient_type!(RadialGradient);

impl RadialGradient {
    pub fn new(x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> RadialGradient {
        unsafe {
            RadialGradient(Gradient(Pattern::from_raw_full(
                ffi::cairo_pattern_create_radial(x0, y0, r0, x1, y1, r1),
            )))
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

            let status = ffi::cairo_pattern_get_radial_circles(
                self.pointer,
                &mut x0,
                &mut y0,
                &mut r0,
                &mut x1,
                &mut y1,
                &mut r1,
            );
            status_to_result(status).expect("Failed to get radial circles");
            (x0, y0, r0, x1, y1, r1)
        }
    }
}

pattern_type!(SurfacePattern = Surface);

impl SurfacePattern {
    pub fn create(surface: &Surface) -> SurfacePattern {
        unsafe {
            SurfacePattern(Pattern::from_raw_full(
                ffi::cairo_pattern_create_for_surface(surface.to_raw_none()),
            ))
        }
    }

    pub fn get_surface(&self) -> Surface {
        unsafe {
            let mut surface_ptr: *mut cairo_surface_t = ptr::null_mut();
            let status = ffi::cairo_pattern_get_surface(self.pointer, &mut surface_ptr);
            status_to_result(status).expect("Failed to get the surface");
            Surface::from_raw_none(surface_ptr)
        }
    }
}

pattern_type!(Mesh = Mesh);

impl Mesh {
    pub fn new() -> Mesh {
        unsafe { Mesh(Pattern::from_raw_full(ffi::cairo_pattern_create_mesh())) }
    }

    pub fn begin_patch(&self) {
        unsafe { ffi::cairo_mesh_pattern_begin_patch(self.pointer) }
        self.status().expect("Failed to begin_patch");
    }

    pub fn end_patch(&self) {
        unsafe { ffi::cairo_mesh_pattern_end_patch(self.pointer) }
        self.status().expect("Failed to end_patch");
    }

    pub fn move_to(&self, x: f64, y: f64) {
        unsafe { ffi::cairo_mesh_pattern_move_to(self.pointer, x, y) }
        self.status().expect("Failed to move to");
    }

    pub fn line_to(&self, x: f64, y: f64) {
        unsafe { ffi::cairo_mesh_pattern_line_to(self.pointer, x, y) }
        self.status().expect("Failed to line to");
    }

    pub fn curve_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        unsafe { ffi::cairo_mesh_pattern_curve_to(self.pointer, x1, y1, x2, y2, x3, y3) }
        self.status().expect("Failed to curve to");
    }

    pub fn set_control_point(&self, corner: MeshCorner, x: f64, y: f64) {
        unsafe { ffi::cairo_mesh_pattern_set_control_point(self.pointer, corner.into(), x, y) }
        self.status().expect("Failed to set control point");
    }

    pub fn get_control_point(&self, patch_num: usize, corner: MeshCorner) -> (f64, f64) {
        let mut x: c_double = 0.0;
        let mut y: c_double = 0.0;

        let status = unsafe {
            ffi::cairo_mesh_pattern_get_control_point(
                self.pointer,
                patch_num as c_uint,
                corner.into(),
                &mut x,
                &mut y,
            )
        };
        status_to_result(status).expect("Failed to get control point");
        (x, y)
    }

    pub fn set_corner_color_rgb(&self, corner: MeshCorner, red: f64, green: f64, blue: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_set_corner_color_rgb(
                self.pointer,
                corner.into(),
                red,
                green,
                blue,
            )
        }
        self.status().expect("Failed to set corner color rgb");
    }

    pub fn set_corner_color_rgba(
        &self,
        corner: MeshCorner,
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
    ) {
        unsafe {
            ffi::cairo_mesh_pattern_set_corner_color_rgba(
                self.pointer,
                corner.into(),
                red,
                green,
                blue,
                alpha,
            )
        }
        self.status().expect("Failed to set corner color rgba");
    }

    pub fn get_corner_color_rgba(
        &self,
        patch_num: usize,
        corner: MeshCorner,
    ) -> (f64, f64, f64, f64) {
        let mut red: c_double = 0.0;
        let mut green: c_double = 0.0;
        let mut blue: c_double = 0.0;
        let mut alpha: c_double = 0.0;

        let status = unsafe {
            ffi::cairo_mesh_pattern_get_corner_color_rgba(
                self.pointer,
                patch_num as c_uint,
                corner.into(),
                &mut red,
                &mut green,
                &mut blue,
                &mut alpha,
            )
        };
        status_to_result(status).expect("Failed to get mesh corner color");
        (red, green, blue, alpha)
    }

    pub fn get_patch_count(&self) -> usize {
        let mut count: c_uint = 0;
        unsafe {
            let status = ffi::cairo_mesh_pattern_get_patch_count(self.pointer, &mut count);
            status_to_result(status).expect("Failed to get mesh patch count")
        }
        count as usize
    }

    pub fn get_path(&self, patch_num: usize) -> Path {
        let path: Path = unsafe {
            Path::from_raw_full(ffi::cairo_mesh_pattern_get_path(
                self.pointer,
                patch_num as c_uint,
            ))
        };
        let status = unsafe {
            let ptr: *mut ffi::cairo_path_t = path.as_ptr();
            (*ptr).status
        };
        status_to_result(status).expect("Failed to get the mesh path");
        path
    }
}

impl Default for Mesh {
    fn default() -> Mesh {
        Mesh::new()
    }
}

#[test]
fn try_from() {
    let linear = LinearGradient::new(0., 0., 1., 1.);
    let gradient = Gradient::clone(&linear);
    let pattern = Pattern::clone(&linear);
    assert!(Gradient::try_from(pattern.clone()).is_ok());
    assert!(LinearGradient::try_from(gradient).is_ok());
    assert!(LinearGradient::try_from(pattern).is_ok());
}
