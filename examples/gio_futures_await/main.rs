use gtk::prelude::*;
use gtk::{gio, glib};

use futures::prelude::*;

use std::str;

// Throughout our chained futures, we convert all errors to strings
// via map_err() return them directly.
async fn read_file(file: gio::File) -> Result<(), String> {
    // Try to open the file.
    let strm = file
        .read_async_future(glib::PRIORITY_DEFAULT)
        .map_err(|err| format!("Failed to open file: {}", err))
        .await?;

    // If opening the file succeeds, we asynchronously loop and
    // read the file in up to 64 byte chunks and re-use the same
    // vec for each read.
    let mut buf = vec![0; 64];
    let mut idx = 0;

    loop {
        let (b, len) = strm
            .read_async_future(buf, glib::PRIORITY_DEFAULT)
            .map_err(|(_buf, err)| format!("Failed to read from stream: {}", err))
            .await?;

        // Once 0 is returned, we know that we're done with reading, otherwise
        // loop again and read another chunk.
        if len == 0 {
            break;
        }

        buf = b;

        println!("line {}: {:?}", idx, str::from_utf8(&buf[0..len]).unwrap());

        idx += 1;
    }

    // Asynchronously close the stream in the end.
    let _ = strm
        .close_async_future(glib::PRIORITY_DEFAULT)
        .map_err(|err| format!("Failed to close stream: {}", err))
        .await?;

    Ok(())
}

fn main() {
    let c = glib::MainContext::default();
    let l = glib::MainLoop::new(Some(&c), false);

    c.push_thread_default();

    let file = gio::File::new_for_path("Cargo.toml");

    let l_clone = l.clone();
    let future = async move {
        match read_file(file).await {
            Ok(()) => (),
            Err(err) => eprintln!("Got error: {}", err),
        }
        l_clone.quit();
    };

    c.spawn_local(future);

    l.run();

    c.pop_thread_default();
}
