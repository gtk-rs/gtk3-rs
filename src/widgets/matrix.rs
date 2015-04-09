// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use ffi::{self, PangoMatrix, PangoRectangle};
//use std::default::Default;

/// A structure specifying a transformation between user-space coordinates and device coordinates.
/// The transformation is given by:
/// 
/// x_device = x_user * matrix->xx + y_user * matrix->xy + matrix->x0;
/// y_device = x_user * matrix->yx + y_user * matrix->yy + matrix->y0;
pub trait Matrix {
    fn new(xx: f64, xy: f64, yx: f64, yy: f64, x0: f64, y0: f64) -> Self;
    fn copy(&self) -> Self;
    fn translate(&mut self, t_x: f64, t_y: f64);
    fn scale(&mut self, scale_x: f64, scale_y: f64);
    fn rotate(&mut self, degrees: f64);
    fn concat(&mut self, new_matrix: &Self);
    fn transform_point(&self, x: &mut f64, y: &mut f64);
    fn transform_distance(&self, dx: &mut f64, dy: &mut f64);
    fn transform_rectangle(&self, rect: &mut PangoRectangle);
    fn transform_pixel_rectangle(&self, rect: &mut PangoRectangle);
    fn get_font_scale_factor(&mut self) -> f64;
}

impl Matrix for PangoMatrix {
    fn new(xx: f64, xy: f64, yx: f64, yy: f64, x0: f64, y0: f64) -> PangoMatrix {
        PangoMatrix {
            xx: xx,
            xy: xy,
            yx: yx,
            yy: yy,
            x0: x0,
            y0: y0
        }
    }

    fn copy(&self) -> PangoMatrix {
        PangoMatrix {
            xx: self.xx,
            xy: self.xy,
            yx: self.yx,
            yy: self.yy,
            x0: self.x0,
            y0: self.y0
        }
    }

    fn translate(&mut self, t_x: f64, t_y: f64) {
        unsafe { ffi::pango_matrix_translate(self, t_x, t_y) }
    }

    fn scale(&mut self, scale_x: f64, scale_y: f64) {
        unsafe { ffi::pango_matrix_scale(self, scale_x, scale_y) }
    }

    fn rotate(&mut self, degrees: f64) {
        unsafe { ffi::pango_matrix_rotate(self, degrees) }
    }

    fn concat(&mut self, new_matrix: &PangoMatrix) {
        unsafe { ffi::pango_matrix_concat(self, new_matrix) }
    }

    fn transform_point(&self, x: &mut f64, y: &mut f64) {
        unsafe { ffi::pango_matrix_transform_point(self, x, y) }
    }

    fn transform_distance(&self, dx: &mut f64, dy: &mut f64) {
        unsafe { ffi::pango_matrix_transform_distance(self, dx, dy) }
    }

    fn transform_rectangle(&self, rect: &mut PangoRectangle) {
        unsafe { ffi::pango_matrix_transform_rectangle(self, rect) }
    }

    fn transform_pixel_rectangle(&self, rect: &mut PangoRectangle) {
        unsafe { ffi::pango_matrix_transform_pixel_rectangle(self, rect) }
    }

    fn get_font_scale_factor(&mut self) -> f64 {
        unsafe { ffi::pango_matrix_get_font_scale_factor(self) }
    }
}

/*impl Default for Matrix {
    fn default() -> Matrix {
        Matrix {
            xx: 1.,
            xy: 0.,
            yx: 0.,
            yy: 1.,
            x0: 0.,
            y0: 0.
        }
    }
}*/