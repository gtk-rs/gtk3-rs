// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Application;
use crate::File;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use glib::GString;
use std::boxed::Box as Box_;
use std::mem::transmute;

pub trait ApplicationExtManual {
    #[doc(alias = "g_application_run")]
    fn run(&self) -> i32;
    #[doc(alias = "g_application_run")]
    fn run_with_args<S: AsRef<str>>(&self, args: &[S]) -> i32;
    fn connect_open<F: Fn(&Self, &[File], &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Application>> ApplicationExtManual for O {
    fn run(&self) -> i32 {
        self.run_with_args(&std::env::args().collect::<Vec<_>>())
    }

    fn run_with_args<S: AsRef<str>>(&self, args: &[S]) -> i32 {
        let argv: Vec<&str> = args.iter().map(|a| a.as_ref()).collect();
        let argc = argv.len() as i32;
        unsafe {
            ffi::g_application_run(self.as_ref().to_glib_none().0, argc, argv.to_glib_none().0)
        }
    }

    fn connect_open<F: Fn(&Self, &[File], &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn open_trampoline<P, F: Fn(&P, &[File], &str) + 'static>(
            this: *mut ffi::GApplication,
            files: *const *mut ffi::GFile,
            n_files: libc::c_int,
            hint: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            let files: Vec<File> = FromGlibContainer::from_glib_none_num(files, n_files as usize);
            f(
                &Application::from_glib_borrow(this).unsafe_cast_ref(),
                &files,
                &GString::from_glib_borrow(hint),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"open\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    open_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
