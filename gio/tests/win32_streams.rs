#![cfg(windows)]

use gio::prelude::*;
use gio::{Win32InputStream, Win32OutputStream};

use std::io;
use std::os::windows::io::AsRawHandle;

#[test]
fn create_win32_stdin() {
    let _stream = unsafe { Win32InputStream::new(io::stdin()) };
}

#[test]
fn create_win32_stdout() {
    let _stream = unsafe { Win32OutputStream::new(io::stdout()) };
}
