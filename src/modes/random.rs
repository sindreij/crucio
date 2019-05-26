/// Returns a infinite stream of random data.
use std::net::SocketAddr;

use futures::prelude::*;
use futures::stream::unfold;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Response, Server};
use log::{error, info};
use rand::{rngs::StdRng, FromEntropy, RngCore};
use snafu::Snafu;

#[derive(Debug, Snafu)]
enum Error {}

pub fn bind(addr: impl Into<SocketAddr>) -> impl Future<Item = (), Error = ()> {
    let addr = addr.into();
    info!("Listening on {}", addr);

    let new_svc = move || {
        service_fn(move |request| {
            info!("{} {}", request.method(), request.uri());
            response().boxed().compat()
        })
    };

    Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| error!("server error: {}", e))
}

async fn response() -> Result<Response<Body>, Error> {
    let rng = StdRng::from_entropy();

    let stream = unfold(
        rng,
        async move |mut rng| -> Option<(Result<Vec<u8>, Error>, StdRng)> {
            let mut res = vec![0u8; 128];
            rng.fill_bytes(&mut res);

            Some((Ok(res), rng))
        },
    );

    Ok(Response::new(Body::wrap_stream(stream.boxed().compat())))
}
