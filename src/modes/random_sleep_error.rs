/// Returns the response with the body, but wait some time before delivering it. The time is
/// random, (using a LogNormal distrubition). Also fails 50% of the time.
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use futures::compat::Future01CompatExt;
use futures::{FutureExt, TryFutureExt};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use log::{error, info};
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
    info!("Listening on {}", addr);

    let data = data.to_owned();
    let new_svc = move || {
        let data = data.clone();
        service_fn(move |request| response(request, data.clone()).boxed().compat())
    };

    Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| error!("server error: {}", e))
}

async fn response(request: Request<Body>, data: Vec<u8>) -> Result<Response<Body>, Error> {
    let std: f64 = 100.;
    let range = LogNormal::new(std.ln(), 4.0);
    let delay = range.sample(&mut rand::thread_rng()) as u64;

    let is_ok: bool = rand::random();

    info!(
        "{} {}. Sleeping {} ms, returning {}",
        request.method(),
        request.uri(),
        delay,
        if is_ok { 200 } else { 500 }
    );

    Delay::new(Instant::now() + Duration::from_millis(delay))
        .compat()
        .await
        .context(Delaying)?;

    if is_ok {
        Ok(Response::new(Body::from(data)))
    } else {
        Ok(Response::builder()
            .status(500)
            .body(Body::from("500 - Internal Server Error"))
            .unwrap())
    }
}
