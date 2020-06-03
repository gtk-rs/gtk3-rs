// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use error::Error;
use ffi;
use std::ffi::CStr;
use std::fmt;

pub unsafe fn debug_reset_static_data() {
    ffi::cairo_debug_reset_static_data()
}

pub fn status_to_result<T>(status: ffi::cairo_status_t, obj: T) -> Result<T, Error> {
    match status {
        ffi::STATUS_SUCCESS => Ok(obj),
        ffi::STATUS_NO_MEMORY => Err(Error::NoMemory),
        ffi::STATUS_INVALID_RESTORE => Err(Error::InvalidRestore),
        ffi::STATUS_INVALID_POP_GROUP => Err(Error::InvalidPopGroup),
        ffi::STATUS_NO_CURRENT_POINT => Err(Error::NoCurrentPoint),
        ffi::STATUS_INVALID_MATRIX => Err(Error::InvalidMatrix),
        ffi::STATUS_INVALID_STATUS => Err(Error::InvalidStatus),
        ffi::STATUS_NULL_POINTER => Err(Error::NullPointer),
        ffi::STATUS_INVALID_STRING => Err(Error::InvalidString),
        ffi::STATUS_INVALID_PATH_DATA => Err(Error::InvalidPathData),
        ffi::STATUS_READ_ERROR => Err(Error::ReadError),
        ffi::STATUS_WRITE_ERROR => Err(Error::WriteError),
        ffi::STATUS_SURFACE_FINISHED => Err(Error::SurfaceFinished),
        ffi::STATUS_SURFACE_TYPE_MISMATCH => Err(Error::SurfaceTypeMismatch),
        ffi::STATUS_PATTERN_TYPE_MISMATCH => Err(Error::PatternTypeMismatch),
        ffi::STATUS_INVALID_CONTENT => Err(Error::InvalidContent),
        ffi::STATUS_INVALID_FORMAT => Err(Error::InvalidFormat),
        ffi::STATUS_INVALID_VISUAL => Err(Error::InvalidVisual),
        ffi::STATUS_FILE_NOT_FOUND => Err(Error::FileNotFound),
        ffi::STATUS_INVALID_DASH => Err(Error::InvalidDash),
        ffi::STATUS_INVALID_DSC_COMMENT => Err(Error::InvalidDscComment),
        ffi::STATUS_INVALID_INDEX => Err(Error::InvalidIndex),
        ffi::STATUS_CLIP_NOT_REPRESENTABLE => Err(Error::ClipNotRepresentable),
        ffi::STATUS_TEMP_FILE_ERROR => Err(Error::TempFileError),
        ffi::STATUS_INVALID_STRIDE => Err(Error::InvalidStride),
        ffi::STATUS_FONT_TYPE_MISMATCH => Err(Error::FontTypeMismatch),
        ffi::STATUS_USER_FONT_IMMUTABLE => Err(Error::UserFontImmutable),
        ffi::STATUS_USER_FONT_ERROR => Err(Error::UserFontError),
        ffi::STATUS_NEGATIVE_COUNT => Err(Error::NegativeCount),
        ffi::STATUS_INVALID_CLUSTERS => Err(Error::InvalidClusters),
        ffi::STATUS_INVALID_SLANT => Err(Error::InvalidSlant),
        ffi::STATUS_INVALID_WEIGHT => Err(Error::InvalidWeight),
        ffi::STATUS_INVALID_SIZE => Err(Error::InvalidSize),
        ffi::STATUS_USER_FONT_NOT_IMPLEMENTED => Err(Error::UserFontNotImplemented),
        ffi::STATUS_DEVICE_TYPE_MISMATCH => Err(Error::DeviceTypeMismatch),
        ffi::STATUS_DEVICE_ERROR => Err(Error::DeviceError),
        ffi::STATUS_INVALID_MESH_CONSTRUCTION => Err(Error::InvalidMeshConstruction),
        ffi::STATUS_DEVICE_FINISHED => Err(Error::DeviceFinished),
        ffi::STATUS_J_BIG2_GLOBAL_MISSING => Err(Error::JBig2GlobalMissing),
        ffi::STATUS_PNG_ERROR => Err(Error::PngError),
        ffi::STATUS_FREETYPE_ERROR => Err(Error::FreetypeError),
        ffi::STATUS_WIN32_GDI_ERROR => Err(Error::Win32GdiError),
        ffi::STATUS_LAST_STATUS => Err(Error::LastStatus),
        value => Err(Error::__Unknown(value)),
    }
}

pub fn get_version_string() -> &'static str {
    unsafe {
        let ptr = ffi::cairo_version_string();
        CStr::from_ptr(ptr)
            .to_str()
            .expect("invalid version string")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
}

impl Version {
    pub fn get_version() -> Version {
        let version = unsafe { ffi::cairo_version() };
        Version {
            major: (version / 10_000 % 100) as _,
            minor: (version / 100 % 100) as _,
            micro: (version % 100) as _,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.micro)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_versions() {
        assert_eq!(get_version_string(), Version::get_version().to_string());
    }
}
