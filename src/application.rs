use Application;
use File;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::{SignalHandlerId, connect};
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

pub trait ApplicationExtManual {
    fn run(&self, argv: &[String]) -> i32;
    fn connect_open<F: Fn(&Self, &[File], &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Application> + IsA<glib::object::Object>> ApplicationExtManual for O {
    fn run(&self, argv: &[String]) -> i32 {
        let argc = argv.len() as i32;
        unsafe {
            ffi::g_application_run(self.to_glib_none().0, argc, argv.to_glib_none().0)
        }
    }

    fn connect_open<F: Fn(&Self, &[File], &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &[File], &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "open",
                transmute(open_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
unsafe extern "C" fn open_trampoline<P>(this: *mut ffi::GApplication, files: *const *mut ffi::GFile, n_files: libc::c_int, hint: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &&(Fn(&P, &[File], &str) + 'static) = transmute(f);
    let files: Vec<File> = FromGlibContainer::from_glib_none_num(files, n_files as usize);
    f(&Application::from_glib_none(this).downcast_unchecked(), &files, &String::from_glib_none(hint))
}
