extern crate futures_core;
extern crate futures_util;

extern crate glib;

extern crate gio;
use gio::prelude::*;

use std::str;

use futures_util::FutureExt;
use futures_util::future;

fn main() {
    let c = glib::MainContext::default();
    let l = glib::MainLoop::new(Some(&c), false);

    c.push_thread_default();

    let file = gio::File::new_for_path("Cargo.toml");

    // Throughout our chained futures, we convert all errors to strings
    // via map_err() and print them at the very end
    let l_clone = l.clone();
    c.spawn_local(
        // Try to open the file
        file.read_async_future(glib::PRIORITY_DEFAULT)
            .map_err(|(_file, err)| {
                format!("Failed to open file: {}", err)
            })
            .and_then(|(_file, strm)| {
                // If opening the file succeeds, we asynchronously loop and
                // read the file in up to 64 byte chunks and re-use the same
                // vec for each read
                let buf = vec![0; 64];
                let idx = 0;
                future::loop_fn((strm, buf, idx), |(strm, buf, idx)| {
                    strm.read_async_future(buf, glib::PRIORITY_DEFAULT)
                        .map_err(|(_strm, (_b, err))| {
                            format!("Failed to read from stream: {}", err)
                        })
                        .and_then(move |(_obj, (buf, len))| {
                            println!("line {}: {:?}", idx, str::from_utf8(&buf[0..len]).unwrap());
                            // Once 0 is returned, we know that we're done with reading
                            // and asynchronously close the stream, otherwise we loop again
                            // with the same stream/buffer
                            if len == 0 {
                                let close_future = strm.close_async_future(glib::PRIORITY_DEFAULT)
                                    .map_err(|(_stream, err)| {
                                        format!("Failed to close stream: {}", err)
                                    });
                                Ok(future::Loop::Break(close_future))
                            } else {
                                Ok(future::Loop::Continue((strm, buf, idx + 1)))
                            }
                        })
                })
            })
            // Once all is done, i.e. the stream was closed above or an error happened, we quit the
            // main loop
            .then(move |res| {
                if let Err(err) = res {
                    eprintln!("Got error: {}", err);
                }

                l_clone.quit();
                Ok(())
            }),
    );

    l.run();

    c.pop_thread_default();
}
