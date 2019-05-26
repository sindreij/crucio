/// This will just echo whatever it gets
/// This may be exactly the same as never.rs
use std::mem;
use std::net::SocketAddr;

use tokio;
use tokio::net::TcpListener;
use tokio::prelude::*;

pub fn bind(addr: impl Into<SocketAddr>) -> impl Future<Item = (), Error = ()> {
    let addr = addr.into();
    let listener = TcpListener::bind(&addr).unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` combinator method
    listener
        .incoming()
        .for_each(|socket| {
            // Make sure drop() on socket is never called, i.e. the connection will never be closed
            mem::forget(socket);
            Ok(())
        })
        .map_err(|err| {
            // Handle error by printing to STDOUT.
            println!("accept error = {:?}", err);
        })
}
