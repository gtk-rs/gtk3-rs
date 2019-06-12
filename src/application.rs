use gio_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use glib::GString;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;
use Application;
use File;

pub trait ApplicationExtManual {
    fn run(&self, argv: &[String]) -> i32;
    fn connect_open<F: Fn(&Self, &[File], &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Application>> ApplicationExtManual for O {
    fn run(&self, argv: &[String]) -> i32 {
        let argc = argv.len() as i32;
        unsafe {
            gio_sys::g_application_run(self.as_ref().to_glib_none().0, argc, argv.to_glib_none().0)
        }
    }

    fn connect_open<F: Fn(&Self, &[File], &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn open_trampoline<P, F: Fn(&P, &[File], &str) + 'static>(
            this: *mut gio_sys::GApplication,
            files: *const *mut gio_sys::GFile,
            n_files: libc::c_int,
            hint: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            let files: Vec<File> = FromGlibContainer::from_glib_none_num(files, n_files as usize);
            f(
                &Application::from_glib_borrow(this).unsafe_cast(),
                &files,
                &GString::from_glib_borrow(hint),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"open\0".as_ptr() as *const _,
                Some(transmute(open_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}
