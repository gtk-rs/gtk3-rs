// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use glib::subclass::prelude::*;

use glib::{Cast, VariantDict};

use crate::Application;

use libc::{c_char, c_int, c_void};
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
            let n_args = glib::ffi::g_strv_length(*self.ptr) as usize;
            assert!(n_args == self.items.len());
            assert!(idx < n_args);

            self.items.remove(idx);

            glib::ffi::g_free((*self.ptr).add(idx) as *mut c_void);

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

impl From<ArgumentList> for Vec<OsString> {
    fn from(list: ArgumentList) -> Vec<OsString> {
        list.items
    }
}

pub trait ApplicationImpl: ObjectImpl + ApplicationImplExt {
    fn activate(&self, application: &Self::Type) {
        self.parent_activate(application)
    }

    fn after_emit(&self, application: &Self::Type, platform_data: &glib::Variant) {
        self.parent_after_emit(application, platform_data)
    }

    fn before_emit(&self, application: &Self::Type, platform_data: &glib::Variant) {
        self.parent_before_emit(application, platform_data)
    }

    fn command_line(
        &self,
        application: &Self::Type,
        command_line: &crate::ApplicationCommandLine,
    ) -> i32 {
        self.parent_command_line(application, command_line)
    }

    fn local_command_line(
        &self,
        application: &Self::Type,
        arguments: &mut ArgumentList,
    ) -> Option<i32> {
        self.parent_local_command_line(application, arguments)
    }

    fn open(&self, application: &Self::Type, files: &[crate::File], hint: &str) {
        self.parent_open(application, files, hint)
    }

    fn quit_mainloop(&self, application: &Self::Type) {
        self.parent_quit_mainloop(application)
    }

    fn run_mainloop(&self, application: &Self::Type) {
        self.parent_run_mainloop(application)
    }

    fn shutdown(&self, application: &Self::Type) {
        self.parent_shutdown(application)
    }

    fn startup(&self, application: &Self::Type) {
        self.parent_startup(application)
    }

    fn handle_local_options(&self, application: &Self::Type, options: &VariantDict) -> i32 {
        self.parent_handle_local_options(application, options)
    }
}

pub trait ApplicationImplExt: ObjectSubclass {
    fn parent_activate(&self, application: &Self::Type);
    fn parent_after_emit(&self, application: &Self::Type, platform_data: &glib::Variant);
    fn parent_before_emit(&self, application: &Self::Type, platform_data: &glib::Variant);
    fn parent_command_line(
        &self,
        application: &Self::Type,
        command_line: &crate::ApplicationCommandLine,
    ) -> i32;
    fn parent_local_command_line(
        &self,
        application: &Self::Type,
        arguments: &mut ArgumentList,
    ) -> Option<i32>;
    fn parent_open(&self, application: &Self::Type, files: &[crate::File], hint: &str);
    fn parent_quit_mainloop(&self, application: &Self::Type);
    fn parent_run_mainloop(&self, application: &Self::Type);
    fn parent_shutdown(&self, application: &Self::Type);
    fn parent_startup(&self, application: &Self::Type);
    fn parent_handle_local_options(&self, application: &Self::Type, options: &VariantDict) -> i32;
}

impl<T: ApplicationImpl> ApplicationImplExt for T {
    fn parent_activate(&self, application: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            let f = (*parent_class)
                .activate
                .expect("No parent class implementation for \"activate\"");
            f(application
                .unsafe_cast_ref::<Application>()
                .to_glib_none()
                .0)
        }
    }

    fn parent_after_emit(&self, application: &Self::Type, platform_data: &glib::Variant) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            let f = (*parent_class)
                .after_emit
                .expect("No parent class implementation for \"after_emit\"");
            f(
                application
                    .unsafe_cast_ref::<Application>()
                    .to_glib_none()
                    .0,
                platform_data.to_glib_none().0,
            )
        }
    }

    fn parent_before_emit(&self, application: &Self::Type, platform_data: &glib::Variant) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            let f = (*parent_class)
                .before_emit
                .expect("No parent class implementation for \"before_emit\"");
            f(
                application
                    .unsafe_cast_ref::<Application>()
                    .to_glib_none()
                    .0,
                platform_data.to_glib_none().0,
            )
        }
    }

    fn parent_command_line(
        &self,
        application: &Self::Type,
        command_line: &crate::ApplicationCommandLine,
    ) -> i32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            let f = (*parent_class)
                .command_line
                .expect("No parent class implementation for \"command_line\"");
            f(
                application
                    .unsafe_cast_ref::<Application>()
                    .to_glib_none()
                    .0,
                command_line.to_glib_none().0,
            )
        }
    }

    fn parent_local_command_line(
        &self,
        application: &Self::Type,
        arguments: &mut ArgumentList,
    ) -> Option<i32> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            let f = (*parent_class)
                .local_command_line
                .expect("No parent class implementation for \"local_command_line\"");

            let mut exit_status = 0;
            let res = f(
                application
                    .unsafe_cast_ref::<Application>()
                    .to_glib_none()
                    .0,
                arguments.ptr,
                &mut exit_status,
            );
            arguments.refresh();

            match res {
                glib::ffi::GFALSE => None,
                _ => Some(exit_status),
            }
        }
    }

    fn parent_open(&self, application: &Self::Type, files: &[crate::File], hint: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            let f = (*parent_class)
                .open
                .expect("No parent class implementation for \"open\"");
            f(
                application
                    .unsafe_cast_ref::<Application>()
                    .to_glib_none()
                    .0,
                files.to_glib_none().0,
                files.len() as i32,
                hint.to_glib_none().0,
            )
        }
    }

    fn parent_quit_mainloop(&self, application: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            let f = (*parent_class)
                .quit_mainloop
                .expect("No parent class implementation for \"quit_mainloop\"");
            f(application
                .unsafe_cast_ref::<Application>()
                .to_glib_none()
                .0)
        }
    }

    fn parent_run_mainloop(&self, application: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            let f = (*parent_class)
                .run_mainloop
                .expect("No parent class implementation for \"run_mainloop\"");
            f(application
                .unsafe_cast_ref::<Application>()
                .to_glib_none()
                .0)
        }
    }

    fn parent_shutdown(&self, application: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            let f = (*parent_class)
                .shutdown
                .expect("No parent class implementation for \"shutdown\"");
            f(application
                .unsafe_cast_ref::<Application>()
                .to_glib_none()
                .0)
        }
    }

    fn parent_startup(&self, application: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            let f = (*parent_class)
                .startup
                .expect("No parent class implementation for \"startup\"");
            f(application
                .unsafe_cast_ref::<Application>()
                .to_glib_none()
                .0)
        }
    }

    fn parent_handle_local_options(&self, application: &Self::Type, options: &VariantDict) -> i32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GApplicationClass;
            if let Some(f) = (*parent_class).handle_local_options {
                f(
                    application
                        .unsafe_cast_ref::<Application>()
                        .to_glib_none()
                        .0,
                    options.to_glib_none().0,
                )
            } else {
                // Continue default handling
                -1
            }
        }
    }
}

unsafe impl<T: ApplicationImpl> IsSubclassable<T> for Application {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
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
        klass.handle_local_options = Some(application_handle_local_options::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <glib::Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn application_activate<T: ApplicationImpl>(ptr: *mut ffi::GApplication) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Application> = from_glib_borrow(ptr);

    imp.activate(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn application_after_emit<T: ApplicationImpl>(
    ptr: *mut ffi::GApplication,
    platform_data: *mut glib::ffi::GVariant,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Application> = from_glib_borrow(ptr);

    imp.after_emit(wrap.unsafe_cast_ref(), &from_glib_borrow(platform_data))
}
unsafe extern "C" fn application_before_emit<T: ApplicationImpl>(
    ptr: *mut ffi::GApplication,
    platform_data: *mut glib::ffi::GVariant,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Application> = from_glib_borrow(ptr);

    imp.before_emit(wrap.unsafe_cast_ref(), &from_glib_borrow(platform_data))
}
unsafe extern "C" fn application_command_line<T: ApplicationImpl>(
    ptr: *mut ffi::GApplication,
    command_line: *mut ffi::GApplicationCommandLine,
) -> i32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Application> = from_glib_borrow(ptr);

    imp.command_line(wrap.unsafe_cast_ref(), &from_glib_borrow(command_line))
}
unsafe extern "C" fn application_local_command_line<T: ApplicationImpl>(
    ptr: *mut ffi::GApplication,
    arguments: *mut *mut *mut c_char,
    exit_status: *mut i32,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Application> = from_glib_borrow(ptr);

    let mut args = ArgumentList::new(arguments);
    let res = imp.local_command_line(wrap.unsafe_cast_ref(), &mut args);
    args.refresh();

    match res {
        Some(ret) => {
            ptr::write(exit_status, ret);
            glib::ffi::GTRUE
        }
        None => glib::ffi::GFALSE,
    }
}
unsafe extern "C" fn application_open<T: ApplicationImpl>(
    ptr: *mut ffi::GApplication,
    files: *mut *mut ffi::GFile,
    num_files: i32,
    hint: *const c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Application> = from_glib_borrow(ptr);

    let files: Vec<crate::File> = FromGlibContainer::from_glib_none_num(files, num_files as usize);
    imp.open(
        wrap.unsafe_cast_ref(),
        files.as_slice(),
        &glib::GString::from_glib_borrow(hint),
    )
}
unsafe extern "C" fn application_quit_mainloop<T: ApplicationImpl>(ptr: *mut ffi::GApplication) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Application> = from_glib_borrow(ptr);

    imp.quit_mainloop(wrap.unsafe_cast_ref())
}
unsafe extern "C" fn application_run_mainloop<T: ApplicationImpl>(ptr: *mut ffi::GApplication) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Application> = from_glib_borrow(ptr);

    imp.run_mainloop(wrap.unsafe_cast_ref())
}
unsafe extern "C" fn application_shutdown<T: ApplicationImpl>(ptr: *mut ffi::GApplication) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Application> = from_glib_borrow(ptr);

    imp.shutdown(wrap.unsafe_cast_ref())
}
unsafe extern "C" fn application_startup<T: ApplicationImpl>(ptr: *mut ffi::GApplication) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Application> = from_glib_borrow(ptr);

    imp.startup(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn application_handle_local_options<T: ApplicationImpl>(
    ptr: *mut ffi::GApplication,
    options: *mut glib::ffi::GVariantDict,
) -> c_int {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Application> = from_glib_borrow(ptr);

    imp.handle_local_options(wrap.unsafe_cast_ref(), &from_glib_borrow(options))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    const EXIT_STATUS: i32 = 20;

    mod imp {
        use super::*;

        #[derive(Default)]
        pub struct SimpleApplication;

        #[glib::object_subclass]
        impl ObjectSubclass for SimpleApplication {
            const NAME: &'static str = "SimpleApplication";
            type Type = super::SimpleApplication;
            type ParentType = Application;
        }

        impl ObjectImpl for SimpleApplication {}

        impl ApplicationImpl for SimpleApplication {
            fn local_command_line(
                &self,
                _application: &Self::Type,
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
                _application: &Self::Type,
                cmd_line: &crate::ApplicationCommandLine,
            ) -> i32 {
                let arguments = cmd_line.get_arguments();

                for arg in arguments {
                    // TODO: we need https://github.com/rust-lang/rust/issues/49802
                    let a = arg.into_string().unwrap();
                    assert!(!a.starts_with("--local-"))
                }

                EXIT_STATUS
            }
        }
    }

    glib::wrapper! {
        pub struct SimpleApplication(ObjectSubclass<imp::SimpleApplication>)
        @implements crate::Application;
    }

    #[test]
    fn test_simple_application() {
        let app = glib::Object::new::<SimpleApplication>(&[
            ("application-id", &"org.gtk-rs.SimpleApplication"),
            ("flags", &crate::ApplicationFlags::empty()),
        ])
        .unwrap()
        .upcast::<crate::Application>();

        app.set_inactivity_timeout(10000);

        assert!(app.run_with_args(&["--local"]) == EXIT_STATUS);
    }
}
