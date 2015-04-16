// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use std::any::Any;
use translate::ToGlibPtr;

pub trait FFIGObject {
    fn unwrap_gobject(&self) -> *mut ffi::C_GObject;
    fn wrap_object(object: *mut ffi::C_GObject) -> Self;
}

// pub trait Connect<T>: FFIGObject {
//     fn connect<'a>(&self, signal: Box<Signal<'a>>) -> () {
//         use std::mem::transmute;

//         unsafe {
//             let signal_name     = signal.get_signal_name().to_string();
//             let trampoline      = signal.get_trampoline();

//             let user_data_ptr   = transmute(Box::new(signal));

//             signal_name.replace("_", "-").with_c_str(|signal_name| {
//                 ffi::glue_signal_connect(
//                     self.unwrap_gobject(),
//                     signal_name,
//                     Some(trampoline),
//                     user_data_ptr
//                 )
//             });
//         }
//     }
// }

pub trait Signal<'a> {
    fn get_signal_name(&self) -> &str;

    fn get_trampoline(&self) -> extern "C" fn();

    fn fetch_cb(&self) -> *mut FnMut();

    fn get_user_data(&'a self) -> &'a Option<Box<Any>>;
}

pub trait Connect<'a, T: Signal<'a>>: FFIGObject {
    fn connect(&self, signal: Box<T>) -> () {
        use std::mem::transmute;

        let signal = signal as Box<Signal<'a>>;

        unsafe {
            let trampoline      = signal.get_trampoline();
            let signal_name = signal.get_signal_name().replace("_", "-");
            let user_data_ptr   = transmute(Box::new(signal));
            
            ffi::glue_signal_connect(
                self.unwrap_gobject(),
                signal_name.borrow_to_glib().0,
                Some(trampoline),
                user_data_ptr
            );
        }
    }
}