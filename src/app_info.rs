// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_ffi;
use object::IsA;

glib_wrapper! {
    pub struct AppInfo(Object<gio_ffi::GAppInfo>);

    match fn {
        get_type => || gio_ffi::g_app_info_get_type(),
    }
}

pub trait AppInfoExt {
}

impl<O: IsA<AppInfo>> AppInfoExt for O {
}
