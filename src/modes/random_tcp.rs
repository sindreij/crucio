/// This will return a fixed amount of noise, but not valid http.
use std::net::SocketAddr;

use log::{error, info};
use rand::{rngs::StdRng, FromEntropy, RngCore};
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
            let mut rng = StdRng::from_entropy();
            let mut noise = vec![0u8; 128];
            rng.fill_bytes(&mut noise);

            tokio::spawn(io::write_all(socket, noise).map(|_| ()).map_err(|err| {
                // Handle error by printing to STDOUT.
                error!("error writing noise to socket = {:?}", err);
            }));
            Ok(())
        })
        .map_err(|err| {
            // Handle error by printing to STDOUT.
            error!("accept error = {:?}", err);
        })
}
