// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::io;
use ffi::enums::Status;

#[derive(Debug)]
pub enum BorrowError { Cairo(Status), NonExclusive }

impl From<Status> for BorrowError {
    fn from(status: Status) -> Self {
        BorrowError::Cairo(status)
    }
}

#[derive(Debug)]
pub enum IoError { Cairo(Status), Io(io::Error) }

impl From<Status> for IoError {
    fn from(status: Status) -> Self {
        IoError::Cairo(status)
    }
}
