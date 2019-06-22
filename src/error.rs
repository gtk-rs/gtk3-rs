// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use enums::Status;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum BorrowError {
    Cairo(Status),
    NonExclusive,
}

impl From<Status> for BorrowError {
    fn from(status: Status) -> Self {
        BorrowError::Cairo(status)
    }
}

impl fmt::Display for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BorrowError::Cairo(status) => write!(f, "BorrowError::Cairo({})", status),
            BorrowError::NonExclusive => write!(f, "BorrowError::NonExclusive"),
        }
    }
}

impl Error for BorrowError {
    fn description(&self) -> &str {
        match *self {
            BorrowError::Cairo(_) => "BorrowError::Cairo",
            BorrowError::NonExclusive => "BorrowError::NonExclusive",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

#[derive(Debug)]
pub enum IoError {
    Cairo(Status),
    Io(io::Error),
}

impl From<Status> for IoError {
    fn from(status: Status) -> Self {
        IoError::Cairo(status)
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IoError::Cairo(status) => write!(f, "IoError::Cairo({})", status),
            IoError::Io(ref e) => write!(f, "IoError::Io({})", e),
        }
    }
}

impl Error for IoError {
    fn description(&self) -> &str {
        match *self {
            IoError::Cairo(_) => "IoError::Cairo",
            IoError::Io(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            IoError::Cairo(_) => None,
            IoError::Io(ref e) => Some(e),
        }
    }
}
