/// Always fails with 500 - internal server error, but with a delay
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use futures::compat::Future01CompatExt;
use futures::{FutureExt, TryFutureExt};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Response, Server};
use log::{error, info};
use snafu::{ResultExt, Snafu};
use tokio::timer::Delay;

#[derive(Debug, Snafu)]
enum Error {
    Delaying { source: tokio::timer::Error },
}

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
    Delay::new(Instant::now() + Duration::from_secs(9))
        .compat()
        .await
        .context(Delaying)?;

    Ok(Response::builder()
        .status(500)
        .body(Body::from("500 - Internal Server Error"))
        .unwrap())
}
