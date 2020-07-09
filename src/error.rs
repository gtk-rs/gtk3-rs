// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>
use std::fmt::Debug;

use std::io;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Copy, Eq)]
#[non_exhaustive]
pub enum Error {
    #[error("No Memory")]
    NoMemory,
    #[error("Invalid Restore")]
    InvalidRestore,
    #[error("Invalid Pop Group")]
    InvalidPopGroup,
    #[error("No Current Point")]
    NoCurrentPoint,
    #[error("Invalid Matrix")]
    InvalidMatrix,
    #[error("Invalid Status")]
    InvalidStatus,
    #[error("Null Pointer")]
    NullPointer,
    #[error("Invalid String")]
    InvalidString,
    #[error("Invalid Path Data")]
    InvalidPathData,
    #[error("Cairo : Read Error")]
    ReadError,
    #[error("Write Error")]
    WriteError,
    #[error("Surface Finishied")]
    SurfaceFinished,
    #[error("Surface Type Mismatch")]
    SurfaceTypeMismatch,
    #[error("Pattern Type Mismatch")]
    PatternTypeMismatch,
    #[error("Invalid Content")]
    InvalidContent,
    #[error("Invalid Format")]
    InvalidFormat,
    #[error("Invalid Visual")]
    InvalidVisual,
    #[error("File Not Found")]
    FileNotFound,
    #[error("Invalid Dash")]
    InvalidDash,
    #[error("Invalid Dash Comment")]
    InvalidDscComment,
    #[error("Invalid Index")]
    InvalidIndex,
    #[error("Clip Not Representable")]
    ClipNotRepresentable,
    #[error("Temp File Error")]
    TempFileError,
    #[error("Invalid Stride")]
    InvalidStride,
    #[error("Font Type Mismatch")]
    FontTypeMismatch,
    #[error("User Font Immutable")]
    UserFontImmutable,
    #[error("User Font Error")]
    UserFontError,
    #[error("Negative Count")]
    NegativeCount,
    #[error("Invalid Clusters")]
    InvalidClusters,
    #[error("Invalid Slant")]
    InvalidSlant,
    #[error("Invalid Weight")]
    InvalidWeight,
    #[error("Ivalid Size")]
    InvalidSize,
    #[error("User Font Not Implemented")]
    UserFontNotImplemented,
    #[error("Device Type Mismatch")]
    DeviceTypeMismatch,
    #[error("Device Error")]
    DeviceError,
    #[error("Invalid Mesh Construction")]
    InvalidMeshConstruction,
    #[error("Device Finished")]
    DeviceFinished,
    #[error("JBig2Global Missing")]
    JBig2GlobalMissing,
    #[error("PNG Error")]
    PngError,
    #[error("Freetype Error")]
    FreetypeError,
    #[error("Win32Gdi Error")]
    Win32GdiError,
    #[error("LastStatus")]
    LastStatus,
    #[error("Unknown {0}")]
    #[doc(hidden)]
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
