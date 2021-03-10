// Take a look at the license at the top of the repository in the LICENSE file.

use crate::error::Error;
use crate::utils::status_to_result;
use libc::c_double;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub xx: c_double,
    pub yx: c_double,

    pub xy: c_double,
    pub yy: c_double,

    pub x0: c_double,
    pub y0: c_double,
}

impl Default for Matrix {
    fn default() -> Matrix {
        Matrix::identity()
    }
}

impl Matrix {
    pub(crate) fn ptr(&self) -> *const ffi::Matrix {
        self as *const Matrix as _
    }

    pub(crate) fn mut_ptr(&mut self) -> *mut ffi::Matrix {
        self as *mut Matrix as _
    }

    pub(crate) fn null() -> Matrix {
        Matrix {
            xx: 0.0,
            yx: 0.0,
            xy: 0.0,
            yy: 0.0,
            x0: 0.0,
            y0: 0.0,
        }
    }

    pub fn identity() -> Matrix {
        Matrix {
            xx: 1.0,
            yx: 0.0,
            xy: 0.0,
            yy: 1.0,
            x0: 0.0,
            y0: 0.0,
        }
    }

    pub fn new(xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) -> Matrix {
        Matrix {
            xx,
            yx,
            xy,
            yy,
            x0,
            y0,
        }
    }

    #[doc(alias = "cairo_matrix_multiply")]
    pub fn multiply(left: &Matrix, right: &Matrix) -> Matrix {
        let mut matrix = Matrix::null();
        unsafe {
            ffi::cairo_matrix_multiply(matrix.mut_ptr(), left.ptr(), right.ptr());
        }
        matrix
    }

    #[doc(alias = "cairo_matrix_translate")]
    pub fn translate(&mut self, tx: f64, ty: f64) {
        unsafe { ffi::cairo_matrix_translate(self.mut_ptr(), tx, ty) }
    }

    #[doc(alias = "cairo_matrix_scale")]
    pub fn scale(&mut self, sx: f64, sy: f64) {
        unsafe { ffi::cairo_matrix_scale(self.mut_ptr(), sx, sy) }
    }

    #[doc(alias = "cairo_matrix_rotate")]
    pub fn rotate(&mut self, angle: f64) {
        unsafe { ffi::cairo_matrix_rotate(self.mut_ptr(), angle) }
    }

    #[doc(alias = "cairo_matrix_invert")]
    pub fn invert(&mut self) {
        let status = unsafe { ffi::cairo_matrix_invert(self.mut_ptr()) };
        status_to_result(status).expect("Failed to invert matrix");
    }

    #[doc(alias = "cairo_matrix_invert")]
    pub fn try_invert(&self) -> Result<Matrix, Error> {
        let mut matrix = *self;

        let status = unsafe { ffi::cairo_matrix_invert(matrix.mut_ptr()) };
        status_to_result(status)?;
        Ok(matrix)
    }

    #[doc(alias = "cairo_matrix_transform_distance")]
    pub fn transform_distance(&self, _dx: f64, _dy: f64) -> (f64, f64) {
        let mut dx = _dx;
        let mut dy = _dy;

        unsafe {
            ffi::cairo_matrix_transform_distance(self.ptr(), &mut dx, &mut dy);
        }
        (dx, dy)
    }

    #[doc(alias = "cairo_matrix_transform_point")]
    pub fn transform_point(&self, _x: f64, _y: f64) -> (f64, f64) {
        let mut x = _x;
        let mut y = _y;

        unsafe {
            ffi::cairo_matrix_transform_point(self.ptr(), &mut x, &mut y);
        }
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_layout_is_ffi_equivalent() {
        macro_rules! dummy_values {
            ($Matrix: ident) => {
                $Matrix {
                    xx: 1.0,
                    yx: 2.0,
                    xy: 3.0,
                    yy: 4.0,
                    x0: 5.0,
                    y0: 6.0,
                }
            };
        }
        use crate::ffi::Matrix as FfiMatrix;
        let transmuted: Matrix = unsafe { std::mem::transmute(dummy_values!(FfiMatrix)) };
        assert_eq!(transmuted, dummy_values!(Matrix));
    }

    #[test]
    fn invalid_matrix_does_not_invert() {
        let matrix = Matrix::null();
        assert!(matrix.try_invert().is_err());
    }

    #[test]
    #[should_panic]
    fn inverting_invalid_matrix_panics() {
        let mut matrix = Matrix::null();
        matrix.invert();
    }

    #[test]
    fn valid_matrix_try_invert() {
        let matrix = Matrix::identity();
        assert!(matrix.try_invert().unwrap() == Matrix::identity());
    }

    #[test]
    fn valid_matrix_invert() {
        let mut matrix = Matrix::identity();
        matrix.invert();
        assert!(matrix == Matrix::identity());
    }
}
