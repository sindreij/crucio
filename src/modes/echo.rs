/// This will just echo whatever it gets
use std::net::SocketAddr;

use log::{error, info};
use tokio;
use tokio::io;
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
        .for_each(|socket| {
            info!("Got request");
            // split the socket stream into readable and writable parts
            let (reader, writer) = socket.split();
            // copy bytes from the reader into the writer
            let amount = io::copy(reader, writer);

            let msg = amount.then(|result| {
                match result {
                    Ok((_, _, _)) => {}
                    Err(e) => error!("error: {}", e),
                }

                Ok(())
            });

            // spawn the task that handles the client connection socket on to the
            // tokio runtime. This means each client connection will be handled
            // concurrently
            tokio::spawn(msg);
            Ok(())
        })
        .map_err(|err| {
            // Handle error by printing to STDOUT.
            error!("accept error = {:?}", err);
        })
}
