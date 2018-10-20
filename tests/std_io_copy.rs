extern crate glib;
extern crate gio;

use std::io;
use gio::prelude::*;

#[test]
fn std_io_copy_with_gio() {
    let bytes = glib::Bytes::from_owned([1, 2, 3]);
    let stream = gio::MemoryInputStream::new_from_bytes(&bytes);
    let mut out: Vec<u8> = Vec::new();

    let result = io::copy(&mut stream.into_read(), &mut out);

    assert_eq!(result.unwrap(), 3);
    assert_eq!(out, [1, 2, 3]);
}
