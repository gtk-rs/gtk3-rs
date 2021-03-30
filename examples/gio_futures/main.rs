use gtk::prelude::*;
use gtk::{gio, glib};

use std::str;

use futures::prelude::*;

// Throughout our chained futures, we convert all errors to strings
// via map_err() and print them at the very end.
//
// Open the file for reading, and if that succeeds read the whole file from
// the resulting input stream.
fn read_and_print_file(
    file: &gio::File,
) -> impl Future<Output = Result<(), String>> + std::marker::Unpin {
    file.read_async_future(glib::PRIORITY_DEFAULT)
        .map_err(|err| format!("Failed to open file: {}", err))
        .and_then(read_and_print_chunks)
}

// Read the input stream in chunks of 64 bytes, always into the same buffer
// without re-allocating it all the time. Continue until the end of the file
// or an error happens.
fn read_and_print_chunks(
    strm: gio::FileInputStream,
) -> impl Future<Output = Result<(), String>> + std::marker::Unpin {
    let buf = vec![0; 64];
    let idx = 0;

    // We use unfold() here, which takes some initialization data and a
    // closure that is returning an item and the next state, or None to
    // finish the stream
    futures::stream::unfold(Some((buf, idx)), move |buf_and_idx| {
        // If None was returned from the last iteration then the last iteration
        // was closing the input stream or an error happened, and now we only
        // have to finish the stream created by unfold().
        //
        // Otherwise we got the buffer to read to and the index of the next line
        // from the previous iteration.
        let (buf, idx) = match buf_and_idx {
            None => {
                return futures::future::Either::Left(futures::future::ready(None));
            }
            Some(buf_and_idx) => buf_and_idx,
        };

        // Read and print the next chunk
        futures::future::Either::Right(read_and_print_next_chunk(&strm, buf, idx).map(move |res| {
            match res {
                // And error happened, return the error from this stream and then finish on the
                // next iteration.
                Err(err) => Some((Err(err), None)),
                // The input stream was closed, return Ok(()) from this stream and then finish on
                // the next iteration.
                Ok(None) => Some((Ok(()), None)),
                // A chunk was successfully read and printed, return Ok(()) from this stream and
                // then continue with the next iteration.
                Ok(Some(buf)) => Some((Ok(()), Some((buf, idx + 1)))),
            }
        }))
    })
    // Convert the stream into a simple future that collects all items and
    // returns Ok(()), or short-circuits on the very first error and returns it
    .try_for_each(|_| futures::future::ok(()))
}

// Read the next chunk into the buffer and print it out, or return an error. If
// the input stream is finished, close the stream.
//
// After reading successfully we return the buffer again so it can be used in the
// next iteration.
fn read_and_print_next_chunk(
    strm: &gio::FileInputStream,
    buf: Vec<u8>,
    idx: usize,
) -> impl Future<Output = Result<Option<Vec<u8>>, String>> + std::marker::Unpin {
    let strm_clone = strm.clone();
    strm.read_async_future(buf, glib::PRIORITY_DEFAULT)
        .map_err(|(_buf, err)| format!("Failed to read from stream: {}", err))
        .and_then(move |(buf, len)| {
            println!("line {}: {:?}", idx, str::from_utf8(&buf[0..len]).unwrap());

            // 0 is only returned when the input stream is finished, in which case
            // we drop the buffer and close the stream asynchronously.
            //
            // Otherwise we simply return the buffer again so it can be read into
            // in the next iteration.
            if len == 0 {
                futures::future::Either::Left(
                    strm_clone
                        .close_async_future(glib::PRIORITY_DEFAULT)
                        .map_err(|err| format!("Failed to close stream: {}", err))
                        .map_ok(|_| None),
                )
            } else {
                futures::future::Either::Right(futures::future::ok(Some(buf)))
            }
        })
}

fn main() {
    let c = glib::MainContext::default();
    let l = glib::MainLoop::new(Some(&c), false);

    c.push_thread_default();

    let file = gio::File::new_for_path("Cargo.toml");

    let l_clone = l.clone();
    c.spawn_local(
        read_and_print_file(&file)
            // Once all is done we quit the main loop and in case of an
            // error first print that error.
            .map(move |res| {
                if let Err(err) = res {
                    eprintln!("Got error: {}", err);
                }

                l_clone.quit();
            }),
    );

    l.run();

    c.pop_thread_default();
}
