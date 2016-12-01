// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem::transmute;
use std::iter::Iterator;
use c_vec::CVec;
use ffi::enums::PathDataType;
use ffi::{
    cairo_path_t,
    cairo_path_data_header
};
use ffi;

pub struct Path(*mut cairo_path_t);

impl Path {
    #[doc(hidden)]
    pub fn get_ptr(&self) -> *mut cairo_path_t {
        let Path(ptr) = *self;

        ptr
    }

    pub fn ensure_status(&self) {
        unsafe {
            let ptr: *mut cairo_path_t = self.get_ptr();
            (*ptr).status.ensure_valid()
        }
    }

    #[doc(hidden)]
    pub fn wrap(pointer: *mut cairo_path_t) -> Path {
        Path(pointer)
    }

    pub fn iter(&self) -> PathSegments {
        unsafe {
            let ptr: *mut cairo_path_t = self.get_ptr();
            let length = (*ptr).num_data as usize;
            let data_ptr = (*ptr).data;
            let data_vec = if length != 0 { Some(CVec::new(data_ptr, length)) } else { None };

            PathSegments {
                data: data_vec,
                i: 0,
                num_data: length
            }
        }
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe{
            ffi::cairo_path_destroy(self.get_ptr());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathSegment {
    MoveTo((f64,f64)),
    LineTo((f64,f64)),
    CurveTo((f64, f64),(f64, f64),(f64, f64)),
    ClosePath
}

pub struct PathSegments {
    data: Option<CVec<[f64; 2]>>,
    i: usize,
    num_data: usize
}

impl Iterator for PathSegments {
    type Item = PathSegment;

    fn next(&mut self) -> Option<PathSegment> {
        let i = self.i;

        if i >= self.num_data{
            return None;
        }

        let (data_type, length) = unsafe {
            let data_header: &cairo_path_data_header = transmute(self.data.as_ref().unwrap().get(i));
            (data_header.data_type, data_header.length)
        };

        self.i += length as usize;

        let ref data = self.data.as_ref().unwrap();

        Some(match data_type {
            PathDataType::MoveTo => PathSegment::MoveTo(to_tuple(data.get(i + 1).unwrap())),
            PathDataType::LineTo => PathSegment::LineTo(to_tuple(data.get(i + 1).unwrap())),
            PathDataType::CurveTo => {
                PathSegment::CurveTo(to_tuple(data.get(i + 1).unwrap()),
                    to_tuple(data.get(i + 2).unwrap()), to_tuple(data.get(i + 3).unwrap()))
            }
            PathDataType::ClosePath => PathSegment::ClosePath
        })
    }
}

fn to_tuple(pair: &[f64; 2]) -> (f64, f64) {
    (pair[0], pair[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    use context::*;
    use image_surface::*;
    use ffi::enums::Format;

    fn make_cr() -> Context {
        let surface = ImageSurface::create(Format::Rgb24, 1, 1);

        Context::new(&surface)
    }

    fn assert_path_equals_segments(expected: &Path, actual: &Vec<PathSegment>) {
        // First ensure the lengths are equal

        let expected_iter = expected.iter();
        let actual_iter = actual.iter();

        assert_eq!(expected_iter.count(), actual_iter.count());

        // Then actually compare the contents

        let expected_iter = expected.iter();
        let actual_iter = actual.iter();

        let mut iter = expected_iter.zip(actual_iter);

        while let Some((e, a)) = iter.next() {
            assert_eq!(e, *a);
        }
    }

    #[test]
    fn empty_path_doesnt_iter() {
        let cr = make_cr();

        let path = cr.copy_path();

        assert!(path.iter().next().is_none());
    }

    #[test]
    fn moveto() {
        let cr = make_cr();

        cr.move_to(1.0, 2.0);

        let path = cr.copy_path();

        assert_path_equals_segments(&path,
                                    &vec![PathSegment::MoveTo((1.0, 2.0))]);
    }

    #[test]
    fn moveto_lineto_moveto() {
        let cr = make_cr();

        cr.move_to(1.0, 2.0);
        cr.line_to(3.0, 4.0);
        cr.move_to(5.0, 6.0);

        let path = cr.copy_path();

        assert_path_equals_segments(&path,
                                    &vec![PathSegment::MoveTo((1.0, 2.0)),
                                          PathSegment::LineTo((3.0, 4.0)),
                                          PathSegment::MoveTo((5.0, 6.0))]);
    }

    #[test]
    fn moveto_closepath() {
        let cr = make_cr();

        cr.move_to(1.0, 2.0);
        cr.close_path();

        let path = cr.copy_path();

        // Note that Cairo represents a close_path as closepath+moveto,
        // so that the next subpath will have a starting point,
        // from the extra moveto.
        assert_path_equals_segments(&path,
                                    &vec![PathSegment::MoveTo((1.0, 2.0)),
                                          PathSegment::ClosePath,
                                          PathSegment::MoveTo((1.0, 2.0))]);
    }
    #[test]
    fn curveto_closed_subpath_lineto() {
        let cr = make_cr();

        cr.move_to(1.0, 2.0);
        cr.curve_to(3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        cr.close_path();
        cr.line_to(9.0, 10.0);

        let path = cr.copy_path();

        assert_path_equals_segments(&path,
                                    &vec![PathSegment::MoveTo((1.0, 2.0)),
                                          PathSegment::CurveTo((3.0, 4.0), (5.0, 6.0), (7.0, 8.0)),
                                          PathSegment::ClosePath,
                                          PathSegment::MoveTo((1.0, 2.0)),
                                          PathSegment::LineTo((9.0, 10.0))]);
    }

}
