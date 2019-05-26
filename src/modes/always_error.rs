/// Always fails with 500 - internal server error
use std::net::SocketAddr;

use futures::{FutureExt, TryFutureExt};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Response, Server};

use snafu::Snafu;

#[derive(Debug, Snafu)]
enum Error {
    Delaying { source: tokio::timer::Error },
}

pub fn bind(addr: impl Into<SocketAddr>) -> impl Future<Item = (), Error = ()> {
    let addr = addr.into();

    let new_svc = move || service_fn(move |_| response().boxed().compat());

    Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server (error) error: {}", e))
}

async fn response() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(500)
        .body(Body::from("500 - Internal Server Error"))
        .unwrap())
}
