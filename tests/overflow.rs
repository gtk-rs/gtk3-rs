extern crate gdk_pixbuf;
use gdk_pixbuf::*;

#[test]
#[cfg(target_pointer_width = "64")]
fn put_pixel_doesnt_overflow() {
    // Only test this on 64-bit boxes; otherwise we can't even
    // allocate a pixbuf this big.

    let pixbuf = Pixbuf::new(Colorspace::Rgb, true, 8, 21000, 29700).unwrap();

    // debug build:  thread 'put_pixel_doesnt_overflow' panicked at
    // 'attempt to multiply with overflow', src/pixbuf.rs:274:24
    //
    // release build: thread 'put_pixel_doesnt_overflow' panicked at
    // 'index out of bounds: the len is 2494800000 but the index is
    // 18446744071598664320', src/pixbuf.rs:276:13

    pixbuf.put_pixel(20000, 26000, 255, 255, 255, 255);
}

#[test]
#[cfg(target_pointer_width = "64")]
fn new_from_mut_slice_doesnt_overflow() {
    // Only test this on 64-bit boxes; otherwise we can't even
    // allocate a pixbuf this big.

    // Plus 5 to test that new_from_mut_slice() can ignore trailing data past the last row
    let data = vec![0u8; 21000 * 4 * 29700 + 5];

    // debug build: thread 'new_from_mut_slice_doesnt_overflow'
    // panicked at 'attempt to multiply with overflow',
    // /home/federico/src/gtk-rs/gdk-pixbuf/src/pixbuf.rs:50:36
    //
    // release build: thread 'new_from_mut_slice_doesnt_overflow'
    // panicked at 'assertion failed: data.len() == ((height - 1) *
    // row_stride + last_row_len) as usize', src/pixbuf.rs:50:13

    let _pixbuf =
        Pixbuf::new_from_mut_slice(data, Colorspace::Rgb, true, 8, 21000, 29700, 21000 * 4);
}
