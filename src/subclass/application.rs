// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib_sys;

use glib::translate::*;

use glib::subclass::prelude::*;

use Application;
use ApplicationClass;

use libc::{c_char, c_void};
use std::convert;
use std::ffi::OsString;
use std::fmt;
use std::ops::Deref;
use std::ptr;

pub struct ArgumentList {
    pub(crate) ptr: *mut *mut *mut c_char,
    items: Vec<OsString>,
}

impl ArgumentList {
    pub(crate) fn new(arguments: *mut *mut *mut c_char) -> Self {
        Self {
            ptr: arguments,
            items: unsafe { FromGlibPtrContainer::from_glib_none(ptr::read(arguments)) },
        }
    }

    pub(crate) fn refresh(&mut self) {
        self.items = unsafe { FromGlibPtrContainer::from_glib_none(ptr::read(self.ptr)) };
    }

    // remove the item at index `idx` and shift the raw array
    pub fn remove(&mut self, idx: usize) {
        unsafe {
            let n_args = glib_sys::g_strv_length(*self.ptr) as usize;
            assert!(n_args == self.items.len());
            assert!(idx < n_args);

            self.items.remove(idx);

            glib_sys::g_free((*self.ptr).add(idx) as *mut c_void);

            for i in idx..n_args - 1 {
                ptr::write((*self.ptr).add(i), *(*self.ptr).add(i + 1))
            }
            ptr::write((*self.ptr).add(n_args - 1), ptr::null_mut());
        }
    }
}

impl Deref for ArgumentList {
    type Target = [OsString];

    fn deref(&self) -> &Self::Target {
        self.items.as_slice()
    }
}

impl fmt::Debug for ArgumentList {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.items.fmt(formatter)
    }
}

impl convert::Into<Vec<OsString>> for ArgumentList {
    fn into(self) -> Vec<OsString> {
        self.items
    }
}

pub trait ApplicationImpl: ApplicationImplExt + 'static {
    fn activate(&self, application: &Application) {
        self.parent_activate(application)
    }

    fn after_emit(&self, application: &Application, platform_data: &glib::Variant) {
        self.parent_after_emit(application, platform_data)
    }

    fn before_emit(&self, application: &Application, platform_data: &glib::Variant) {
        self.parent_before_emit(application, platform_data)
    }

    fn command_line(
        &self,
        application: &Application,
        command_line: &::ApplicationCommandLine,
    ) -> i32 {
        self.parent_command_line(application, command_line)
    }

    fn local_command_line(
        &self,
        application: &Application,
        arguments: &mut ArgumentList,
    ) -> Option<i32> {
        self.parent_local_command_line(application, arguments)
    }

    fn open(&self, application: &Application, files: &[::File], hint: &str) {
        self.parent_open(application, files, hint)
    }

    fn quit_mainloop(&self, application: &Application) {
        self.parent_quit_mainloop(application)
    }

    fn run_mainloop(&self, application: &Application) {
        self.parent_run_mainloop(application)
    }

    fn shutdown(&self, application: &Application) {
        self.parent_shutdown(application)
    }

    fn startup(&self, application: &Application) {
        self.parent_startup(application)
    }
}

pub trait ApplicationImplExt {
    fn parent_activate(&self, application: &Application);
    fn parent_after_emit(&self, application: &Application, platform_data: &glib::Variant);
    fn parent_before_emit(&self, application: &Application, platform_data: &glib::Variant);
    fn parent_command_line(
        &self,
        application: &Application,
        command_line: &::ApplicationCommandLine,
    ) -> i32;
    fn parent_local_command_line(
        &self,
        application: &Application,
        arguments: &mut ArgumentList,
    ) -> Option<i32>;
    fn parent_open(&self, application: &Application, files: &[::File], hint: &str);
    fn parent_quit_mainloop(&self, application: &Application);
    fn parent_run_mainloop(&self, application: &Application);
    fn parent_shutdown(&self, application: &Application);
    fn parent_startup(&self, application: &Application);
}

impl<T: ApplicationImpl + ObjectImpl> ApplicationImplExt for T {
    fn parent_activate(&self, application: &Application) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GApplicationClass;
            let f = (*parent_class)
                .activate
                .expect("No parent class implementation for \"activate\"");
            f(application.to_glib_none().0)
        }
    }

    fn parent_after_emit(&self, application: &Application, platform_data: &glib::Variant) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GApplicationClass;
            let f = (*parent_class)
                .after_emit
                .expect("No parent class implementation for \"after_emit\"");
            f(application.to_glib_none().0, platform_data.to_glib_none().0)
        }
    }

    fn parent_before_emit(&self, application: &Application, platform_data: &glib::Variant) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GApplicationClass;
            let f = (*parent_class)
                .before_emit
                .expect("No parent class implementation for \"before_emit\"");
            f(application.to_glib_none().0, platform_data.to_glib_none().0)
        }
    }

    fn parent_command_line(
        &self,
        application: &Application,
        command_line: &::ApplicationCommandLine,
    ) -> i32 {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GApplicationClass;
            let f = (*parent_class)
                .command_line
                .expect("No parent class implementation for \"command_line\"");
            f(application.to_glib_none().0, command_line.to_glib_none().0)
        }
    }

    fn parent_local_command_line(
        &self,
        application: &Application,
        arguments: &mut ArgumentList,
    ) -> Option<i32> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GApplicationClass;
            let f = (*parent_class)
                .local_command_line
                .expect("No parent class implementation for \"local_command_line\"");

            let mut exit_status = 0;
            let res = f(
                application.to_glib_none().0,
                arguments.ptr,
                &mut exit_status,
            );
            arguments.refresh();

            match res {
                glib_sys::GFALSE => None,
                _ => Some(exit_status),
            }
        }
    }

    fn parent_open(&self, application: &Application, files: &[::File], hint: &str) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GApplicationClass;
            let f = (*parent_class)
                .open
                .expect("No parent class implementation for \"open\"");
            f(
                application.to_glib_none().0,
                files.to_glib_none().0,
                files.len() as i32,
                hint.to_glib_none().0,
            )
        }
    }

    fn parent_quit_mainloop(&self, application: &Application) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GApplicationClass;
            let f = (*parent_class)
                .quit_mainloop
                .expect("No parent class implementation for \"quit_mainloop\"");
            f(application.to_glib_none().0)
        }
    }

    fn parent_run_mainloop(&self, application: &Application) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GApplicationClass;
            let f = (*parent_class)
                .run_mainloop
                .expect("No parent class implementation for \"run_mainloop\"");
            f(application.to_glib_none().0)
        }
    }

    fn parent_shutdown(&self, application: &Application) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GApplicationClass;
            let f = (*parent_class)
                .shutdown
                .expect("No parent class implementation for \"shutdown\"");
            f(application.to_glib_none().0)
        }
    }

    fn parent_startup(&self, application: &Application) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gio_sys::GApplicationClass;
            let f = (*parent_class)
                .startup
                .expect("No parent class implementation for \"startup\"");
            f(application.to_glib_none().0)
        }
    }
}

unsafe impl<T: ObjectSubclass + ApplicationImpl> IsSubclassable<T> for ApplicationClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gio_sys::GApplicationClass);
            klass.activate = Some(application_activate::<T>);
            klass.after_emit = Some(application_after_emit::<T>);
            klass.before_emit = Some(application_before_emit::<T>);
            klass.command_line = Some(application_command_line::<T>);
            klass.local_command_line = Some(application_local_command_line::<T>);
            klass.open = Some(application_open::<T>);
            klass.quit_mainloop = Some(application_quit_mainloop::<T>);
            klass.run_mainloop = Some(application_run_mainloop::<T>);
            klass.shutdown = Some(application_shutdown::<T>);
            klass.startup = Some(application_startup::<T>);
        }
    }
}

unsafe extern "C" fn application_activate<T: ObjectSubclass>(ptr: *mut gio_sys::GApplication)
where
    T: ApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    imp.activate(&wrap)
}

unsafe extern "C" fn application_after_emit<T: ObjectSubclass>(
    ptr: *mut gio_sys::GApplication,
    platform_data: *mut glib_sys::GVariant,
) where
    T: ApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    imp.after_emit(&wrap, &from_glib_borrow(platform_data))
}
unsafe extern "C" fn application_before_emit<T: ObjectSubclass>(
    ptr: *mut gio_sys::GApplication,
    platform_data: *mut glib_sys::GVariant,
) where
    T: ApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    imp.before_emit(&wrap, &from_glib_borrow(platform_data))
}
unsafe extern "C" fn application_command_line<T: ObjectSubclass>(
    ptr: *mut gio_sys::GApplication,
    command_line: *mut gio_sys::GApplicationCommandLine,
) -> i32
where
    T: ApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    imp.command_line(&wrap, &from_glib_borrow(command_line))
}
unsafe extern "C" fn application_local_command_line<T: ObjectSubclass>(
    ptr: *mut gio_sys::GApplication,
    arguments: *mut *mut *mut c_char,
    exit_status: *mut i32,
) -> glib_sys::gboolean
where
    T: ApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    let mut args = ArgumentList::new(arguments);
    let res = imp.local_command_line(&wrap, &mut args);
    args.refresh();

    match res {
        Some(ret) => {
            ptr::write(exit_status, ret);
            glib_sys::GTRUE
        }
        None => glib_sys::GFALSE,
    }
}
unsafe extern "C" fn application_open<T: ObjectSubclass>(
    ptr: *mut gio_sys::GApplication,
    files: *mut *mut gio_sys::GFile,
    num_files: i32,
    hint: *const c_char,
) where
    T: ApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    let files: Vec<::File> = FromGlibContainer::from_glib_none_num(files, num_files as usize);
    imp.open(
        &wrap,
        files.as_slice(),
        &glib::GString::from_glib_borrow(hint),
    )
}
unsafe extern "C" fn application_quit_mainloop<T: ObjectSubclass>(ptr: *mut gio_sys::GApplication)
where
    T: ApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    imp.quit_mainloop(&wrap)
}
unsafe extern "C" fn application_run_mainloop<T: ObjectSubclass>(ptr: *mut gio_sys::GApplication)
where
    T: ApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    imp.run_mainloop(&wrap)
}
unsafe extern "C" fn application_shutdown<T: ObjectSubclass>(ptr: *mut gio_sys::GApplication)
where
    T: ApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    imp.shutdown(&wrap)
}
unsafe extern "C" fn application_startup<T: ObjectSubclass>(ptr: *mut gio_sys::GApplication)
where
    T: ApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    imp.startup(&wrap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use glib;
    use glib::subclass;

    const EXIT_STATUS: i32 = 20;

    struct SimpleApplication;

    impl ObjectSubclass for SimpleApplication {
        const NAME: &'static str = "SimpleApplication";
        type ParentType = Application;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        fn new() -> Self {
            Self
        }
    }

    impl ObjectImpl for SimpleApplication {
        glib_object_impl!();
    }

    impl ApplicationImpl for SimpleApplication {
        fn local_command_line(
            &self,
            _application: &Application,
            arguments: &mut ArgumentList,
        ) -> Option<i32> {
            let mut rm = Vec::new();

            for (i, line) in arguments.iter().enumerate() {
                // TODO: we need https://github.com/rust-lang/rust/issues/49802
                let l = line.clone().into_string().unwrap();
                if l.starts_with("--local-") {
                    rm.push(i)
                }
            }

            rm.reverse();

            for i in rm.iter() {
                arguments.remove(*i);
            }

            None
        }

        fn command_line(
            &self,
            _application: &Application,
            cmd_line: &::ApplicationCommandLine,
        ) -> i32 {
            let arguments = cmd_line.get_arguments();

            for arg in arguments {
                // TODO: we need https://github.com/rust-lang/rust/issues/49802
                let a = arg.into_string().unwrap();
                assert!(!a.starts_with("--local-"))
            }

            return EXIT_STATUS;
        }
    }

    #[test]
    fn test_simple_application() {
        let app = glib::Object::new(
            SimpleApplication::get_type(),
            &[
                ("application-id", &"org.gtk-rs.SimpleApplication"),
                ("flags", &::ApplicationFlags::empty()),
            ],
        )
        .unwrap()
        .downcast::<::Application>()
        .unwrap();

        app.set_inactivity_timeout(10000);

        assert!(app.run(&["--local".to_string()]) == EXIT_STATUS);
    }
}
