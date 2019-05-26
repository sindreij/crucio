mod always_error;
mod drop;
mod echo;
mod forget_socket;
mod healthy;
mod never;
mod never_accept;
mod never_body;
mod random;
mod random_infinite_tcp;
mod random_sleep;
mod random_sleep_error;
mod random_tcp;
mod random_text;
mod slow;
mod slow_body;
mod slow_error;

use std::net::Ipv4Addr;

use hyper::rt;

pub fn spawn_all(data: &[u8], bind_addr: Ipv4Addr, base_port: u16) {
    rt::spawn(healthy::bind(&data, (bind_addr, base_port)));
    rt::spawn(slow::bind(&data, (bind_addr, base_port + 1)));
    rt::spawn(slow_body::bind(&data, (bind_addr, base_port + 2)));
    rt::spawn(random::bind((bind_addr, base_port + 3)));
    rt::spawn(random_text::bind((bind_addr, base_port + 4)));
    rt::spawn(never::bind((bind_addr, base_port + 5)));
    rt::spawn(never_body::bind((bind_addr, base_port + 6)));
    rt::spawn(echo::bind((bind_addr, base_port + 7)));
    rt::spawn(drop::bind((bind_addr, base_port + 8)));
    rt::spawn(forget_socket::bind((bind_addr, base_port + 9)));
    never_accept::bind((bind_addr, base_port + 10));
    rt::spawn(random_tcp::bind((bind_addr, base_port + 11)));
    rt::spawn(random_infinite_tcp::bind((bind_addr, base_port + 12)));
    rt::spawn(random_sleep::bind(&data, (bind_addr, base_port + 13)));
    rt::spawn(random_sleep_error::bind(&data, (bind_addr, base_port + 14)));
    rt::spawn(always_error::bind((bind_addr, base_port + 15)));
    rt::spawn(slow_error::bind((bind_addr, base_port + 16)));
}
