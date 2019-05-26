/// Returns a infinite stream of random text. This will crash browsers because they try to render it.
use std::net::SocketAddr;

use futures::prelude::*;
use futures::stream::unfold;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Response, Server};
use rand::{distributions::Alphanumeric, rngs::StdRng, FromEntropy, Rng};
use snafu::Snafu;

#[derive(Debug, Snafu)]
enum Error {}

pub fn bind(addr: impl Into<SocketAddr>) -> impl Future<Item = (), Error = ()> {
    let addr = addr.into();

    let new_svc = move || service_fn(move |_| response().boxed().compat());

    Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server (slow) error: {}", e))
}

async fn response() -> Result<Response<Body>, Error> {
    let rng = StdRng::from_entropy();

    let stream = unfold(
        rng,
        async move |mut rng| -> Option<(Result<String, Error>, StdRng)> {
            let res = rng.sample_iter(&Alphanumeric).take(128).collect::<String>();

            Some((Ok(res), rng))
        },
    );

    Ok(Response::new(Body::wrap_stream(stream.boxed().compat())))
}
