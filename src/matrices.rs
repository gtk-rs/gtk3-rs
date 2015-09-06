// Copyright 2013-2015, The RGtk Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Generic matrix operations

pub use ffi::Matrix;
use ffi;

pub trait MatrixTrait {
    fn null() -> Matrix;
    fn new(xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) -> Matrix;
    fn multiply(left: &Matrix, right: &Matrix) -> Matrix;
    fn identity() -> Matrix;
    fn init(&mut self, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64);
    fn translate(&mut self, tx: f64, ty: f64);
    fn scale(&mut self, sx: f64, sy: f64);
    fn rotate(&mut self, angle: f64);
    fn invert(&mut self);
    fn transform_distance(&self, _dx: f64, _dy: f64) -> (f64, f64);
    fn transform_point(&self, _x: f64, _y: f64) -> (f64, f64);
}

impl MatrixTrait for Matrix {
    /// Creates a new Matrix filled with zeroes
    fn null() -> Matrix {
        Matrix{
            xx: 0.0,
            yx: 0.0,
            xy: 0.0,
            yy: 0.0,
            x0: 0.0,
            y0: 0.0
        }
    }

    /// Creates a new matrix and fills it with given values
    fn new(xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) -> Matrix{
        let mut matrix = Matrix::null();
        matrix.init(xx, yx, xy, yy, x0, y0);
        matrix
    }

    /// Multiplies the affine transformations in a and b together and stores the result in
    /// the returned Matrix. The effect of the resulting transformation is to first apply
    /// the transformation in left to the coordinates and then apply the transformation in
    /// right to the coordinates.
    /// 
    /// It is allowable for the returned Matrix to be identical to either a or  .
    fn multiply(left: &Matrix, right: &Matrix) -> Matrix {
        let mut matrix = Matrix::null();
        unsafe {
            ffi::cairo_matrix_multiply(&mut matrix, left, right);
        }
        matrix
    }

    /// Returns a new matrix after modifying it to be an identity transformation.
    fn identity() -> Matrix {
        let mut matrix = Matrix::null();
        unsafe {
            ffi::cairo_matrix_init_identity(&mut matrix);
        }
        matrix
    }

    /// Sets self to be the affine transformation given by xx , yx , xy , yy , x0 , y0. The
    /// transformation is given by:
    ///
    /// ```ignore
    /// x_new = xx * x + xy * y + x0;
    /// y_new = yx * x + yy * y + y0;
    /// ```
    fn init(&mut self, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) {
        unsafe {
            ffi::cairo_matrix_init(self, xx, yx, xy, yy, x0, y0)
        }
    }

    /// Applies a translation by tx, ty to the transformation in self. The effect of the new
    /// transformation is to first translate the coordinates by tx and ty, then apply the
    /// original transformation to the coordinates.
    fn translate(&mut self, tx: f64, ty: f64) {
        unsafe {
            ffi::cairo_matrix_translate(self, tx, ty)
        }
    }

    /// Applies scaling by sx, sy to the transformation in self. The effect of the new
    /// transformation is to first scale the coordinates by sx and sy, then apply the original
    /// transformation to the coordinates.
    fn scale(&mut self, sx: f64, sy: f64) {
        unsafe {
            ffi::cairo_matrix_scale(self, sx, sy)
        }
    }

    /// Applies rotation by radians to the transformation in self. The effect of the new
    /// transformation is to first rotate the coordinates by radians , then apply the original
    /// transformation to the coordinates.
    fn rotate(&mut self, angle: f64) {
        unsafe {
            ffi::cairo_matrix_rotate(self, angle)
        }
    }

    /// Changes self to be the inverse of its original value. Not all transformation matrices
    /// have inverses; if the matrix collapses points together (it is degenerate), then it has
    /// no inverse and this function will fail.
    fn invert(&mut self) {
        let result = unsafe{
            ffi::cairo_matrix_invert(self)
        };
        result.ensure_valid();
    }

    /// Transforms the distance vector (dx, dy) by self. This is similar to
    /// Matrix::transform_point() except that the translation components of the transformation
    /// are ignored. The calculation of the returned vector is as follows:
    ///
    /// ```ignore
    /// dx2 = dx1 * a + dy1 * c;
    /// dy2 = dx1 * b + dy1 * d;
    /// ```
    ///
    /// Affine transformations are position invariant, so the same vector always transforms to
    /// the same vector. If (x1 ,y1 ) transforms to (x2 ,y2 ) then (x1 +dx1 ,y1 +dy1 ) will
    /// transform to (x1 + dx2, y1 + dy2) for all values of x1 and x2 .
    fn transform_distance(&self, _dx: f64, _dy: f64) -> (f64, f64){
        let mut dx = _dx;
        let mut dy = _dy;

        unsafe {
            ffi::cairo_matrix_transform_distance(self, &mut dx, &mut dy);
        }
        (dx, dy)
    }

    /// Transforms the point (x , y) by self.
    fn transform_point(&self, _x: f64, _y: f64) -> (f64, f64){
        let mut x = _x;
        let mut y = _y;

        unsafe {
            ffi::cairo_matrix_transform_point(self, &mut x, &mut y);
        }
        (x, y)
    }
}
