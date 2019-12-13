use glib_sys;
#[cfg(any(feature = "v2_58", feature = "dox"))]
use std;
use std::boxed::Box as Box_;
use std::fs::File;
use std::mem;
#[cfg(not(windows))]
#[cfg(any(feature = "v2_58", feature = "dox"))]
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
#[cfg(any(feature = "v2_58", feature = "dox"))]
use std::os::windows::io::AsRawHandle;
use std::ptr;
use translate::*;
use Error;
use Pid;
use SpawnFlags;

#[cfg(any(feature = "v2_58", feature = "dox"))]
#[cfg(not(windows))]
pub fn spawn_async_with_fds<P: AsRef<std::path::Path>, T: AsRawFd, U: AsRawFd, V: AsRawFd>(
    working_directory: P,
    argv: &[&str],
    envp: &[&str],
    flags: SpawnFlags,
    child_setup: Option<Box_<dyn FnOnce() + 'static>>,
    stdin_fd: T,
    stdout_fd: U,
    stderr_fd: V,
) -> Result<Pid, Error> {
    let child_setup_data: Box_<Option<Box_<dyn FnOnce() + 'static>>> = Box_::new(child_setup);
    unsafe extern "C" fn child_setup_func<P: AsRef<std::path::Path>>(
        user_data: glib_sys::gpointer,
    ) {
        let callback: Box_<Option<Box_<dyn FnOnce() + 'static>>> =
            Box_::from_raw(user_data as *mut _);
        let callback = (*callback).expect("cannot get closure...");
        callback()
    }
    let child_setup = if child_setup_data.is_some() {
        Some(child_setup_func::<P> as _)
    } else {
        None
    };
    let super_callback0: Box_<Option<Box_<dyn FnOnce() + 'static>>> = child_setup_data;
    unsafe {
        let mut child_pid = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = glib_sys::g_spawn_async_with_fds(
            working_directory.as_ref().to_glib_none().0,
            argv.to_glib_none().0,
            envp.to_glib_none().0,
            flags.to_glib(),
            child_setup,
            Box_::into_raw(super_callback0) as *mut _,
            child_pid.as_mut_ptr(),
            stdin_fd.as_raw_fd(),
            stdout_fd.as_raw_fd(),
            stderr_fd.as_raw_fd(),
            &mut error,
        );
        let child_pid = from_glib(child_pid.assume_init());
        if error.is_null() {
            Ok(child_pid)
        } else {
            Err(from_glib_full(error))
        }
    }
}

// #[cfg(any(feature = "v2_58", feature = "dox"))]
// #[cfg(windows)]
// pub fn spawn_async_with_fds<
//     P: AsRef<std::path::Path>,
//     T: AsRawHandle,
//     U: AsRawHandle,
//     V: AsRawHandle,
// >(
//     working_directory: P,
//     argv: &[&str],
//     envp: &[&str],
//     flags: SpawnFlags,
//     child_setup: Option<Box_<dyn FnOnce() + 'static>>,
//     stdin_fd: T,
//     stdout_fd: U,
//     stderr_fd: V,
// ) -> Result<Pid, Error> {
//     let child_setup_data: Box_<Option<Box_<dyn FnOnce() + 'static>>> = Box_::new(child_setup);
//     unsafe extern "C" fn child_setup_func<P: AsRef<std::path::Path>>(
//         user_data: glib_sys::gpointer,
//     ) {
//         let callback: Box_<Option<Box_<dyn FnOnce() + 'static>>> =
//             Box_::from_raw(user_data as *mut _);
//         let callback = (*callback).expect("cannot get closure...");
//         callback()
//     }
//     let child_setup = if child_setup_data.is_some() {
//         Some(child_setup_func::<P> as _)
//     } else {
//         None
//     };
//     let super_callback0: Box_<Option<Box_<dyn FnOnce() + 'static>>> = child_setup_data;
//     unsafe {
//         let mut child_pid = mem::MaybeUninit::uninit();
//         let mut error = ptr::null_mut();
//         let _ = glib_sys::g_spawn_async_with_fds(
//             working_directory.as_ref().to_glib_none().0,
//             argv.to_glib_none().0,
//             envp.to_glib_none().0,
//             flags.to_glib(),
//             child_setup,
//             Box_::into_raw(super_callback0) as *mut _,
//             child_pid.as_mut_ptr(),
//             stdin_fd.as_raw_handle() as usize as _,
//             stdout_fd.as_raw_handle() as usize as _,
//             stderr_fd.as_raw_handle() as usize as _,
//             &mut error,
//         );
//         let child_pid = from_glib(child_pid.assume_init());
//         if error.is_null() {
//             Ok(child_pid)
//         } else {
//             Err(from_glib_full(error))
//         }
//     }
// }

pub fn spawn_async_with_pipes<P: AsRef<std::path::Path>>(
    working_directory: P,
    argv: &[&std::path::Path],
    envp: &[&std::path::Path],
    flags: SpawnFlags,
    child_setup: Option<Box_<dyn FnOnce() + 'static>>,
) -> Result<(Pid, File, File, File), Error> {
    let child_setup_data: Box_<Option<Box_<dyn FnOnce() + 'static>>> = Box_::new(child_setup);
    unsafe extern "C" fn child_setup_func<P: AsRef<std::path::Path>>(
        user_data: glib_sys::gpointer,
    ) {
        let callback: Box_<Option<Box_<dyn FnOnce() + 'static>>> =
            Box_::from_raw(user_data as *mut _);
        let callback = (*callback).expect("cannot get closure...");
        callback()
    }
    let child_setup = if child_setup_data.is_some() {
        Some(child_setup_func::<P> as _)
    } else {
        None
    };
    let super_callback0: Box_<Option<Box_<dyn FnOnce() + 'static>>> = child_setup_data;
    unsafe {
        let mut child_pid = mem::MaybeUninit::uninit();
        let mut standard_input = mem::MaybeUninit::uninit();
        let mut standard_output = mem::MaybeUninit::uninit();
        let mut standard_error = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = glib_sys::g_spawn_async_with_pipes(
            working_directory.as_ref().to_glib_none().0,
            argv.to_glib_none().0,
            envp.to_glib_none().0,
            flags.to_glib(),
            child_setup,
            Box_::into_raw(super_callback0) as *mut _,
            child_pid.as_mut_ptr(),
            standard_input.as_mut_ptr(),
            standard_output.as_mut_ptr(),
            standard_error.as_mut_ptr(),
            &mut error,
        );
        let child_pid = from_glib(child_pid.assume_init());
        let standard_input = standard_input.assume_init();
        let standard_output = standard_output.assume_init();
        let standard_error = standard_error.assume_init();
        if error.is_null() {
            #[cfg(not(windows))]
            {
                use std::os::unix::io::FromRawFd;
                Ok((
                    child_pid,
                    File::from_raw_fd(standard_input),
                    File::from_raw_fd(standard_output),
                    File::from_raw_fd(standard_error),
                ))
            }
            // #[cfg(windows)]
            // {
            //     use std::os::windows::io::{FromRawHandle, RawHandle};
            //     Ok((
            //         child_pid,
            //         File::from_raw_handle(standard_input as usize as RawHandle),
            //         File::from_raw_handle(standard_output as usize as RawHandle),
            //         File::from_raw_handle(standard_error as usize as RawHandle),
            //     ))
            // }
        } else {
            Err(from_glib_full(error))
        }
    }
}
