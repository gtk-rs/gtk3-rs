// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>
use std::fmt::Debug;

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cairo: No Memory")]
    NoMemory,
    #[error("Cairo: Invalid Restore")]
    InvalidRestore,
    #[error("Cairo: Invalid Pop Group")]
    InvalidPopGroup,
    #[error("Cairo: No Current Point")]
    NoCurrentPoint,
    #[error("Cairo: Invalid Matrix")]
    InvalidMatrix,
    #[error("Cairo: Invalid Status")]
    InvalidStatus,
    #[error("Cairo: Null Pointer")]
    NullPointer,
    #[error("Cairo: Invalid String")]
    InvalidString,
    #[error("Cairo: Invalid Path Data")]
    InvalidPathData,
    #[error("Cairo : Read Error")]
    ReadError,
    #[error("Cairo: Write Error")]
    WriteError,
    #[error("Cairo: Surface Finishied")]
    SurfaceFinished,
    #[error("Cairo: Surface Type Mismatch")]
    SurfaceTypeMismatch,
    #[error("Cairo: Pattern Type Mismatch")]
    PatternTypeMismatch,
    #[error("Cairo: Invalid Content")]
    InvalidContent,
    #[error("Cairo: Invalid Format")]
    InvalidFormat,
    #[error("Cairo: Invalid Visual")]
    InvalidVisual,
    #[error("Cairo: File Not Found")]
    FileNotFound,
    #[error("Cairo: Invalid Dash")]
    InvalidDash,
    #[error("Cairo: Invalid Dash Comment")]
    InvalidDscComment,
    #[error("Cairo: Invalid Index")]
    InvalidIndex,
    #[error("Cairo: Clip Not Representable")]
    ClipNotRepresentable,
    #[error("Cairo: Temp File Error")]
    TempFileError,
    #[error("Cairo: Invalid Stride")]
    InvalidStride,
    #[error("Cairo: Font Type Mismatch")]
    FontTypeMismatch,
    #[error("Cairo: User Font Immutable")]
    UserFontImmutable,
    #[error("Cairo: User Font Error")]
    UserFontError,
    #[error("Cairo: Negative Count")]
    NegativeCount,
    #[error("Cairo: Invalid Clusters")]
    InvalidClusters,
    #[error("Cairo: Invalid Slant")]
    InvalidSlant,
    #[error("Cairo: Invalid Weight")]
    InvalidWeight,
    #[error("Cairo: Ivalid Size")]
    InvalidSize,
    #[error("Cairo: User Font Not Implemented")]
    UserFontNotImplemented,
    #[error("Cairo: Device Type Mismatch")]
    DeviceTypeMismatch,
    #[error("Cairo: Device Error")]
    DeviceError,
    #[error("Cairo: Invalid Mesh Construction")]
    InvalidMeshConstruction,
    #[error("Cairo: Device Finished")]
    DeviceFinished,
    #[error("Cairo: JBig2Global Missing")]
    JBig2GlobalMissing,
    #[error("Cairo: PNG Error")]
    PngError,
    #[error("Cairo: Freetype Error")]
    FreetypeError,
    #[error("Cairo: Win32Gdi Error")]
    Win32GdiError,
    #[error("Cairo: LastStatus")]
    LastStatus,
    #[error("Cairo: Uknown {0}")]
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
