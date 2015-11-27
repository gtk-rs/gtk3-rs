// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

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

    fn new(xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) -> Matrix{
        let mut matrix = Matrix::null();
        matrix.init(xx, yx, xy, yy, x0, y0);
        matrix
    }

    fn multiply(left: &Matrix, right: &Matrix) -> Matrix {
        let mut matrix = Matrix::null();
        unsafe {
            ffi::cairo_matrix_multiply(&mut matrix, left, right);
        }
        matrix
    }

    fn identity() -> Matrix {
        let mut matrix = Matrix::null();
        unsafe {
            ffi::cairo_matrix_init_identity(&mut matrix);
        }
        matrix
    }

    fn init(&mut self, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) {
        unsafe {
            ffi::cairo_matrix_init(self, xx, yx, xy, yy, x0, y0)
        }
    }

    fn translate(&mut self, tx: f64, ty: f64) {
        unsafe {
            ffi::cairo_matrix_translate(self, tx, ty)
        }
    }

    fn scale(&mut self, sx: f64, sy: f64) {
        unsafe {
            ffi::cairo_matrix_scale(self, sx, sy)
        }
    }

    fn rotate(&mut self, angle: f64) {
        unsafe {
            ffi::cairo_matrix_rotate(self, angle)
        }
    }

    fn invert(&mut self) {
        let result = unsafe{
            ffi::cairo_matrix_invert(self)
        };
        result.ensure_valid();
    }

    fn transform_distance(&self, _dx: f64, _dy: f64) -> (f64, f64){
        let mut dx = _dx;
        let mut dy = _dy;

        unsafe {
            ffi::cairo_matrix_transform_distance(self, &mut dx, &mut dy);
        }
        (dx, dy)
    }

    fn transform_point(&self, _x: f64, _y: f64) -> (f64, f64){
        let mut x = _x;
        let mut y = _y;

        unsafe {
            ffi::cairo_matrix_transform_point(self, &mut x, &mut y);
        }
        (x, y)
    }
}
