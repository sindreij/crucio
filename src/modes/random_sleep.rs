/// Returns the response with the body, but wait some time before delivering it. The time is
/// random, (using a LogNormal distrubition)
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use futures::compat::Future01CompatExt;
use futures::{FutureExt, TryFutureExt};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Response, Server};
use rand::distributions::LogNormal;
use rand::prelude::*;
use snafu::{ResultExt, Snafu};
use tokio::timer::Delay;

#[derive(Debug, Snafu)]
enum Error {
    Delaying { source: tokio::timer::Error },
}

pub fn bind(data: &[u8], addr: impl Into<SocketAddr>) -> impl Future<Item = (), Error = ()> {
    let addr = addr.into();

    let data = data.to_owned();
    let new_svc = move || {
        let data = data.clone();
        service_fn(move |_| response(data.clone()).boxed().compat())
    };

    Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server (slow) error: {}", e))
}

async fn response(data: Vec<u8>) -> Result<Response<Body>, Error> {
    let std: f64 = 100.;
    let range = LogNormal::new(std.ln(), 4.0);
    let delay = range.sample(&mut rand::thread_rng());

    Delay::new(Instant::now() + Duration::from_millis(delay as u64))
        .compat()
        .await
        .context(Delaying)?;

    Ok(Response::new(Body::from(data)))
}
