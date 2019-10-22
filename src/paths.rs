// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use enums::{PathDataType, Status};
use ffi;
use ffi::cairo_path_t;
use std::fmt;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Path(*mut cairo_path_t);

impl Path {
    pub fn as_ptr(&self) -> *mut cairo_path_t {
        self.0
    }

    pub fn ensure_status(&self) {
        unsafe {
            let ptr: *mut cairo_path_t = self.as_ptr();
            Status::from((*ptr).status).ensure_valid()
        }
    }

    pub unsafe fn from_raw_full(pointer: *mut cairo_path_t) -> Path {
        Path(pointer)
    }

    pub fn iter(&self) -> PathSegments {
        use std::slice;

        unsafe {
            let ptr: *mut cairo_path_t = self.as_ptr();
            let length = (*ptr).num_data as usize;
            let data_ptr = (*ptr).data;
            let data_vec = if length != 0 && !data_ptr.is_null() {
                slice::from_raw_parts(data_ptr, length)
            } else {
                &[]
            };

            PathSegments {
                data: data_vec,
                i: 0,
                num_data: length,
            }
        }
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_path_destroy(self.as_ptr());
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathSegment {
    MoveTo((f64, f64)),
    LineTo((f64, f64)),
    CurveTo((f64, f64), (f64, f64), (f64, f64)),
    ClosePath,
}

impl fmt::Display for PathSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PathSegment::{}",
            match *self {
                PathSegment::MoveTo(_) => "MoveTo",
                PathSegment::LineTo(_) => "LineTo",
                PathSegment::CurveTo(_, _, _) => "CurveTo",
                PathSegment::ClosePath => "ClosePath",
            }
        )
    }
}

pub struct PathSegments<'a> {
    data: &'a [ffi::cairo_path_data],
    i: usize,
    num_data: usize,
}

impl<'a> Iterator for PathSegments<'a> {
    type Item = PathSegment;

    fn next(&mut self) -> Option<PathSegment> {
        if self.i >= self.num_data {
            return None;
        }

        unsafe {
            let res = match PathDataType::from(self.data[self.i].header.data_type) {
                PathDataType::MoveTo => PathSegment::MoveTo(to_tuple(&self.data[self.i + 1].point)),
                PathDataType::LineTo => PathSegment::LineTo(to_tuple(&self.data[self.i + 1].point)),
                PathDataType::CurveTo => PathSegment::CurveTo(
                    to_tuple(&self.data[self.i + 1].point),
                    to_tuple(&self.data[self.i + 2].point),
                    to_tuple(&self.data[self.i + 3].point),
                ),
                PathDataType::ClosePath => PathSegment::ClosePath,
                PathDataType::__Unknown(x) => panic!("Unknown value: {}", x),
            };

            self.i += self.data[self.i].header.length as usize;

            Some(res)
        }
    }
}

impl<'a> fmt::Display for PathSegments<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PathSegments")
    }
}

fn to_tuple(pair: &[f64; 2]) -> (f64, f64) {
    (pair[0], pair[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    use context::*;
    use enums::Format;
    use image_surface::*;

    fn make_cr() -> Context {
        let surface = ImageSurface::create(Format::Rgb24, 1, 1).unwrap();

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

        assert_path_equals_segments(&path, &vec![PathSegment::MoveTo((1.0, 2.0))]);
    }

    #[test]
    fn moveto_lineto_moveto() {
        let cr = make_cr();

        cr.move_to(1.0, 2.0);
        cr.line_to(3.0, 4.0);
        cr.move_to(5.0, 6.0);

        let path = cr.copy_path();

        assert_path_equals_segments(
            &path,
            &vec![
                PathSegment::MoveTo((1.0, 2.0)),
                PathSegment::LineTo((3.0, 4.0)),
                PathSegment::MoveTo((5.0, 6.0)),
            ],
        );
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
        assert_path_equals_segments(
            &path,
            &vec![
                PathSegment::MoveTo((1.0, 2.0)),
                PathSegment::ClosePath,
                PathSegment::MoveTo((1.0, 2.0)),
            ],
        );
    }
    #[test]
    fn curveto_closed_subpath_lineto() {
        let cr = make_cr();

        cr.move_to(1.0, 2.0);
        cr.curve_to(3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        cr.close_path();
        cr.line_to(9.0, 10.0);

        let path = cr.copy_path();

        assert_path_equals_segments(
            &path,
            &vec![
                PathSegment::MoveTo((1.0, 2.0)),
                PathSegment::CurveTo((3.0, 4.0), (5.0, 6.0), (7.0, 8.0)),
                PathSegment::ClosePath,
                PathSegment::MoveTo((1.0, 2.0)),
                PathSegment::LineTo((9.0, 10.0)),
            ],
        );
    }
}
