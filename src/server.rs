extern crate tokio;

use tokio::prelude::*;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use bytes::{BufMut, BytesMut, IntoBuf, Buf};

use std::io::Read;

use protocol::ClientMessage;
use serde_json;

pub struct Server {
    port: usize,
}

struct Commands {
    /// The TCP socket.
    socket: TcpStream,

    /// Buffer used when reading from the socket. Data is not returned from this
    /// buffer until an entire line has been read.
    rd: BytesMut,

    /// Buffer used to stage data before writing it to the socket.
    wr: BytesMut,
}

impl Commands {
    /// Create a new `Commands` codec backed by the socket
    fn new(socket: TcpStream) -> Self {
        Commands {
            socket,
            rd: BytesMut::new(),
            wr: BytesMut::new(),
        }
    }

    /// Buffer a line.
    ///
    /// This writes the line to an internal buffer. Calls to `poll_flush` will
    /// attempt to flush this buffer to the socket.
    fn buffer(&mut self, line: &[u8]) {
        // Ensure the buffer has capacity. Ideally this would not be unbounded,
        // but to keep the example simple, we will not limit this.
        self.wr.reserve(line.len());

        // Push the line onto the end of the write buffer.
        //
        // The `put` function is from the `BufMut` trait.
        self.wr.put(line);
    }

    /// Flush the write buffer to the socket
    fn poll_flush(&mut self) -> Poll<(), io::Error> {
        // As long as there is buffered data to write, try to write it.
        while !self.wr.is_empty() {
            // Try to read some bytes from the socket
            let n = try_ready!(self.socket.poll_write(&self.wr));

            // As long as the wr is not empty, a successful write should
            // never write 0 bytes.
            assert!(n > 0);

            // This discards the first `n` bytes of the buffer.
            let _ = self.wr.split_to(n);
        }

        Ok(Async::Ready(()))
    }

    /// Read data from the socket.
    ///
    /// This only returns `Ready` when the socket has closed.
    fn fill_read_buf(&mut self) -> Poll<(), io::Error> {
        loop {
            // Ensure the read buffer has capacity.
            //
            // This might result in an internal allocation.
            self.rd.reserve(1024);

            // Read data into the buffer.
            let n = try_ready!(self.socket.read_buf(&mut self.rd));

            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }
}

impl Stream for Commands {
    type Item = ClientMessage;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // First, read any new data that might have been received off the socket
        let sock_closed = self.fill_read_buf()?.is_ready();

        // Now, try finding lines
        let pos = self.rd.windows(2).enumerate()
            .find(|&(_, bytes)| bytes == b"\r\n")
            .map(|(i, _)| i);

        if let Some(pos) = pos {
            // Remove the line from the read buffer and set it to `line`.
            let mut line = self.rd.split_to(pos + 2);

            // Drop the trailing \r\n
            line.split_off(pos);

            let message = serde_json::from_reader(line.into_buf().reader());

            // Return the line
            return match message {
                Ok(valid_message) => Ok(Async::Ready(Some(valid_message))),
                Err(invalid_message) => {
                    println!("Invalid message: {:?}", invalid_message);
                    Ok(Async::Ready(None))
                }
            }
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}

fn process(socket: TcpStream) -> impl Future<Item = (), Error = ()> + Send {
    let lines = Commands::new(socket);

    lines
    .map_err(|e| println!("error = {:?}", e))
    .for_each(|head| {
        println!("got message `{:?}`", head);
        Ok(())
    })
}

impl Server {
    pub fn new(port: usize) -> Server {
        Server { port }
    }

    pub fn run(&self) {
        // Bind the server's socket.
        let addr = format!("127.0.0.1:{}", self.port).parse().unwrap();
        let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");

        // Pull out a stream of sockets for incoming connections
        let server = listener.incoming()
            .map_err(|e| println!("error = {:?}", e))
            .for_each(move |socket| {
                tokio::spawn(process(socket))
            });

        // Start the Tokio runtime
        tokio::run(server);
    }
}
