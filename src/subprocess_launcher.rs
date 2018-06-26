#[cfg(any(feature = "v2_40", feature = "dox"))]
use SubprocessLauncher;
#[cfg(any(all(feature = "v2_40", not(windows)), feature = "dox"))]
use ffi;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use glib;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use glib::object::IsA;
#[cfg(any(all(feature = "v2_40", not(windows)), all(feature = "dox", unix)))]
use std::os::unix::io::IntoRawFd;
#[cfg(all(feature = "dox", not(unix)))]
/// Replacement for "real" [`IntoRawFd`] trait for non-unix targets.
///
/// [`IntoRawFd`]: https://doc.rust-lang.org/std/os/unix/io/trait.IntoRawFd.html
pub trait IntoRawFd {}

pub trait SubprocessLauncherExtManual {
    #[cfg(any(all(feature = "v2_40", not(windows)), feature = "dox"))]
    fn take_fd<F: IntoRawFd, G: IntoRawFd>(&self, source_fd: F, target_fd: G);

    #[cfg(any(all(feature = "v2_40", not(windows)), feature = "dox"))]
    fn take_stderr_fd<F: IntoRawFd>(&self, fd: F);

    #[cfg(any(all(feature = "v2_40", not(windows)), feature = "dox"))]
    fn take_stdin_fd<F: IntoRawFd>(&self, fd: F);

    #[cfg(any(all(feature = "v2_40", not(windows)), feature = "dox"))]
    fn take_stdout_fd<F: IntoRawFd>(&self, fd: F);
}

#[cfg(any(feature = "v2_40", feature = "dox"))]
impl<O: IsA<SubprocessLauncher> + IsA<glib::object::Object>> SubprocessLauncherExtManual for O {
    #[cfg(any(all(feature = "v2_40", not(windows)), feature = "dox"))]
    fn take_fd<F: IntoRawFd, G: IntoRawFd>(&self, source_fd: F, target_fd: G) {
        unsafe {
            ffi::g_subprocess_launcher_take_fd(self.to_glib_none().0,
                                               source_fd.into_raw_fd(),
                                               target_fd.into_raw_fd());
        }
    }

    #[cfg(any(all(feature = "v2_40", not(windows)), feature = "dox"))]
    fn take_stderr_fd<F: IntoRawFd>(&self, fd: F) {
        unsafe {
            ffi::g_subprocess_launcher_take_stderr_fd(self.to_glib_none().0,
                                                      fd.into_raw_fd());
        }
    }

    #[cfg(any(all(feature = "v2_40", not(windows)), feature = "dox"))]
    fn take_stdin_fd<F: IntoRawFd>(&self, fd: F) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdin_fd(self.to_glib_none().0,
                                                     fd.into_raw_fd());
        }
    }

    #[cfg(any(all(feature = "v2_40", not(windows)), feature = "dox"))]
    fn take_stdout_fd<F: IntoRawFd>(&self, fd: F) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdout_fd(self.to_glib_none().0,
                                                      fd.into_raw_fd());
        }
    }
}
