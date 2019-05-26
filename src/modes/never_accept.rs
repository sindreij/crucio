/// This will create a socket which never accept connections
use std::mem;
use std::net::SocketAddr;

use tokio;
use tokio::net::TcpListener;

// This does not need to return a future since it does not do anything...
pub fn bind(addr: impl Into<SocketAddr>) {
    let addr = addr.into();
    let listener = TcpListener::bind(&addr).unwrap();
    // Will never call drop on listener
    mem::forget(listener);
}
