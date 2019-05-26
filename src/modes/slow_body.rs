/// Returns the the headers quickly, but then waits 1 seconds between each byte of the response.
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use futures::compat::Future01CompatExt;
use futures::prelude::*;
use futures::stream::unfold;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Response, Server};
use log::{error, info};
use snafu::Snafu;
use tokio::timer::Delay;

#[derive(Debug, Snafu)]
enum Error {}

pub fn bind(data: &[u8], addr: impl Into<SocketAddr>) -> impl Future<Item = (), Error = ()> {
    let addr = addr.into();
    info!("Listening on {}", addr);

    let data = data.to_owned();
    let new_svc = move || {
        let data = data.clone();
        service_fn(move |request| {
            info!("{} {}", request.method(), request.uri());
            response(data.clone()).boxed().compat()
        })
    };

    Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| error!("server error: {}", e))
}

async fn response(data: Vec<u8>) -> Result<Response<Body>, Error> {
    let data: VecDeque<_> = data.into();

    let stream = unfold(
        data,
        async move |mut data| -> Option<(Result<Vec<u8>, Error>, VecDeque<u8>)> {
            Delay::new(Instant::now() + Duration::from_secs(1))
                .compat()
                .await
                .expect("Could not delay");

            let value = data.pop_front()?;

            Some((Ok(vec![value]), data))
        },
    );

    Ok(Response::new(Body::wrap_stream(stream.boxed().compat())))
}
