/// This will just echo whatever it gets
use std::net::SocketAddr;

use log::{error, info};
use tokio;
use tokio::net::TcpListener;
use tokio::prelude::*;

pub fn bind(addr: impl Into<SocketAddr>) -> impl Future<Item = (), Error = ()> {
    let addr = addr.into();
    info!("Listening on {}", addr);
    let listener = TcpListener::bind(&addr).unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` combinator method
    listener
        .incoming()
        .for_each(|_socket| {
            info!("Got request");
            Ok(())
        })
        .map_err(|err| {
            // Handle error by printing to STDOUT.
            error!("{:?}", err);
        })
}
