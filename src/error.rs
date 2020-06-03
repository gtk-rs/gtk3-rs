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
    __Unknown(i32),
}
#[doc(hidden)]
impl Into<ffi::cairo_status_t> for Error {
    fn into(self) -> ffi::cairo_status_t {
        match self {
            Error::NoMemory => ffi::STATUS_NO_MEMORY,
            Error::InvalidRestore => ffi::STATUS_INVALID_RESTORE,
            Error::InvalidPopGroup => ffi::STATUS_INVALID_POP_GROUP,
            Error::NoCurrentPoint => ffi::STATUS_NO_CURRENT_POINT,
            Error::InvalidMatrix => ffi::STATUS_INVALID_MATRIX,
            Error::InvalidStatus => ffi::STATUS_INVALID_STATUS,
            Error::NullPointer => ffi::STATUS_NULL_POINTER,
            Error::InvalidString => ffi::STATUS_INVALID_STRING,
            Error::InvalidPathData => ffi::STATUS_INVALID_PATH_DATA,
            Error::ReadError => ffi::STATUS_READ_ERROR,
            Error::WriteError => ffi::STATUS_WRITE_ERROR,
            Error::SurfaceFinished => ffi::STATUS_SURFACE_FINISHED,
            Error::SurfaceTypeMismatch => ffi::STATUS_SURFACE_TYPE_MISMATCH,
            Error::PatternTypeMismatch => ffi::STATUS_PATTERN_TYPE_MISMATCH,
            Error::InvalidContent => ffi::STATUS_INVALID_CONTENT,
            Error::InvalidFormat => ffi::STATUS_INVALID_FORMAT,
            Error::InvalidVisual => ffi::STATUS_INVALID_VISUAL,
            Error::FileNotFound => ffi::STATUS_FILE_NOT_FOUND,
            Error::InvalidDash => ffi::STATUS_INVALID_DASH,
            Error::InvalidDscComment => ffi::STATUS_INVALID_DSC_COMMENT,
            Error::InvalidIndex => ffi::STATUS_INVALID_INDEX,
            Error::ClipNotRepresentable => ffi::STATUS_CLIP_NOT_REPRESENTABLE,
            Error::TempFileError => ffi::STATUS_TEMP_FILE_ERROR,
            Error::InvalidStride => ffi::STATUS_INVALID_STRIDE,
            Error::FontTypeMismatch => ffi::STATUS_FONT_TYPE_MISMATCH,
            Error::UserFontImmutable => ffi::STATUS_USER_FONT_IMMUTABLE,
            Error::UserFontError => ffi::STATUS_USER_FONT_ERROR,
            Error::NegativeCount => ffi::STATUS_NEGATIVE_COUNT,
            Error::InvalidClusters => ffi::STATUS_INVALID_CLUSTERS,
            Error::InvalidSlant => ffi::STATUS_INVALID_SLANT,
            Error::InvalidWeight => ffi::STATUS_INVALID_WEIGHT,
            Error::InvalidSize => ffi::STATUS_INVALID_SIZE,
            Error::UserFontNotImplemented => ffi::STATUS_USER_FONT_NOT_IMPLEMENTED,
            Error::DeviceTypeMismatch => ffi::STATUS_DEVICE_TYPE_MISMATCH,
            Error::DeviceError => ffi::STATUS_DEVICE_ERROR,
            Error::InvalidMeshConstruction => ffi::STATUS_INVALID_MESH_CONSTRUCTION,
            Error::DeviceFinished => ffi::STATUS_DEVICE_FINISHED,
            Error::JBig2GlobalMissing => ffi::STATUS_J_BIG2_GLOBAL_MISSING,
            Error::PngError => ffi::STATUS_PNG_ERROR,
            Error::FreetypeError => ffi::STATUS_FREETYPE_ERROR,
            Error::Win32GdiError => ffi::STATUS_WIN32_GDI_ERROR,
            Error::LastStatus => ffi::STATUS_LAST_STATUS,
            Error::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_status_t> for Error {
    fn from(value: ffi::cairo_status_t) -> Self {
        match value {
            ffi::STATUS_NO_MEMORY => Error::NoMemory,
            ffi::STATUS_INVALID_RESTORE => Error::InvalidRestore,
            ffi::STATUS_INVALID_POP_GROUP => Error::InvalidPopGroup,
            ffi::STATUS_NO_CURRENT_POINT => Error::NoCurrentPoint,
            ffi::STATUS_INVALID_MATRIX => Error::InvalidMatrix,
            ffi::STATUS_INVALID_STATUS => Error::InvalidStatus,
            ffi::STATUS_NULL_POINTER => Error::NullPointer,
            ffi::STATUS_INVALID_STRING => Error::InvalidString,
            ffi::STATUS_INVALID_PATH_DATA => Error::InvalidPathData,
            ffi::STATUS_READ_ERROR => Error::ReadError,
            ffi::STATUS_WRITE_ERROR => Error::WriteError,
            ffi::STATUS_SURFACE_FINISHED => Error::SurfaceFinished,
            ffi::STATUS_SURFACE_TYPE_MISMATCH => Error::SurfaceTypeMismatch,
            ffi::STATUS_PATTERN_TYPE_MISMATCH => Error::PatternTypeMismatch,
            ffi::STATUS_INVALID_CONTENT => Error::InvalidContent,
            ffi::STATUS_INVALID_FORMAT => Error::InvalidFormat,
            ffi::STATUS_INVALID_VISUAL => Error::InvalidVisual,
            ffi::STATUS_FILE_NOT_FOUND => Error::FileNotFound,
            ffi::STATUS_INVALID_DASH => Error::InvalidDash,
            ffi::STATUS_INVALID_DSC_COMMENT => Error::InvalidDscComment,
            ffi::STATUS_INVALID_INDEX => Error::InvalidIndex,
            ffi::STATUS_CLIP_NOT_REPRESENTABLE => Error::ClipNotRepresentable,
            ffi::STATUS_TEMP_FILE_ERROR => Error::TempFileError,
            ffi::STATUS_INVALID_STRIDE => Error::InvalidStride,
            ffi::STATUS_FONT_TYPE_MISMATCH => Error::FontTypeMismatch,
            ffi::STATUS_USER_FONT_IMMUTABLE => Error::UserFontImmutable,
            ffi::STATUS_USER_FONT_ERROR => Error::UserFontError,
            ffi::STATUS_NEGATIVE_COUNT => Error::NegativeCount,
            ffi::STATUS_INVALID_CLUSTERS => Error::InvalidClusters,
            ffi::STATUS_INVALID_SLANT => Error::InvalidSlant,
            ffi::STATUS_INVALID_WEIGHT => Error::InvalidWeight,
            ffi::STATUS_INVALID_SIZE => Error::InvalidSize,
            ffi::STATUS_USER_FONT_NOT_IMPLEMENTED => Error::UserFontNotImplemented,
            ffi::STATUS_DEVICE_TYPE_MISMATCH => Error::DeviceTypeMismatch,
            ffi::STATUS_DEVICE_ERROR => Error::DeviceError,
            ffi::STATUS_INVALID_MESH_CONSTRUCTION => Error::InvalidMeshConstruction,
            ffi::STATUS_DEVICE_FINISHED => Error::DeviceFinished,
            ffi::STATUS_J_BIG2_GLOBAL_MISSING => Error::JBig2GlobalMissing,
            ffi::STATUS_PNG_ERROR => Error::PngError,
            ffi::STATUS_FREETYPE_ERROR => Error::FreetypeError,
            ffi::STATUS_WIN32_GDI_ERROR => Error::Win32GdiError,
            ffi::STATUS_LAST_STATUS => Error::LastStatus,
            value => Error::__Unknown(value),
        }
    }
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
    Cairo(#[from] Error),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum BorrowError {
    #[error("Failed to borrow with Cairo error: {0}")]
    Cairo(#[from] ::Error),
    #[error("Can't get exclusive access")]
    NonExclusive,
}
