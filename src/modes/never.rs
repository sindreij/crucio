/// Never returns any response, will just wait forever
use std::net::SocketAddr;

use futures::{future, FutureExt, TryFutureExt};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Response, Server};
use log::{error, info};
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
    // Will nevern return
    let () = future::empty().await;
    Ok(Response::new(Body::from("This should never happen")))
}
