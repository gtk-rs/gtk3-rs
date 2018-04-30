#![feature(proc_macro, proc_macro_non_items, generators, pin)]

extern crate futures;

use futures_macro_async::*;
extern crate futures_macro_async;
#[macro_use]
extern crate futures_macro_await;

extern crate glib;

extern crate gio;
use gio::prelude::*;

use std::str;

// Throughout our chained futures, we convert all errors to strings
// via map_err() return them directly
#[async]
fn read_file(file: gio::File) -> Result<(), String> {
    // Try to open the file
    let (_file, strm) = await!(file.read_async_future(glib::PRIORITY_DEFAULT))
        .map_err(|(_file, err)| format!("Failed to open file: {}", err))?;

    // If opening the file succeeds, we asynchronously loop and
    // read the file in up to 64 byte chunks and re-use the same
    // vec for each read
    let mut buf = vec![0; 64];
    let mut idx = 0;

    loop {
        let (_strm, (b, len)) = await!(strm.read_async_future(buf, glib::PRIORITY_DEFAULT))
            .map_err(|(_strm, (_buf, err))| format!("Failed to read from stream: {}", err))?;

        // Once 0 is returned, we know that we're done with reading, otherwise
        // loop again and read another chunk
        if len == 0 {
            break;
        }

        buf = b;

        println!("line {}: {:?}", idx, str::from_utf8(&buf[0..len]).unwrap());

        idx += 1;
    }

    // asynchronously close the stream
    let _ = await!(strm.close_async_future(glib::PRIORITY_DEFAULT))
        .map_err(|(_stream, err)| format!("Failed to close stream: {}", err))?;

    Ok(())
}

fn main() {
    let c = glib::MainContext::default();
    let l = glib::MainLoop::new(Some(&c), false);

    c.push_thread_default();

    let file = gio::File::new_for_path("Cargo.toml");

    let l_clone = l.clone();
    let future = async_block! {
        match await!(read_file(file)) {
            Ok(()) => (),
            Err(err) => eprintln!("Got error: {}", err),
        }
        l_clone.quit();
        Ok(())
    };

    c.spawn_local(future);

    l.run();

    c.pop_thread_default();
}
