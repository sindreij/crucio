/// This will return a fixed amount of noise, but not valid http.
use std::net::SocketAddr;

use log::{error, info};
use rand::{rngs::StdRng, FromEntropy, RngCore};
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
        .for_each(|mut socket| {
            info!("Got request");
            let mut rng = StdRng::from_entropy();
            let future = stream::poll_fn(move || -> Poll<Option<()>, std::io::Error> {
                let mut noise = [0u8; 32];
                rng.fill_bytes(&mut noise);

                match socket.poll_write(&noise)? {
                    Async::NotReady => Ok(Async::NotReady),
                    Async::Ready(_) => Ok(Async::Ready(Some(()))),
                }
            })
            .map_err(|err| {
                // Handle error by printing to STDOUT.
                error!("write noise error = {:?}", err);
            })
            .for_each(|()| Ok(()));

            tokio::spawn(future);
            Ok(())
        })
        .map_err(|err| {
            // Handle error by printing to STDOUT.
            error!("accept error = {:?}", err);
        })
}
