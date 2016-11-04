// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum UserDirectory {
    Desktop,
    Documents,
    Downloads,
    Music,
    Pictures,
    PublicShare,
    Templates,
    Videos,
    #[doc(hidden)]
    NDirectories,
}

#[doc(hidden)]
impl ToGlib for UserDirectory {
    type GlibType = ffi::GUserDirectory;

    fn to_glib(&self) -> ffi::GUserDirectory {
        match *self {
            UserDirectory::Desktop => ffi::G_USER_DIRECTORY_DESKTOP,
            UserDirectory::Documents => ffi::G_USER_DIRECTORY_DOCUMENTS,
            UserDirectory::Downloads => ffi::G_USER_DIRECTORY_DOWNLOAD,
            UserDirectory::Music => ffi::G_USER_DIRECTORY_MUSIC,
            UserDirectory::Pictures => ffi::G_USER_DIRECTORY_PICTURES,
            UserDirectory::PublicShare => ffi::G_USER_DIRECTORY_PUBLIC_SHARE,
            UserDirectory::Templates => ffi::G_USER_DIRECTORY_TEMPLATES,
            UserDirectory::Videos => ffi::G_USER_DIRECTORY_VIDEOS,
            UserDirectory::NDirectories => ffi::G_USER_N_DIRECTORIES,
        }
    }
}
