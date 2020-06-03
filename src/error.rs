// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>
use std::fmt::{self, Debug};

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    NoMemory,
    InvalidRestore,
    InvalidPopGroup,
    NoCurrentPoint,
    InvalidMatrix,
    InvalidStatus,
    NullPointer,
    InvalidString,
    InvalidPathData,
    ReadError,
    WriteError,
    SurfaceFinished,
    SurfaceTypeMismatch,
    PatternTypeMismatch,
    InvalidContent,
    InvalidFormat,
    InvalidVisual,
    FileNotFound,
    InvalidDash,
    InvalidDscComment,
    InvalidIndex,
    ClipNotRepresentable,
    TempFileError,
    InvalidStride,
    FontTypeMismatch,
    UserFontImmutable,
    UserFontError,
    NegativeCount,
    InvalidClusters,
    InvalidSlant,
    InvalidWeight,
    InvalidSize,
    UserFontNotImplemented,
    DeviceTypeMismatch,
    DeviceError,
    InvalidMeshConstruction,
    DeviceFinished,
    JBig2GlobalMissing,
    PngError,
    FreetypeError,
    Win32GdiError,
    LastStatus,
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error::{}",
            match *self {
                Error::NoMemory => "NoMemory",
                Error::InvalidRestore => "InvalidRestore",
                Error::InvalidPopGroup => "InvalidPopGroup",
                Error::NoCurrentPoint => "NoCurrentPoint",
                Error::InvalidMatrix => "InvalidMatrix",
                Error::InvalidStatus => "InvalidStatus",
                Error::NullPointer => "NullPointer",
                Error::InvalidString => "InvalidString",
                Error::InvalidPathData => "InvalidPathData",
                Error::ReadError => "ReadError",
                Error::WriteError => "WriteError",
                Error::SurfaceFinished => "SurfaceFinished",
                Error::SurfaceTypeMismatch => "SurfaceTypeMismatch",
                Error::PatternTypeMismatch => "PatternTypeMismatch",
                Error::InvalidContent => "InvalidContent",
                Error::InvalidFormat => "InvalidFormat",
                Error::InvalidVisual => "InvalidVisual",
                Error::FileNotFound => "FileNotFound",
                Error::InvalidDash => "InvalidDash",
                Error::InvalidDscComment => "InvalidDscComment",
                Error::InvalidIndex => "InvalidIndex",
                Error::ClipNotRepresentable => "ClipNotRepresentable",
                Error::TempFileError => "TempFileError",
                Error::InvalidStride => "InvalidStride",
                Error::FontTypeMismatch => "FontTypeMismatch",
                Error::UserFontImmutable => "UserFontImmutable",
                Error::UserFontError => "UserFontError",
                Error::NegativeCount => "NegativeCount",
                Error::InvalidClusters => "InvalidClusters",
                Error::InvalidSlant => "InvalidSlant",
                Error::InvalidWeight => "InvalidWeight",
                Error::InvalidSize => "InvalidSize",
                Error::UserFontNotImplemented => "UserFontNotImplemented",
                Error::DeviceTypeMismatch => "DeviceTypeMismatch",
                Error::DeviceError => "DeviceError",
                Error::InvalidMeshConstruction => "InvalidMeshConstruction",
                Error::DeviceFinished => "DeviceFinished",
                Error::JBig2GlobalMissing => "JBig2GlobalMissing",
                Error::PngError => "PngError",
                Error::FreetypeError => "FreetypeError",
                Error::Win32GdiError => "Win32GdiError",
                Error::LastStatus => "LastStatus",
                _ => "Unknown",
            }
        )
    }
}

#[derive(Error, Debug)]
pub enum IoError {
    #[error("Cairo error: {0}")]
    Cairo(Error),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

impl From<Error> for IoError {
    fn from(error: Error) -> Self {
        IoError::Cairo(error)
    }
}

#[derive(Error, Debug)]
pub enum BorrowError {
    #[error("Failed to borrow with Cairo error:{0}")]
    Cairo(Error),
    #[error("Can't get exclusive access")]
    NonExclusive,
}

impl From<Error> for BorrowError {
    fn from(error: Error) -> Self {
        BorrowError::Cairo(error)
    }
}
