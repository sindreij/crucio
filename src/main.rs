#![feature(async_await)]

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

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::net::Ipv4Addr;
use std::path::PathBuf;

use hyper::rt;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    filename: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut file = File::open(opt.filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let bind_addr: Ipv4Addr = [127, 0, 0, 1].into();

    rt::run(rt::lazy(move || {
        rt::spawn(healthy::bind(&contents, (bind_addr, 3000)));
        rt::spawn(slow::bind(&contents, (bind_addr, 3001)));
        rt::spawn(slow_body::bind(&contents, (bind_addr, 3002)));
        rt::spawn(random::bind((bind_addr, 3003)));
        rt::spawn(random_text::bind((bind_addr, 3004)));
        rt::spawn(never::bind((bind_addr, 3005)));
        rt::spawn(never_body::bind((bind_addr, 3006)));
        rt::spawn(echo::bind((bind_addr, 3007)));
        rt::spawn(drop::bind((bind_addr, 3008)));
        rt::spawn(forget_socket::bind((bind_addr, 3009)));
        never_accept::start((bind_addr, 3010));
        rt::spawn(random_tcp::bind((bind_addr, 3011)));
        rt::spawn(random_infinite_tcp::bind((bind_addr, 3012)));
        rt::spawn(random_sleep::bind(&contents, (bind_addr, 3013)));
        rt::spawn(random_sleep_error::bind(&contents, (bind_addr, 3014)));
        rt::spawn(always_error::bind((bind_addr, 3015)));
        rt::spawn(slow_error::bind((bind_addr, 3016)));
        Ok(())
    }));

    Ok(())
}
