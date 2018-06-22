#[cfg(any(feature = "v2_40", feature = "dox"))]
use SubprocessLauncher;
use ffi;
use glib;
use glib::object::IsA;
use GtkRawFd;

pub trait SubprocessLauncherExtManual {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn take_fd(&self, source_fd: GtkRawFd, target_fd: GtkRawFd);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn take_stderr_fd(&self, fd: GtkRawFd);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn take_stdin_fd(&self, fd: GtkRawFd);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn take_stdout_fd(&self, fd: GtkRawFd);
}

#[cfg(any(feature = "v2_40", feature = "dox"))]
impl<O: IsA<SubprocessLauncher> + IsA<glib::object::Object>> SubprocessLauncherExtManual for O {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn take_fd(&self, source_fd: GtkRawFd, target_fd: GtkRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_fd(self.to_glib_none().0, source_fd, target_fd);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn take_stderr_fd(&self, fd: GtkRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stderr_fd(self.to_glib_none().0, fd);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn take_stdin_fd(&self, fd: GtkRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdin_fd(self.to_glib_none().0, fd);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn take_stdout_fd(&self, fd: GtkRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdout_fd(self.to_glib_none().0, fd);
        }
    }
}
