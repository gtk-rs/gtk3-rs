// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use enums::Status;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BorrowError {
    #[error("Failed to borrow with Cairo error:{0}")]
    Cairo(Status),
    #[error("Can't get exclusive access")]
    NonExclusive,
}

impl From<Status> for BorrowError {
    fn from(status: Status) -> Self {
        BorrowError::Cairo(status)
    }
}

#[derive(Error, Debug)]
pub enum IoError {
    #[error("Cairo error: {0}")]
    Cairo(Status),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

impl From<Status> for IoError {
    fn from(status: Status) -> Self {
        IoError::Cairo(status)
    }
}
