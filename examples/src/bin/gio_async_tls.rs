use futures::prelude::*;
use futures::task::{Context, Poll};
use gtk::prelude::*;
use gtk::{gio, glib};

use std::io;
use std::pin::Pin;

use std::error::Error;

async fn run() -> Result<(), Box<dyn Error>> {
    // Connect to https://www.rust-lang.org
    let client = gio::SocketClient::new();
    let connectable = gio::NetworkAddress::new("www.rust-lang.org", 443);

    let connection = client.connect_async_future(&connectable).await?;
    let connection = connection.downcast::<gio::TcpConnection>().unwrap();

    // Get the input/output streams and convert them to the AsyncRead and AsyncWrite adapters
    let ostream = connection
        .get_output_stream()
        .unwrap()
        .dynamic_cast::<gio::PollableOutputStream>()
        .unwrap();
    let write = ostream.into_async_write().unwrap();

    let istream = connection
        .get_input_stream()
        .unwrap()
        .dynamic_cast::<gio::PollableInputStream>()
        .unwrap();
    let read = istream.into_async_read().unwrap();

    // Wrap both in our Connection struct and start the TLS handshake on it
    let connection = Connection { read, write };
    let connector = async_tls::TlsConnector::new();
    let mut connection = connector.connect("www.rust-lang.org", connection).await?;

    // Send the HTTP request
    connection
        .write_all(&b"GET / HTTP/1.1\r\nHost: www.rust-lang.org\r\nConnection: close\r\n\r\n"[..])
        .await?;

    // And then read the response until the end
    let mut buffer = [0u8; 8192];
    loop {
        let len = connection.read(&mut buffer[..]).await?;

        if len == 0 {
            break;
        }

        print!("{}", String::from_utf8_lossy(&buffer[..len]));
    }
    println!();

    connection.close().await?;

    Ok(())
}

// Wrapper type around the AsyncRead/Write adapters that provides both at once
#[derive(Debug)]
struct Connection {
    read: gio::InputStreamAsyncRead<gio::PollableInputStream>,
    write: gio::OutputStreamAsyncWrite<gio::PollableOutputStream>,
}

// Proxy to the internal AsyncRead
impl AsyncRead for Connection {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(&mut Pin::get_mut(self).read).poll_read(cx, buf)
    }
}

// Proxy to the internal AsyncWrite
impl AsyncWrite for Connection {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(&mut Pin::get_mut(self).write).poll_write(cx, buf)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut Pin::get_mut(self).write).poll_close(cx)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut Pin::get_mut(self).write).poll_flush(cx)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the default main context and run our async function on it
    let main_context = glib::MainContext::default();
    main_context.block_on(run())
}
