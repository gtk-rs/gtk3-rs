// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

// This target is build with edition 2018 for testing futures API.
// TODO: merge to the test module of the corresponding source files once the crate
// has been ported to 2018.

use futures::prelude::*;
use gio::prelude::*;
use gio::MemoryInputStream;
use glib::Bytes;
use std::error::Error;

#[test]
fn async_read() {
    async fn run() -> Result<(), Box<dyn Error>> {
        let b = Bytes::from_owned(vec![1, 2, 3]);

        // Adapter is big enough to read everything in one read
        let mut read = MemoryInputStream::from_bytes(&b).into_async_buf_read(8);
        let mut buf = [0u8; 4];
        assert_eq!(read.read(&mut buf).await?, 3);
        assert_eq!(buf, [1, 2, 3, 0]);
        assert_eq!(read.read(&mut buf).await?, 0);

        let mut read = MemoryInputStream::from_bytes(&b).into_async_buf_read(8);
        let mut buf = [0u8; 1];
        assert_eq!(read.read(&mut buf).await?, 1);
        assert_eq!(buf, [1]);
        assert_eq!(read.read(&mut buf).await?, 1);
        assert_eq!(buf, [2]);
        assert_eq!(read.read(&mut buf).await?, 1);
        assert_eq!(buf, [3]);
        assert_eq!(read.read(&mut buf).await?, 0);

        // Adapter is NOT big enough to read everything in one read
        let mut read = MemoryInputStream::from_bytes(&b).into_async_buf_read(2);
        let mut buf = [0u8; 4];
        assert_eq!(read.read(&mut buf).await?, 2);
        assert_eq!(buf, [1, 2, 0, 0]);
        assert_eq!(read.read(&mut buf).await?, 1);
        assert_eq!(buf[0], 3);
        assert_eq!(read.read(&mut buf).await?, 0);

        let mut read = MemoryInputStream::from_bytes(&b).into_async_buf_read(2);
        let mut buf = [0u8; 1];
        assert_eq!(read.read(&mut buf).await?, 1);
        assert_eq!(buf, [1]);
        assert_eq!(read.read(&mut buf).await?, 1);
        assert_eq!(buf, [2]);
        assert_eq!(read.read(&mut buf).await?, 1);
        assert_eq!(buf, [3]);
        assert_eq!(read.read(&mut buf).await?, 0);

        Ok(())
    }

    let main_context = glib::MainContext::new();
    main_context.block_on(run()).unwrap();
}

#[test]
fn async_buf_read() {
    async fn run() -> Result<(), Box<dyn Error>> {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        // Adapter is big enough to read everything in one read
        let mut read = MemoryInputStream::from_bytes(&b).into_async_buf_read(16);
        let mut buf = String::new();
        assert_eq!(read.read_line(&mut buf).await?, 3);
        assert_eq!(buf.as_bytes(), [1, 2, 3]);
        assert_eq!(read.read_line(&mut buf).await?, 0);

        // Adapter is NOT big enough to read everything in one read
        let mut read = MemoryInputStream::from_bytes(&b).into_async_buf_read(2);
        let mut buf = String::new();
        assert_eq!(read.read_line(&mut buf).await?, 3);
        assert_eq!(buf.as_bytes(), [1, 2, 3]);
        assert_eq!(read.read_line(&mut buf).await?, 0);

        Ok(())
    }

    let main_context = glib::MainContext::new();
    main_context.block_on(run()).unwrap();
}
