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

            PathSegments {
                data: CVec::new(data_ptr, length),
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

#[derive(Debug, Clone, Copy)]
pub enum PathSegment {
    MoveTo((f64,f64)),
    LineTo((f64,f64)),
    CurveTo((f64, f64),(f64, f64),(f64, f64)),
    ClosePath
}

pub struct PathSegments {
    data: CVec<[f64; 2]>,
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
            let data_header: &cairo_path_data_header = transmute(self.data.get(i));
            (data_header.data_type, data_header.length)
        };

        self.i += length as usize;

        let ref data = self.data;

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
