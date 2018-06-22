use std::os;

/// Is equivalent to [`RawFd`]. The equivalent on Windows is [`RawHandle`].
///
/// [`RawFd`]: https://doc.rust-lang.org/std/os/unix/io/type.RawFd.html
/// [`RawHandle`]: https://doc.rust-lang.org/std/os/windows/io/type.RawHandle.html
#[cfg(not(windows))]
pub type GtkRawFd = os::unix::io::RawFd;

/// Is equivalent to [`RawHandle`]. The equivalent on Unix is [`RawFd`].
///
/// [`RawFd`]: https://doc.rust-lang.org/std/os/unix/io/type.RawFd.html
/// [`RawHandle`]: https://doc.rust-lang.org/std/os/windows/io/type.RawHandle.html
#[cfg(windows)]
pub type GtkRawFd = os::windows::io::RawHandle;
