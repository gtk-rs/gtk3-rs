#[cfg(any(feature = "v2_40", feature = "dox"))]
use SubprocessLauncher;
#[cfg(any(all(feature = "v2_40", unix), feature = "dox"))]
use ffi;
#[cfg(any(all(feature = "v2_40", unix), feature = "dox"))]
use glib::translate::*;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use glib::object::IsA;
#[cfg(any(all(feature = "v2_40", unix), all(feature = "dox", unix)))]
use std::os::unix::io::IntoRawFd;
#[cfg(all(feature = "dox", not(unix)))]
pub trait IntoRawFd: Sized {
    fn into_raw_fd(self) -> i32 { 0 }
}

pub trait SubprocessLauncherExtManual {
    #[cfg(any(all(feature = "v2_40", unix), feature = "dox"))]
    fn take_fd<F: IntoRawFd, G: IntoRawFd>(&self, source_fd: F, target_fd: G);

    #[cfg(any(all(feature = "v2_40", unix), feature = "dox"))]
    fn take_stderr_fd<F: IntoRawFd>(&self, fd: F);

    #[cfg(any(all(feature = "v2_40", unix), feature = "dox"))]
    fn take_stdin_fd<F: IntoRawFd>(&self, fd: F);

    #[cfg(any(all(feature = "v2_40", unix), feature = "dox"))]
    fn take_stdout_fd<F: IntoRawFd>(&self, fd: F);
}

#[cfg(any(feature = "v2_40", feature = "dox"))]
impl<O: IsA<SubprocessLauncher>> SubprocessLauncherExtManual for O {
    #[cfg(any(all(feature = "v2_40", unix), feature = "dox"))]
    fn take_fd<F: IntoRawFd, G: IntoRawFd>(&self, source_fd: F, target_fd: G) {
        unsafe {
            ffi::g_subprocess_launcher_take_fd(self.as_ref().to_glib_none().0,
                                               source_fd.into_raw_fd(),
                                               target_fd.into_raw_fd());
        }
    }

    #[cfg(any(all(feature = "v2_40", unix), feature = "dox"))]
    fn take_stderr_fd<F: IntoRawFd>(&self, fd: F) {
        unsafe {
            ffi::g_subprocess_launcher_take_stderr_fd(self.as_ref().to_glib_none().0,
                                                      fd.into_raw_fd());
        }
    }

    #[cfg(any(all(feature = "v2_40", unix), feature = "dox"))]
    fn take_stdin_fd<F: IntoRawFd>(&self, fd: F) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdin_fd(self.as_ref().to_glib_none().0,
                                                     fd.into_raw_fd());
        }
    }

    #[cfg(any(all(feature = "v2_40", unix), feature = "dox"))]
    fn take_stdout_fd<F: IntoRawFd>(&self, fd: F) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdout_fd(self.as_ref().to_glib_none().0,
                                                      fd.into_raw_fd());
        }
    }
}
